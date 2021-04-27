use std::slice;
use std::mem;
use std::marker::PhantomData;

#[derive(Debug,Eq,PartialEq)]
pub struct BadPos;

pub(crate) struct Look<T, B> {
    buf: B,
    pos: usize,
    phantom: PhantomData<T>,
}

pub(crate) trait View<T>: AsRef<[T]> {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos>;
}

// impl<T, B: AsRef<[T]>> Look<T, B> {
//     pub fn new(buf: B) -> Look<T, B> {
//         Look { buf, pos: 0, phantom: Default::default() }
//     }

//     pub fn new_with_pos(buf: B, pos: usize) -> Result<Look<T, B>, BadPos> {
//         let buf_ref = buf.as_ref();
//         if pos > buf_ref.len() { return Err(BadPos) }
//         Ok(Look { buf, pos, phantom: Default::default() })
//     }

//     pub fn from_slice<'a, N: AsRef<[T]> + 'a>(buf: B, slice: &'a [T]) -> Result<Look<T, B>, BadPos> {
//         let buf_ref = buf.as_ref();
//         let buf_start_ptr = buf_ref.as_ptr();
//         let buf_end_ptr = unsafe { buf_start_ptr.add(buf_ref.len()) };
//         let slice_start_ptr = slice.as_ptr();
//         let slice_end_ptr = unsafe { slice_start_ptr.add(slice.len()) };

//         if
//             slice_start_ptr < buf_start_ptr ||
//             slice_start_ptr > buf_end_ptr ||
//             slice_end_ptr > buf_end_ptr
//         {
//             return Err(BadPos)
//         }

//         let pos = buf_end_ptr as usize - slice_start_ptr as usize;

//         Ok(Look { buf, pos, phantom: Default::default() })
//     }

//     pub fn pos(&self) -> usize {
//         self.pos
//     }

//     pub fn into_inner(self) -> B {
//         self.buf
//     }
// }

impl<T, B: AsRef<[T]>> AsRef<[T]> for Look<T, B> {
    fn as_ref(&self) -> &[T] {
        &self.buf.as_ref()[self.pos ..]
    }
}

impl<T, B: AsMut<[T]>> AsMut<[T]> for Look<T, B> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.buf.as_mut()[self.pos ..]
    }
}

impl<T, B: AsRef<[T]>> View<T> for Look<T, B> {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos> {
        let buf = self.buf.as_ref();
        let len = buf.len();
        if n > len { return Err(BadPos) }
        let out = &buf[..n];
        self.pos += n;
        Ok(out)
    }
}

impl<T: Copy, B: AsMut<[T]>> ViewMut<T> for Look<T, B> {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos> {
        let buf = self.as_mut();
        let n = val.len();
        let len = buf.len();
        if n > len { return Err(BadPos) }
        let write = &mut buf[..n];
        write.copy_from_slice(val);
        self.pos += n;
        Ok(())
    }
}

impl<B: AsRef<[u8]>> Bytes for Look<u8, B> {}

impl<B: AsMut<[u8]>> BytesMut for Look<u8, B> {}

pub(crate) trait ViewMut<T: Copy>: AsMut<[T]> {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos>;
}

impl<T: Copy> ViewMut<T> for &mut [T] {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos> {
        let n = val.len();
        let len = self.len();
        if n > len { return Err(BadPos) }
        let write = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), n) };
        write.copy_from_slice(val);
        *self = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr().add(n), len - n) };
        Ok(())
    }
}

pub(crate) trait BytesMut: ViewMut<u8> {
    fn put_u8(&mut self, val: u8) -> Result<(), BadPos> {
        self.put(&[val])
    }

    fn put_i8(&mut self, val: i8) -> Result<(), BadPos> {
        self.put(&[val as u8])
    }

    fn put_u16_le(&mut self, val: u16) -> Result<(), BadPos> {
        self.put(&u16::to_le_bytes(val))
    }

    fn put_u16_be(&mut self, val: u16) -> Result<(), BadPos> {
        self.put(&u16::to_be_bytes(val))
    }

