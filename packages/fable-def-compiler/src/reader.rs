//! Lowering: convert parsed text Definitions into typed binary def structs.
//!
//! Three composable primitives:
//! - [`Evaluator`] — evaluates a single [`Expr`] to a value using a [`SymbolTable`]
//! - [`Args`] — reads positional values from a constructor or method-call argument list
//! - [`DefReader`] — scans [`Statement`]s by name, consuming fields and producing values or sub-readers
//!
//! These replace the old `FieldReader` (which was never fully defined). The design
//! follows the spec laid out for text→binary compilation: depth-based path matching,
//! consumed-entry tracking, and recursive composition for indexed sub-structs and
//! tagged blocks.

use fable_data::def::text::{Call, Expr, PathSegment, Statement, SymbolTable};

/// If `stmt` is a leaf field whose path is exactly `name` at `depth` (i.e. ends there), return its
/// value expression. Used by the by-name leaf accessors so the path-matching lives in one place.
fn leaf_field<'a>(stmt: &'a Statement, depth: usize, name: &str) -> Option<&'a Expr> {
    let Statement::Field(field) = stmt else {
        return None;
    };
    let segments = &field.path.segments;
    match segments.get(depth) {
        Some(PathSegment::Field(n)) if n == name && segments.len() == depth + 1 => Some(&field.expr),
        _ => None,
    }
}

// ── Evaluator ────────────────────────────────────────────────────────────────

/// Evaluates parsed [`Expr`] values against a [`SymbolTable`].
///
/// This is the single source of truth for expression evaluation. The old
/// `Expr::eval_*` methods from `def_text.rs` have been removed — all evaluation
/// lives here.
#[derive(Clone, Copy)]
pub struct Evaluator<'s> {
    symbols: &'s SymbolTable,
}

#[derive(Debug)]
pub enum EvalError {
    UnknownSymbol(String),
    OutOfRange(i64),
    Overflow,
    UnexpectedExpression,
    ExpectedConstructor { found: &'static str },
    WrongConstructor { expected: String, found: String },
}

impl<'s> Evaluator<'s> {
    pub fn new(symbols: &'s SymbolTable) -> Self {
        Self { symbols }
    }

    pub fn i32(&self, expr: &Expr) -> Result<i32, EvalError> {
        use EvalError as E;
        match expr {
            Expr::Integer(n) => i32::try_from(*n).map_err(|_| E::OutOfRange(*n)),
            Expr::Symbol(name) => {
                let n = self
                    .symbols
                    .lookup(name)
                    .ok_or_else(|| E::UnknownSymbol(name.clone()))?;
                i32::try_from(n).map_err(|_| E::OutOfRange(n))
            }
            Expr::BitOr(parts) => parts
                .iter()
                .try_fold(0i32, |acc, p| Ok(acc | self.i32(p)?)),
            Expr::Add(parts) => parts.iter().try_fold(0i32, |acc, p| {
                acc.checked_add(self.i32(p)?).ok_or(E::Overflow)
            }),
            _ => Err(E::UnexpectedExpression),
        }
    }

    pub fn u32(&self, expr: &Expr) -> Result<u32, EvalError> {
        use EvalError as E;
        match expr {
            Expr::Integer(n) => u32::try_from(*n).map_err(|_| E::OutOfRange(*n)),
            Expr::Symbol(name) => {
                let n = self
                    .symbols
                    .lookup(name)
                    .ok_or_else(|| E::UnknownSymbol(name.clone()))?;
                u32::try_from(n).map_err(|_| E::OutOfRange(n))
            }
            Expr::BitOr(parts) => parts
                .iter()
                .try_fold(0u32, |acc, p| Ok(acc | self.u32(p)?)),
            Expr::Add(parts) => parts.iter().try_fold(0u32, |acc, p| {
                acc.checked_add(self.u32(p)?).ok_or(E::Overflow)
            }),
            _ => Err(E::UnexpectedExpression),
        }
    }

    pub fn f32(&self, expr: &Expr) -> Result<f32, EvalError> {
        use EvalError as E;
        match expr {
            Expr::Float(n) => Ok(*n),
            Expr::Integer(n) => Ok(*n as f32),
            Expr::Symbol(name) => self
                .symbols
                .lookup(name)
                .map(|v| v as f32)
                .ok_or_else(|| E::UnknownSymbol(name.clone())),
            Expr::Add(parts) => parts
                .iter()
                .try_fold(0f32, |acc, p| Ok(acc + self.f32(p)?)),
            _ => Err(E::UnexpectedExpression),
        }
    }

    pub fn bool(&self, expr: &Expr) -> Result<bool, EvalError> {
        match expr {
            Expr::Bool(b) => Ok(*b),
            _ => Err(EvalError::UnexpectedExpression),
        }
    }

