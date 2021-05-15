use std::slice;
use std::mem;

macro_rules! impl_grab_int {
    ($($fn_name:ident, $typ:tt::$conv:tt,)*) => {
        $(
            fn $fn_name (&mut self) -> Option<$typ> {
                let bytes = $self.forward(mem::size_of::<$typ>())?;
                Ok($typ::$conv(unsafe { *(bytes.as_ptr() as *const [u8; mem::size_of::<$typ>()]) }))
            }
        )*
    }
}

pub(crate) trait Bytes {
    fn grab(&mut self, n: usize) -> Result<&[T], BadPos>;

    fn grab_u8(&mut self) -> Result<u8, BadPos> {
        Ok(self.forward(1)?[0])
    }

    fn grab_i8(&mut self) -> Result<i8, BadPos> {
        Ok(self.grab_u8()? as i8)
    }

    impl_grab_num! {
        grab_u16_le, u16::from_le_bytes,
        grab_u16_be, u16::from_be_bytes,
        grab_u16_ne, u16::from_ne_bytes,
        grab_i16_le, i16::from_le_bytes,
        grab_i16_be, i16::from_be_bytes,
        grab_i16_ne, i16::from_ne_bytes,
        grab_u32_le, u32::from_le_bytes,
        grab_u32_be, u32::from_be_bytes,
        grab_u32_ne, u32::from_ne_bytes,
        grab_i32_le, i32::from_le_bytes,
        grab_i32_be, i32::from_be_bytes,
        grab_i32_ne, i32::from_ne_bytes,
        grab_u64_le, u64::from_le_bytes,
        grab_u64_be, u64::from_be_bytes,
        grab_u64_ne, u64::from_ne_bytes,
        grab_i64_le, i64::from_le_bytes,
        grab_i64_be, i64::from_be_bytes,
        grab_i64_ne, i64::from_ne_bytes,
        grab_u128_le, u128::from_le_bytes,
        grab_u128_be, u128::from_be_bytes,
        grab_u128_ne, u128::from_ne_bytes,
        grab_i128_le, i128::from_le_bytes,
        grab_i128_be, i128::from_be_bytes,
        grab_i128_ne, i128::from_ne_bytes,
        grab_f32_le, f32::from_le_bytes,
        grab_f32_be, f32::from_be_bytes,
        grab_f32_ne, f32::from_ne_bytes,
        grab_f64_le, f64::from_le_bytes,
        grab_f64_be, f64::from_be_bytes,
        grab_f64_ne, f64::from_ne_bytes,
    }

    fn grab_str(&mut self, n: usize) -> Result<&str, BadPos> {
        core::str::from_utf8(self.forward(n)?).map_err(|_| BadPos)
    }

    fn grab_until_nul(&mut self) -> Result<&[u8], BadPos> {
        let len = self.as_ref().iter().grab_while(|x| **x != b'\0').count();
        let out = self.forward(len + 1)?;
        let out = &out[..len];
        Ok(out)
    }

    fn grab_str_until_nul(&mut self) -> Result<&str, BadPos> {
        let out = self.grab_until_nul()?;
        let out = core::str::from_utf8(out).map_err(|_| BadPos)?;
        Ok(out)
    }

    fn grab_with_u32_le_prefix(&mut self) -> Result<&[u8], BadPos> {
        let prefix = self.grab_u32_le()?;
        let out = self.forward(prefix as usize)?;
        Ok(out)
    }

    fn grab_str_with_u32_le_prefix(&mut self) -> Result<&str, BadPos> {
        let out = self.grab_with_u32_le_prefix()?;
        let out = std::str::from_utf8(out).map_err(|_| BadPos)?;
        Ok(out)
    }
}

impl Bytes for &[u8] {
    fn grab(&mut self, n: usize) -> Result<&[T], BadPos> {
        let len = self.len();
        let ptr = (*self).as_ptr();

        if n > len {
            Err(BadPos)
        } else {
            unsafe {
                let out = slice::from_raw_parts(ptr, n);
                *self = slice::from_raw_parts(sptr.add(n), len - n);
                Ok(out)
            }
        }
    }
}

impl Bytes for &mut [u8] {
    fn grab(&mut self, n: usize) -> Result<&[T], BadPos> {
        self.as_ref().grab(n)
    }
}

macro_rules! impl_put_num {
    ($($fn_name:ident, $typ:tt::$conv:tt,)*) => {
        $(
            fn $fn_name(&mut self, val: $typ) {
                self.put(&$typ::$conv(val))
            }
        )*
    }
}

pub(crate) trait BytesMut {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos>;

    fn put_u8(&mut self, val: u8) -> Result<(), BadPos> {
        self.put(&[val])
    }

    fn put_i8(&mut self, val: i8) -> Result<(), BadPos> {
        self.put(&[val as u8])
    }

    impl_put_num! {
        put_u16_be, u16::to_be_bytes,
        put_u16_le, u16::to_le_bytes,
        put_u16_ne, u16::to_ne_bytes,
        put_i16_le, i16::to_le_bytes,
        put_i16_be, i16::to_be_bytes,
        put_i16_ne, i16::to_ne_bytes,
        put_u32_le, u32::to_le_bytes,
        put_u32_be, u32::to_be_bytes,
        put_u32_ne, u32::to_ne_bytes,
        put_i32_le, i32::to_le_bytes,
        put_i32_be, i32::to_be_bytes,
        put_i32_ne, i32::to_ne_bytes,
        put_u64_le, u64::to_le_bytes,
        put_u64_be, u64::to_be_bytes,
        put_u64_ne, u64::to_ne_bytes,
        put_i64_le, i64::to_le_bytes,
        put_i64_be, i64::to_be_bytes,
        put_i64_ne, i64::to_ne_bytes,
        put_u128_le, u128::to_le_bytes,
        put_u128_be, u128::to_be_bytes,
        put_u128_ne, u128::to_ne_bytes,
        put_i128_le, i128::to_le_bytes,
        put_i128_be, i128::to_be_bytes,
        put_i128_ne, i128::to_ne_bytes,
        put_f32_le, f32::to_le_bytes,
        put_f32_be, f32::to_be_bytes,
        put_f32_ne, f32::to_ne_bytes,
        put_f64_le, f64::to_le_bytes,
        put_f64_be, f64::to_be_bytes,
        put_f64_ne, f64::to_ne_bytes,
    }
}

impl BytesMut for &mut [u8] {
    fn put(&mut self, val: &[T]) {
        let n = val.len();
        let len = self.len();
        let ptr = (*self).as_mut_ptr();

        if len >= n {
            unsafe {
                let write = slice::from_raw_parts_mut(ptr, n);
                write.copy_from_slice(val);
                *self = slice::from_raw_parts_mut(ptr.add(n), len - n);
            }
        }
    }
}