    fn put_u16_ne(&mut self, val: u16) -> Result<(), BadPos> {
        self.put(&u16::to_ne_bytes(val))
    }

    fn put_i16_le(&mut self, val: i16) -> Result<(), BadPos> {
        self.put(&i16::to_le_bytes(val))
    }

    fn put_i16_be(&mut self, val: i16) -> Result<(), BadPos> {
        self.put(&i16::to_be_bytes(val))
    }

    fn put_i16_ne(&mut self, val: i16) -> Result<(), BadPos> {
        self.put(&i16::to_ne_bytes(val))
    }

    fn put_u32_le(&mut self, val: u32) -> Result<(), BadPos> {
        self.put(&u32::to_le_bytes(val))
    }

    fn put_u32_be(&mut self, val: u32) -> Result<(), BadPos> {
        self.put(&u32::to_be_bytes(val))
    }

    fn put_u32_ne(&mut self, val: u32) -> Result<(), BadPos> {
        self.put(&u32::to_ne_bytes(val))
    }

    fn put_i32_le(&mut self, val: i32) -> Result<(), BadPos> {
        self.put(&i32::to_le_bytes(val))
    }

    fn put_i32_be(&mut self, val: i32) -> Result<(), BadPos> {
        self.put(&i32::to_be_bytes(val))
    }

    fn put_i32_ne(&mut self, val: i32) -> Result<(), BadPos> {
        self.put(&i32::to_ne_bytes(val))
    }

    fn put_u64_le(&mut self, val: u64) -> Result<(), BadPos> {
        self.put(&u64::to_le_bytes(val))
    }

    fn put_u64_be(&mut self, val: u64) -> Result<(), BadPos> {
        self.put(&u64::to_be_bytes(val))
    }

    fn put_u64_ne(&mut self, val: u64) -> Result<(), BadPos> {
        self.put(&u64::to_ne_bytes(val))
    }

    fn put_i64_le(&mut self, val: i64) -> Result<(), BadPos> {
        self.put(&i64::to_le_bytes(val))
    }

    fn put_i64_be(&mut self, val: i64) -> Result<(), BadPos> {
        self.put(&i64::to_be_bytes(val))
    }

    fn put_i64_ne(&mut self, val: i64) -> Result<(), BadPos> {
        self.put(&i64::to_ne_bytes(val))
    }

    fn put_u128_le(&mut self, val: u128) -> Result<(), BadPos> {
        self.put(&u128::to_le_bytes(val))
    }

    fn put_u128_be(&mut self, val: u128) -> Result<(), BadPos> {
        self.put(&u128::to_be_bytes(val))
    }

    fn put_u128_ne(&mut self, val: u128) -> Result<(), BadPos> {
        self.put(&u128::to_ne_bytes(val))
    }

    fn put_i128_le(&mut self, val: i128) -> Result<(), BadPos> {
        self.put(&i128::to_le_bytes(val))
    }

    fn put_i128_be(&mut self, val: i128) -> Result<(), BadPos> {
        self.put(&i128::to_be_bytes(val))
    }

    fn put_i128_ne(&mut self, val: i128) -> Result<(), BadPos> {
        self.put(&i128::to_ne_bytes(val))
    }

    fn put_f32_le(&mut self, val: f32) -> Result<(), BadPos> {
        self.put(&f32::to_le_bytes(val))
    }

    fn put_f32_be(&mut self, val: f32) -> Result<(), BadPos> {
        self.put(&f32::to_be_bytes(val))
    }

    fn put_f32_ne(&mut self, val: f32) -> Result<(), BadPos> {
        self.put(&f32::to_ne_bytes(val))
    }

    fn put_f64_le(&mut self, val: f64) -> Result<(), BadPos> {
        self.put(&f64::to_le_bytes(val))
    }

    fn put_f64_be(&mut self, val: f64) -> Result<(), BadPos> {
        self.put(&f64::to_be_bytes(val))
    }