    pub fn string<'e>(&self, expr: &'e Expr) -> Result<&'e str, EvalError> {
        match expr {
            Expr::String(s) => Ok(s),
            _ => Err(EvalError::UnexpectedExpression),
        }
    }

    /// Evaluate an expression as a `usize` — used for index expressions like `[0]`.
    pub fn usize(&self, expr: &Expr) -> Result<usize, EvalError> {
        let n = self.i32(expr)?;
        usize::try_from(n).map_err(|_| EvalError::OutOfRange(n as i64))
    }

    /// Validate that `expr` is a `Constructor` with the expected name and return
    /// a reference to its [`Call`] (name + argument list).
    pub fn call<'e>(&self, expr: &'e Expr, name: &str) -> Result<&'e Call, EvalError> {
        match expr {
            Expr::Constructor(call) if call.name == name => Ok(call),
            Expr::Constructor(call) => {
                let found = call.name.clone();
                Err(EvalError::WrongConstructor {
                    expected: name.to_string(),
                    found,
                })
            }
            _ => Err(EvalError::ExpectedConstructor {
                found: "not a constructor",
            }),
        }
    }
}

// ── Args ─────────────────────────────────────────────────────────────────────

/// Positional reader over a constructor or method-call argument list.
///
/// Arguments are addressed by zero-based index. `Args` is immutable and shared
/// (positional reads don't consume), so all methods take `&self`.
#[derive(Clone)]
pub struct Args<'e, 's> {
    args: &'e [Expr],
    eval: Evaluator<'s>,
}

impl<'e, 's> Args<'e, 's> {
    pub fn new(args: &'e [Expr], eval: Evaluator<'s>) -> Self {
        Self { args, eval }
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    fn get(&self, idx: usize) -> Result<&'e Expr, DefReaderError> {
        self.args
            .get(idx)
            .ok_or(DefReaderError::MissingArg(idx))
    }

    pub fn i32(&self, idx: usize) -> Result<i32, DefReaderError> {
        self.eval.i32(self.get(idx)?).map_err(DefReaderError::Eval)
    }

    pub fn u32(&self, idx: usize) -> Result<u32, DefReaderError> {
        self.eval.u32(self.get(idx)?).map_err(DefReaderError::Eval)
    }

    pub fn f32(&self, idx: usize) -> Result<f32, DefReaderError> {
        self.eval.f32(self.get(idx)?).map_err(DefReaderError::Eval)
    }

    pub fn bool(&self, idx: usize) -> Result<bool, DefReaderError> {
        self.eval.bool(self.get(idx)?).map_err(DefReaderError::Eval)
    }

    pub fn string(&self, idx: usize) -> Result<String, DefReaderError> {
        self.eval
            .string(self.get(idx)?)
            .map(|s| s.to_string())
            .map_err(DefReaderError::Eval)
    }

    pub fn opt(&self, idx: usize) -> Option<&'e Expr> {
        self.args.get(idx)
    }

    pub fn ctor(
        &self,
        idx: usize,
        name: &'static str,
    ) -> Result<Args<'e, 's>, DefReaderError> {
        let call = self.eval.call(self.get(idx)?, name).map_err(DefReaderError::Eval)?;
        Ok(Args::new(&call.arguments, self.eval))
    }
}

// ── DefReaderError ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum BadIndex {
    NotAnInteger(Expr),
    OutOfRange(usize),
    Gap { expected: usize, found: usize },
}

#[derive(Debug)]
pub enum DefReaderError {
    MissingField(&'static str),
    DuplicateField(String),
    UnexpectedStatement(Statement),
    UnexpectedPath,
    BadIndex(BadIndex),
    MissingArg(usize),
    Eval(EvalError),
    Semantic(&'static str),
}

impl From<EvalError> for DefReaderError {
    fn from(e: EvalError) -> Self {
        DefReaderError::Eval(e)
    }
}

// ── DefReader ────────────────────────────────────────────────────────────────

struct Entry<'a> {
    stmt: &'a Statement,
    consumed: bool,
}

pub struct DefReader<'a, 's> {
    entries: Vec<Entry<'a>>,
    depth: usize,
    eval: Evaluator<'s>,
}

impl<'a, 's> DefReader<'a, 's> {
    pub fn new(body: &'a [Statement], symbols: &'s SymbolTable) -> Self {
        Self {
            entries: body.iter().map(|s| Entry { stmt: s, consumed: false }).collect(),
            depth: 0,
            eval: Evaluator::new(symbols),
        }
    }

    fn new_with_depth(
        entries: Vec<Entry<'a>>,
        depth: usize,
        eval: Evaluator<'s>,
    ) -> Self {
        Self { entries, depth, eval }
    }

    // ── leaf accessors ──────────────────────────────────────────────────────

