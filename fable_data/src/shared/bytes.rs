use std::slice;
use std::mem;
use std::marker::PhantomData;

#[derive(Debug,Eq,PartialEq)]
pub struct BadPos;

// pub(crate) struct Look<T, B: AsRef<[T]>> {
//     buf: B,
//     pos: usize,
//     phantom: PhantomData<T>,
// }

pub(crate) trait View<T>: AsRef<[T]> {
    fn forward(&mut self, n: usize) -> Result<&[T], BadPos>;
}

pub(crate) trait ViewMut<T: Copy>: AsMut<[T]> {
    fn put(&mut self, val: &[T]) -> Result<(), BadPos>;
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

// impl<T, B: AsRef<[T]>> AsRef<[T]> for Look<T, B> {
//     fn as_ref(&self) -> &[T] {
//         &self.buf.as_ref()[self.pos ..]
//     }
// }

// impl<T, B: AsRef<[T]>> View<T> for Look<T, B> {
//     fn forward(&mut self, n: usize) -> Result<&[T], BadPos> {
//         let buf = self.buf.as_ref();
//         let len = buf.len();
//         if n > len { return Err(BadPos) }
//         let out = &buf[..n];
//         self.pos += n;
//         Ok(out)
//     }
// }

// impl<T, B: AsRef<[T]> + AsMut<[T]>> AsMut<[T]> for Look<T, B> {
//     fn as_mut(&mut self) -> &mut [T] {
//         &mut self.buf.as_mut()[self.pos ..]
//     }
// }

// impl<T: Copy, B: AsRef<[T]> + AsMut<[T]>> ViewMut<T> for Look<T, B> {
//     fn put(&mut self, val: &[T]) -> Result<(), BadPos> {
//         let buf = self.as_mut();
//         let n = val.len();
//         let len = buf.len();
//         if n > len { return Err(BadPos) }
//         let write = &mut buf[..n];
//         write.copy_from_slice(val);
//         self.pos += n;
//         Ok(())
//     }
// }

// impl<B: AsRef<[u8]>> Bytes for Look<u8, B> {}

// impl<B: AsRef<[u8]> + AsMut<[u8]>> BytesMut for Look<u8, B> {}

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
    fn forward(&mut self, n: usize) -> Result<&[T], BadPos> {
        let len = self.len();
        if n > len { return Err(BadPos) }
        let out = unsafe { slice::from_raw_parts(self.as_ptr(), n) };
        *self = unsafe { slice::from_raw_parts(self.as_ptr().add(n), len - n) };
        Ok(out)
    }
}

impl<T> View<T> for &mut [T] {
    fn forward(&mut self, n: usize) -> Result<&[T], BadPos> {
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
            let bytes = $self.forward(mem::size_of::<$typ>())?;
            Ok($typ::$conv(unsafe { *(bytes.as_ptr() as *const [u8; mem::size_of::<$typ>()]) }))
        }
    }
}

pub(crate) trait Bytes: View<u8> {
    fn take_u8(&mut self) -> Result<u8, BadPos> {
        Ok(self.forward(1)?[0])
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
        core::str::from_utf8(self.forward(n)?).map_err(|_| BadPos)
    }

    fn take_until_nul(&mut self) -> Result<&[u8], BadPos> {
        let len = self.as_ref().iter().take_while(|x| **x != b'\0').count();
        let out = self.forward(len + 1)?;
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
        let out = self.forward(prefix as usize)?;
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