    fn put_f64_ne(&mut self, val: f64) -> Result<(), BadPos> {
        self.put(&f64::to_ne_bytes(val))
    }
}

impl BytesMut for &mut [u8] {}

impl<T> View<T> for &[T] {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos> {
        let len = self.len();
        if n > len { return Err(BadPos) }
        let out = unsafe { slice::from_raw_parts(self.as_ptr(), n) };
        *self = unsafe { slice::from_raw_parts(self.as_ptr().add(n), len - n) };
        Ok(out)
    }
}

impl<T> View<T> for &mut [T] {
    fn take(&mut self, n: usize) -> Result<&[T], BadPos> {
        let len = self.len();
        if n > len { return Err(BadPos) }
        let out = unsafe { slice::from_raw_parts(self.as_ptr(), n) };
        *self = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr().add(n), len - n) };
        Ok(out)
    }
}

macro_rules! take_int {
    ($self:ident, $typ:tt::$conv:tt) => {
        {
            let bytes = $self.take(mem::size_of::<$typ>())?;
            Ok($typ::$conv(unsafe { *(bytes.as_ptr() as *const [u8; mem::size_of::<$typ>()]) }))
        }
    }
}

pub(crate) trait Bytes: View<u8> {
    fn take_u8(&mut self) -> Result<u8, BadPos> {
        Ok(self.take(1)?[0])
    }

    fn take_i8(&mut self) -> Result<i8, BadPos> {
        Ok(self.take_u8()? as i8)
    }

    fn take_u16_le(&mut self) -> Result<u16, BadPos> {
        take_int!(self, u16::from_le_bytes)
    }

    fn take_u16_be(&mut self) -> Result<u16, BadPos> {
        take_int!(self, u16::from_be_bytes)
    }

    fn take_u16_ne(&mut self) -> Result<u16, BadPos> {
        take_int!(self, u16::from_ne_bytes)
    }

    fn take_i16_le(&mut self) -> Result<i16, BadPos> {
        take_int!(self, i16::from_le_bytes)
    }

    fn take_i16_be(&mut self) -> Result<i16, BadPos> {
        take_int!(self, i16::from_be_bytes)
    }

    fn take_i16_ne(&mut self) -> Result<i16, BadPos> {
        take_int!(self, i16::from_ne_bytes)
    }

    fn take_u32_le(&mut self) -> Result<u32, BadPos> {
        take_int!(self, u32::from_le_bytes)
    }

    fn take_u32_be(&mut self) -> Result<u32, BadPos> {
        take_int!(self, u32::from_be_bytes)
    }

    fn take_u32_ne(&mut self) -> Result<u32, BadPos> {
        take_int!(self, u32::from_ne_bytes)
    }

    fn take_i32_le(&mut self) -> Result<i32, BadPos> {
        take_int!(self, i32::from_le_bytes)
    }

    fn take_i32_be(&mut self) -> Result<i32, BadPos> {
        take_int!(self, i32::from_be_bytes)
    }

    fn take_i32_ne(&mut self) -> Result<i32, BadPos> {
        take_int!(self, i32::from_ne_bytes)
    }

    fn take_u64_le(&mut self) -> Result<u64, BadPos> {
        take_int!(self, u64::from_le_bytes)
    }

    fn take_u64_be(&mut self) -> Result<u64, BadPos> {
        take_int!(self, u64::from_be_bytes)
    }

    fn take_u64_ne(&mut self) -> Result<u64, BadPos> {
        take_int!(self, u64::from_ne_bytes)
    }

    fn take_i64_le(&mut self) -> Result<i64, BadPos> {
        take_int!(self, i64::from_le_bytes)
    }

    fn take_i64_be(&mut self) -> Result<i64, BadPos> {
        take_int!(self, i64::from_be_bytes)
    }

    fn take_i64_ne(&mut self) -> Result<i64, BadPos> {
        take_int!(self, i64::from_ne_bytes)
    }