    fn find_leaf(&mut self, name: &'static str) -> Result<&'a Expr, DefReaderError> {
        let mut found: Option<(usize, &'a Expr)> = None;
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.consumed {
                continue;
            }
            if let Some(expr) = leaf_field(entry.stmt, self.depth, name) {
                if found.is_some() {
                    return Err(DefReaderError::DuplicateField(name.to_string()));
                }
                found = Some((i, expr));
            }
        }
        let (idx, expr) = found.ok_or(DefReaderError::MissingField(name))?;
        self.entries[idx].consumed = true;
        Ok(expr)
    }

    pub fn i32(&mut self, name: &'static str) -> Result<i32, DefReaderError> {
        let expr = self.find_leaf(name)?;
        self.eval.i32(expr).map_err(DefReaderError::Eval)
    }

    pub fn u32(&mut self, name: &'static str) -> Result<u32, DefReaderError> {
        let expr = self.find_leaf(name)?;
        self.eval.u32(expr).map_err(DefReaderError::Eval)
    }

    pub fn f32(&mut self, name: &'static str) -> Result<f32, DefReaderError> {
        let expr = self.find_leaf(name)?;
        self.eval.f32(expr).map_err(DefReaderError::Eval)
    }

    pub fn bool(&mut self, name: &'static str) -> Result<bool, DefReaderError> {
        let expr = self.find_leaf(name)?;
        self.eval.bool(expr).map_err(DefReaderError::Eval)
    }

    pub fn string(&mut self, name: &'static str) -> Result<String, DefReaderError> {
        let expr = self.find_leaf(name)?;
        self.eval.string(expr).map(|s| s.to_string()).map_err(DefReaderError::Eval)
    }

    // ── optional accessors ──────────────────────────────────────────────────

    fn find_opt_leaf(&mut self, name: &'static str) -> Option<&'a Expr> {
        for entry in self.entries.iter_mut() {
            if entry.consumed {
                continue;
            }
            if let Some(expr) = leaf_field(entry.stmt, self.depth, name) {
                entry.consumed = true;
                return Some(expr);
            }
        }
        None
    }

    pub fn opt_i32(&mut self, name: &'static str) -> Result<Option<i32>, DefReaderError> {
        self.find_opt_leaf(name)
            .map(|e| self.eval.i32(e).map_err(DefReaderError::Eval))
            .transpose()
    }

    pub fn opt_u32(&mut self, name: &'static str) -> Result<Option<u32>, DefReaderError> {
        self.find_opt_leaf(name)
            .map(|e| self.eval.u32(e).map_err(DefReaderError::Eval))
            .transpose()
    }

    pub fn opt_f32(&mut self, name: &'static str) -> Result<Option<f32>, DefReaderError> {
        self.find_opt_leaf(name)
            .map(|e| self.eval.f32(e).map_err(DefReaderError::Eval))
            .transpose()
    }

    pub fn opt_bool(&mut self, name: &'static str) -> Result<Option<bool>, DefReaderError> {
        self.find_opt_leaf(name)
            .map(|e| self.eval.bool(e).map_err(DefReaderError::Eval))
            .transpose()
    }

    pub fn opt_string(&mut self, name: &'static str) -> Result<Option<String>, DefReaderError> {
        self.find_opt_leaf(name)
            .map(|e| self.eval.string(e).map(|s| s.to_string()).map_err(DefReaderError::Eval))
            .transpose()
    }

    // ── constructor ─────────────────────────────────────────────────────────

    pub fn ctor(
        &mut self,
        name: &'static str,
        ctor_name: &'static str,
    ) -> Result<Args<'a, 's>, DefReaderError> {
        let expr = self.find_leaf(name)?;
        let call = self.eval.call(expr, ctor_name).map_err(DefReaderError::Eval)?;
        Ok(Args::new(&call.arguments, self.eval))
    }

    // ── calls ───────────────────────────────────────────────────────────────

    pub fn calls(&mut self, object: &str, method: &str) -> Vec<Args<'a, 's>> {
        let mut results = Vec::new();
        for entry in self.entries.iter_mut() {
            if entry.consumed {
                continue;
            }
            if let Statement::MethodCall(mc) = entry.stmt {
                let obj_name = mc.object.segments.first();
                let matches_obj = matches!(obj_name, Some(PathSegment::Field(n)) if n == object);
                if matches_obj && mc.object.segments.len() == 1 && mc.call.name == method {
                    entry.consumed = true;
                    results.push(Args::new(&mc.call.arguments, self.eval));
                }
            }
        }
        results
    }

    // ── block ───────────────────────────────────────────────────────────────

    pub fn block(&mut self, tag: &str) -> Option<DefReader<'a, 's>> {
        for entry in self.entries.iter_mut() {
            if entry.consumed {
                continue;
            }
            if let Statement::TaggedBlock(tb) = entry.stmt {
                if tb.tag == tag {
                    entry.consumed = true;
                    return Some(DefReader::new(&tb.body, self.eval.symbols));
                }
            }
        }
        None
    }

    // ── indexed ─────────────────────────────────────────────────────────────

    pub fn indexed(
        &mut self,
        name: &str,
    ) -> Result<Vec<DefReader<'a, 's>>, DefReaderError> {
        use std::collections::BTreeMap;

        let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

        for (i, entry) in self.entries.iter().enumerate() {
            if entry.consumed {
                continue;
            }
            if let Statement::Field(field) = entry.stmt {
                let segs = &field.path.segments;
                if segs.len() == self.depth + 2
                    && matches!(segs.get(self.depth), Some(PathSegment::Field(n)) if n == name)
                {
                    if let Some(PathSegment::Index(idx_expr)) = segs.get(self.depth + 1) {
                        let idx = self.eval.usize(idx_expr).map_err(DefReaderError::Eval)?;
                        groups.entry(idx).or_default().push(i);
                    }
                }
            }
        }

        let mut expected = 0usize;
        for (&idx, _) in &groups {
            if idx != expected {
                return Err(DefReaderError::BadIndex(BadIndex::Gap {
                    expected,
                    found: idx,
                }));
            }
            expected = idx + 1;
        }

        let mut readers: Vec<DefReader<'a, 's>> = Vec::with_capacity(groups.len());
        for (_idx, entry_indices) in groups {
            for &i in &entry_indices {
                self.entries[i].consumed = true;
            }
            let entries: Vec<Entry<'a>> = entry_indices
                .iter()
                .map(|&i| Entry {
                    stmt: self.entries[i].stmt,
                    consumed: false,
                })
                .collect();
            readers.push(DefReader::new_with_depth(entries, self.depth + 2, self.eval));
        }

        Ok(readers)
    }

    // ── any (nameless values) ───────────────────────────────────────────────

    pub fn any_expr(&mut self) -> Result<&'a Expr, DefReaderError> {
        for (i, entry) in self.entries.iter_mut().enumerate() {
            if entry.consumed {
                continue;
            }
            if let Statement::Field(field) = entry.stmt {
                let segs = &field.path.segments;
                if segs.len() == self.depth {
                    self.entries[i].consumed = true;
                    return Ok(&field.expr);
                }
            }
        }
        Err(DefReaderError::MissingField("(any)"))
    }

    pub fn any_i32(&mut self) -> Result<i32, DefReaderError> {
        let expr = self.any_expr()?;
        self.eval.i32(expr).map_err(DefReaderError::Eval)
    }

    pub fn any_u32(&mut self) -> Result<u32, DefReaderError> {
        let expr = self.any_expr()?;
        self.eval.u32(expr).map_err(DefReaderError::Eval)
    }

    pub fn any_f32(&mut self) -> Result<f32, DefReaderError> {
        let expr = self.any_expr()?;
        self.eval.f32(expr).map_err(DefReaderError::Eval)
    }

    pub fn any_string(&mut self) -> Result<String, DefReaderError> {
        let expr = self.any_expr()?;
        self.eval.string(expr).map(|s| s.to_string()).map_err(DefReaderError::Eval)
    }

    // ── finish ──────────────────────────────────────────────────────────────

    pub fn finish(self) -> Result<(), DefReaderError> {
        match self.entries.iter().find(|e| !e.consumed) {
            Some(entry) => Err(DefReaderError::UnexpectedStatement(entry.stmt.clone())),
            None => Ok(()),
        }
    }
}


#[cfg(test)]
mod eval_tests {
    use super::Evaluator;
    use fable_data::def::text::{DefParser, Expr, SymbolTable};

    fn syms() -> Evaluator<'static> {
        let table = SymbolTable::new();
        Evaluator::new(Box::leak(Box::new(table)))
    }

    fn parse_expr(s: &str) -> Expr {
        DefParser::new(s).parse_expr().unwrap()
    }

    #[test]
    fn eval_i32() {
        assert_eq!(syms().i32(&parse_expr("42")).unwrap(), 42);
        assert_eq!(syms().i32(&parse_expr("-42")).unwrap(), -42);
    }

    #[test]
    fn eval_u32_negative() {
        let e = parse_expr("-1");
        assert!(syms().u32(&e).is_err());
    }

    #[test]
    fn eval_f32() {
        assert_eq!(syms().f32(&parse_expr("64")).unwrap(), 64.0);
        assert_eq!(syms().f32(&parse_expr("3.25")).unwrap(), 3.25);
    }

    #[test]
    fn eval_bit_or() {
        let expr = parse_expr("1 | 2 | 4");
        assert_eq!(syms().u32(&expr).unwrap(), 7);
    }

    #[test]
    fn eval_bit_or_on_float_is_error() {
        assert!(syms().f32(&parse_expr("1 | 2")).is_err());
    }
}