    fn take_u128_le(&mut self) -> Result<u128, BadPos> {
        take_int!(self, u128::from_le_bytes)
    }

    fn take_u128_be(&mut self) -> Result<u128, BadPos> {
        take_int!(self, u128::from_be_bytes)
    }

    fn take_u128_ne(&mut self) -> Result<u128, BadPos> {
        take_int!(self, u128::from_ne_bytes)
    }

    fn take_i128_le(&mut self) -> Result<i128, BadPos> {
        take_int!(self, i128::from_le_bytes)
    }

    fn take_i128_be(&mut self) -> Result<i128, BadPos> {
        take_int!(self, i128::from_be_bytes)
    }

    fn take_i128_ne(&mut self) -> Result<i128, BadPos> {
        take_int!(self, i128::from_ne_bytes)
    }

    fn take_f32_le(&mut self) -> Result<f32, BadPos> {
        take_int!(self, f32::from_le_bytes)
    }

    fn take_f32_be(&mut self) -> Result<f32, BadPos> {
        take_int!(self, f32::from_be_bytes)
    }

    fn take_f32_ne(&mut self) -> Result<f32, BadPos> {
        take_int!(self, f32::from_ne_bytes)
    }

    fn take_f64_le(&mut self) -> Result<f64, BadPos> {
        take_int!(self, f64::from_le_bytes)
    }

    fn take_f64_be(&mut self) -> Result<f64, BadPos> {
        take_int!(self, f64::from_be_bytes)
    }

    fn take_f64_ne(&mut self) -> Result<f64, BadPos> {
        take_int!(self, f64::from_ne_bytes)
    }

    /// Invalid UTF8 is considered an invalid position and out of bounds.
    fn take_as_str(&mut self, n: usize) -> Result<&str, BadPos> {
        core::str::from_utf8(self.take(n)?).map_err(|_| BadPos)
    }

    fn take_until_nul(&mut self) -> Result<&[u8], BadPos> {
        let len = self.as_ref().iter().take_while(|x| **x != b'\0').count();
        let out = self.take(len + 1)?;
        let out = &out[..len];
        Ok(out)
    }

    fn take_as_str_until_nul(&mut self) -> Result<&str, BadPos> {
        let out = self.take_until_nul()?;
        let out = core::str::from_utf8(out).map_err(|_| BadPos)?;
        Ok(out)
    }

    fn take_with_u32_le_prefix(&mut self) -> Result<&[u8], BadPos> {
        let prefix = self.take_u32_le()?;
        let out = self.take(prefix as usize)?;
        Ok(out)
    }

    fn take_as_str_with_u32_le_prefix(&mut self) -> Result<&str, BadPos> {
        let out = self.take_with_u32_le_prefix()?;
        let out = std::str::from_utf8(out).map_err(|_| BadPos)?;
        Ok(out)
    }
}

impl Bytes for &[u8] {}
impl Bytes for &mut [u8] {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Look,BytesMut};

    #[test]
    fn test_take() {
        let mut a = &[2,0,0,0,b'H',b'i',b'\0'][..];

        let out = a.take(0);
        assert!(out == Ok(&[]), "take none");
        assert!(a == &[2,0,0,0,b'H',b'i',b'\0'][..], "took none");

        let four = a.take(4);
        assert!(four == Ok(&[2,0,0,0]), "take 4");
        assert!(a == &[b'H',b'i',b'\0'][..]);

        let four_err = a.take(4);
        assert!(four_err.is_err(), "take 4 out of bounds");
        assert!(a == &[b'H',b'i',b'\0'][..], "took none");

        let rest = a.take(3);
        assert!(rest == Ok(&[b'H',b'i',b'\0']), "take rest");
        assert!(a == &[], "took rest");

        let one_err = a.take(1);
        assert!(one_err.is_err(), "take 1 out of bounds");
        assert!(a == &[], "took none");
    }

    #[test]
    fn test_take_u8() {
        let mut a: &[u8] = &[0,1];

        let b = a.take_u8();
        assert!(b == Ok(0));
        assert!(a == &[1]);

        let c = a.take_u8();
        assert!(c == Ok(1));
        assert!(a == &[]);

        let d = a.take_u8();
        assert!(d.is_err());
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i8() {
        let mut a: &[u8] = &[0,1,-1i8 as u8];

        let b = a.take_i8();
        assert!(b == Ok(0));
        assert!(a == &[1,-1i8 as u8]);

        let c = a.take_i8();
        assert!(c == Ok(1));
        assert!(a == &[-1i8 as u8]);

        let d = a.take_i8();
        assert!(d == Ok(-1));
        assert!(a == &[]);

        let e = a.take_i8();
        assert!(e.is_err());
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u16() {
        let mut a = &[1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u16_le();
        assert!(b == Ok(1));
        assert!(a == &u16::MAX.to_le_bytes()[..]);

        let c = a.take_u16_le();
        assert!(c == Ok(u16::MAX));
        assert!(a == &[]);

        let mut a = &[1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u16_be();
        assert!(b == Ok(1));
        assert!(a == &u16::MAX.to_be_bytes()[..]);

        let c = a.take_u16_be();
        assert!(c == Ok(u16::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i16() {
        let mut a = &[1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat()[..];

        let b = a.take_i16_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i16).to_le_bytes()[..]);

        let c = a.take_i16_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat()[..];

        let b = a.take_i16_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i16).to_be_bytes()[..]);

        let c = a.take_i16_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u32() {
        let mut a = &[1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u32_le();
        assert!(b == Ok(1));
        assert!(a == &u32::MAX.to_le_bytes()[..]);

        let c = a.take_u32_le();
        assert!(c == Ok(u32::MAX));
        assert!(a == &[]);

        let mut a = &[1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u32_be();
        assert!(b == Ok(1));
        assert!(a == &u32::MAX.to_be_bytes()[..]);

        let c = a.take_u32_be();
        assert!(c == Ok(u32::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i32() {
        let mut a = &[1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat()[..];

        let b = a.take_i32_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i32).to_le_bytes()[..]);

        let c = a.take_i32_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat()[..];

        let b = a.take_i32_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i32).to_be_bytes()[..]);

        let c = a.take_i32_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u64() {
        let mut a = &[1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u64_le();
        assert!(b == Ok(1));
        assert!(a == &u64::MAX.to_le_bytes()[..]);

        let c = a.take_u64_le();
        assert!(c == Ok(u64::MAX));
        assert!(a == &[]);

        let mut a = &[1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u64_be();
        assert!(b == Ok(1));
        assert!(a == &u64::MAX.to_be_bytes()[..]);

        let c = a.take_u64_be();
        assert!(c == Ok(u64::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i64() {
        let mut a = &[1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat()[..];

        let b = a.take_i64_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i64).to_le_bytes()[..]);

        let c = a.take_i64_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat()[..];

        let b = a.take_i64_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i64).to_be_bytes()[..]);

        let c = a.take_i64_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_u128() {
        let mut a = &[1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat()[..];

        let b = a.take_u128_le();
        assert!(b == Ok(1));
        assert!(a == &u128::MAX.to_le_bytes()[..]);

        let c = a.take_u128_le();
        assert!(c == Ok(u128::MAX));
        assert!(a == &[]);

        let mut a = &[1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat()[..];

        let b = a.take_u128_be();
        assert!(b == Ok(1));
        assert!(a == &u128::MAX.to_be_bytes()[..]);

        let c = a.take_u128_be();
        assert!(c == Ok(u128::MAX));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_i128() {
        let mut a = &[1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat()[..];

        let b = a.take_i128_le();
        assert!(b == Ok(1));
        assert!(a == &(-1i128).to_le_bytes()[..]);

        let c = a.take_i128_le();
        assert!(c == Ok(-1));
        assert!(a == &[]);

        let mut a = &[1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat()[..];

        let b = a.take_i128_be();
        assert!(b == Ok(1));
        assert!(a == &(-1i128).to_be_bytes()[..]);

        let c = a.take_i128_be();
        assert!(c == Ok(-1));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_f32() {
        let mut a = &[1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat()[..];

        let b = a.take_f32_le();
        assert!(b == Ok(1f32));
        assert!(a == &(-1f32).to_le_bytes()[..]);

        let c = a.take_f32_le();
        assert!(c == Ok(-1f32));
        assert!(a == &[]);

        let mut a = &[1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat()[..];

        let b = a.take_f32_be();
        assert!(b == Ok(1f32));
        assert!(a == &(-1f32).to_be_bytes()[..]);

        let c = a.take_f32_be();
        assert!(c == Ok(-1f32));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_f64() {
        let mut a = &[1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat()[..];

        let b = a.take_f64_le();
        assert!(b == Ok(1f64));
        assert!(a == &(-1f64).to_le_bytes()[..]);

        let c = a.take_f64_le();
        assert!(c == Ok(-1f64));
        assert!(a == &[]);

        let mut a = &[1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat()[..];

        let b = a.take_f64_be();
        assert!(b == Ok(1f64));
        assert!(a == &(-1f64).to_be_bytes()[..]);

        let c = a.take_f64_be();
        assert!(c == Ok(-1f64));
        assert!(a == &[]);
    }

    #[test]
    fn test_take_until_nul() {
        let mut a = &[b'H',b'i',b'\0'][..];
        let b = a.take_until_nul();
        assert!(b == Ok(&[b'H',b'i']));
        assert!(a == &[]);
    }

    #[test]
    fn test_put() {
        let src = &mut [2,0,0,0,b'H',b'i',b'\0'][..];
        let mut a = Look::new(src);

        let out = a.put(&[]);
        assert!(out.is_ok());
        assert!(a.as_ref() == &[2,0,0,0,b'H',b'i',b'\0'][..]);
        // assert!(src == &[2,0,0,0,b'H',b'i',b'\0'][..]);

        let four = a.put(&[0xFF,0xFF,0xFF,0xFF]);
        assert!(four.is_ok(), "put 4");
        assert!(a.as_ref() == &[b'H',b'i',b'\0'][..]);
        // assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'H',b'i',b'\0'][..]);

        let four_err = a.put(&[0xFF,0xFF,0xFF,0xFF]);
        assert!(four_err.is_err(), "put 4 out of bounds");
        assert!(a.as_ref() == &[b'H',b'i',b'\0'][..]);
        // assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'H',b'i',b'\0'][..]);

        let rest = a.put(&[b'O',b'k',b'\0']);
        assert!(rest.is_ok(), "put rest");
        assert!(a.as_ref() == &[]);
        // assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'O',b'k',b'\0'][..]);

        let one_err = a.put(&[0xFF]);
        assert!(one_err.is_err(), "put 1 out of bounds");
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();

        assert!(src == &[0xFF,0xFF,0xFF,0xFF,b'O',b'k',b'\0'][..]);
    }

    #[test]
    fn test_put_u8() {
        let src: &mut [u8] = &mut [0,1][..];
        let mut a = Look::new(src);

        let b = a.put_u8(0xFF);
        assert!(b.is_ok());
        assert!(a.as_ref() == &[1]);

        let c = a.put_u8(0xFE);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let d = a.put_u8(0xFD);
        assert!(d.is_err());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[0xFF,0xFE]);
    }

    #[test]
    fn test_put_i8() {
        let src: &mut [u8] = &mut [0,1,-1i8 as u8];
        let mut a = Look::new(src);

        let b = a.put_i8(-1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &[1,-1i8 as u8]);

        let c = a.put_i8(0);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[-1i8 as u8]);

        let d = a.put_i8(1);
        assert!(d.is_ok());
        assert!(a.as_ref() == &[]);

        let e = a.put_i8(-2);
        assert!(e.is_err());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[-1i8 as u8,0,1]);
    }

    #[test]
    fn test_put_u16() {
        let src = &mut [1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u16_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u16::MAX.to_le_bytes()[..]);

        let c = a.put_u16_le(u16::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u16.to_le_bytes(),u16::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u16_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u16::MAX.to_be_bytes()[..]);

        let c = a.put_u16_be(u16::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u16.to_be_bytes(),u16::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i16() {
        let src: &mut [u8] = &mut [1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i16_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i16).to_le_bytes()[..]);

        let c = a.put_i16_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i16.to_le_bytes(),(-1i16).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i16_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i16).to_be_bytes()[..]);

        let c = a.put_i16_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i16.to_be_bytes(),(-1i16).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_u32() {
        let src = &mut [1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u32_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u32::MAX.to_le_bytes()[..]);

        let c = a.put_u32_le(u32::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u32.to_le_bytes(),u32::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u32_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u32::MAX.to_be_bytes()[..]);

        let c = a.put_u32_be(u32::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u32.to_be_bytes(),u32::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i32() {
        let src: &mut [u8] = &mut [1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i32_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i32).to_le_bytes()[..]);

        let c = a.put_i32_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i32.to_le_bytes(),(-1i32).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i32_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i32).to_be_bytes()[..]);

        let c = a.put_i32_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i32.to_be_bytes(),(-1i32).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_u64() {
        let src = &mut [1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u64_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u64::MAX.to_le_bytes()[..]);

        let c = a.put_u64_le(u64::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u64.to_le_bytes(),u64::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u64_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u64::MAX.to_be_bytes()[..]);

        let c = a.put_u64_be(u64::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u64.to_be_bytes(),u64::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i64() {
        let src: &mut [u8] = &mut [1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i64_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i64).to_le_bytes()[..]);

        let c = a.put_i64_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i64.to_le_bytes(),(-1i64).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i64_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i64).to_be_bytes()[..]);

        let c = a.put_i64_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i64.to_be_bytes(),(-1i64).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_u128() {
        let src = &mut [1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u128_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u128::MAX.to_le_bytes()[..]);

        let c = a.put_u128_le(u128::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u128.to_le_bytes(),u128::MAX.to_le_bytes()].concat()[..]);

        let src = &mut [1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_u128_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &u128::MAX.to_be_bytes()[..]);

        let c = a.put_u128_be(u128::MAX);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1u128.to_be_bytes(),u128::MAX.to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_i128() {
        let src: &mut [u8] = &mut [1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i128_le(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i128).to_le_bytes()[..]);

        let c = a.put_i128_le(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i128.to_le_bytes(),(-1i128).to_le_bytes()].concat()[..]);

        let src: &mut [u8] = &mut [1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_i128_be(1);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1i128).to_be_bytes()[..]);

        let c = a.put_i128_be(-1);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1i128.to_be_bytes(),(-1i128).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_f32() {
        let src = &mut [1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f32_le(1f32);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f32).to_le_bytes()[..]);

        let c = a.put_f32_le(-1f32);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f32.to_le_bytes(),(-1f32).to_le_bytes()].concat()[..]);

        let src = &mut [1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f32_be(1f32);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f32).to_be_bytes()[..]);

        let c = a.put_f32_be(-1f32);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f32.to_be_bytes(),(-1f32).to_be_bytes()].concat()[..]);
    }

    #[test]
    fn test_put_f64() {
        let src = &mut [1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f64_le(1f64);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f64).to_le_bytes()[..]);

        let c = a.put_f64_le(-1f64);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f64.to_le_bytes(),(-1f64).to_le_bytes()].concat()[..]);

        let src = &mut [1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat()[..];
        let mut a = Look::new(src);

        let b = a.put_f64_be(1f64);
        assert!(b.is_ok());
        assert!(a.as_ref() == &(-1f64).to_be_bytes()[..]);

        let c = a.put_f64_be(-1f64);
        assert!(c.is_ok());
        assert!(a.as_ref() == &[]);

        let src = a.into_inner();
        assert!(src == &[1f64.to_be_bytes(),(-1f64).to_be_bytes()].concat()[..]);
    }
}