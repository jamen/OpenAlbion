use std::mem;
use std::slice;

macro_rules! impl_num_parse {
    ($($fn_name:ident, $typ:tt::$conv:tt,)*) => {
        $(
            fn $fn_name (&mut self) -> Option<$typ> {
                let bytes = self.advance(mem::size_of::<$typ>())?;
                Some($typ::$conv(unsafe { *(bytes.as_ptr() as *const [u8; mem::size_of::<$typ>()]) }))
            }
        )*
    }
}

macro_rules! impl_mint_parse {
    ($($fn_name:ident, $typ:ty, [ $($c:expr),* ],)*) => {
        $(
            fn $fn_name (&mut self) -> Option<$typ> {
                Some([ $($c(self)?),* ].into())
            }
        )*
    }
}

pub(crate) trait Bytes: AsRef<[u8]> {
    fn advance(&mut self, n: usize) -> Option<&[u8]>;

    impl_num_parse! {
        parse_u16_le, u16::from_le_bytes,
        parse_u16_be, u16::from_be_bytes,
        parse_u16_ne, u16::from_ne_bytes,
        parse_i16_le, i16::from_le_bytes,
        parse_i16_be, i16::from_be_bytes,
        parse_i16_ne, i16::from_ne_bytes,
        parse_u32_le, u32::from_le_bytes,
        parse_u32_be, u32::from_be_bytes,
        parse_u32_ne, u32::from_ne_bytes,
        parse_i32_le, i32::from_le_bytes,
        parse_i32_be, i32::from_be_bytes,
        parse_i32_ne, i32::from_ne_bytes,
        parse_u64_le, u64::from_le_bytes,
        parse_u64_be, u64::from_be_bytes,
        parse_u64_ne, u64::from_ne_bytes,
        parse_i64_le, i64::from_le_bytes,
        parse_i64_be, i64::from_be_bytes,
        parse_i64_ne, i64::from_ne_bytes,
        parse_u128_le, u128::from_le_bytes,
        parse_u128_be, u128::from_be_bytes,
        parse_u128_ne, u128::from_ne_bytes,
        parse_i128_le, i128::from_le_bytes,
        parse_i128_be, i128::from_be_bytes,
        parse_i128_ne, i128::from_ne_bytes,
        parse_f32_le, f32::from_le_bytes,
        parse_f32_be, f32::from_be_bytes,
        parse_f32_ne, f32::from_ne_bytes,
        parse_f64_le, f64::from_le_bytes,
        parse_f64_be, f64::from_be_bytes,
        parse_f64_ne, f64::from_ne_bytes,
    }

    impl_mint_parse! {
        parse_vector2_f16_le, [F16; 2], [Self::parse_f16_le, Self::parse_f16_le],
        parse_vector2_f32_le, [f32; 2], [Self::parse_f32_le, Self::parse_f32_le],
        parse_vector3_f32_le, [f32; 3], [Self::parse_f32_le, Self::parse_f32_le, Self::parse_f32_le],
        parse_vector4_f16_le, [F16; 4], [
            Self::parse_f16_le,
            Self::parse_f16_le,
            Self::parse_f16_le,
            Self::parse_f16_le
        ],
        parse_vector4_f32_le, [f32; 4], [
            Self::parse_f32_le,
            Self::parse_f32_le,
            Self::parse_f32_le,
            Self::parse_f32_le
        ],
        parse_quaternion_f32_le, mint::Quaternion<f32>, [
            Self::parse_f32_le,
            Self::parse_f32_le,
            Self::parse_f32_le,
            Self::parse_f32_le
        ],
        parse_colmatrix3x4_f32_le, mint::ColumnMatrix3x4<f32>, [
            Self::parse_vector3_f32_le,
            Self::parse_vector3_f32_le,
            Self::parse_vector3_f32_le,
            Self::parse_vector3_f32_le
        ],
        parse_colmatrix4_f32_le, mint::ColumnMatrix4<f32>, [
            Self::parse_vector4_f32_le,
            Self::parse_vector4_f32_le,
            Self::parse_vector4_f32_le,
            Self::parse_vector4_f32_le
        ],
    }

    fn parse_u8(&mut self) -> Option<u8> {
        Some(self.advance(1)?[0])
    }
    fn parse_i8(&mut self) -> Option<i8> {
        Some(self.parse_u8()? as i8)
    }

    fn parse_f16_le(&mut self) -> Option<F16> {
        Some(F16::new_unchecked(self.parse_u16_le()?))
    }
    fn parse_f16_be(&mut self) -> Option<F16> {
        Some(F16::new_unchecked(self.parse_u16_be()?))
    }
    fn parse_f16_ne(&mut self) -> Option<F16> {
        Some(F16::new_unchecked(self.parse_u16_ne()?))
    }

    fn parse_vector3_packed(&mut self) -> Option<[f32; 3]> {
        Some(Vector3Packed::new_unchecked(self.parse_u32_le()?).into())
    }

    fn parse_str(&mut self, n: usize) -> Option<&str> {
        core::str::from_utf8(self.advance(n)?).ok()
    }

    fn parse_while(&mut self, pred: impl Fn(&&u8) -> bool) -> Option<&[u8]> {
        let len = self.as_ref().iter().take_while(pred).count();
        let out = self.advance(len + 1)?;
        let out = &out[..len];
        Some(out)
    }

    fn parse_until_nul(&mut self) -> Option<&[u8]> {
        self.parse_while(|x| **x != b'\0')
    }

    fn parse_str_until_nul(&mut self) -> Option<&str> {
        let out = self.parse_until_nul()?;
        let out = core::str::from_utf8(out).ok()?;
        Some(out)
    }

    fn parse_with_u32_le_prefix(&mut self) -> Option<&[u8]> {
        let prefix = self.parse_u32_le()?;
        let out = self.advance(prefix as usize)?;
        Some(out)
    }

    fn parse_str_with_u32_le_prefix(&mut self) -> Option<&str> {
        let out = self.parse_with_u32_le_prefix()?;
        let out = std::str::from_utf8(out).ok()?;
        Some(out)
    }
}

impl Bytes for &[u8] {
    fn advance(&mut self, n: usize) -> Option<&[u8]> {
        let len = self.len();
        let ptr = (*self).as_ptr();

        if n > len {
            None
        } else {
            unsafe {
                let out = slice::from_raw_parts(ptr, n);
                *self = slice::from_raw_parts(ptr.add(n), len - n);
                Some(out)
            }
        }
    }
}

impl Bytes for &mut [u8] {
    fn advance(&mut self, n: usize) -> Option<&[u8]> {
        let len = self.len();
        let ptr = (*self).as_mut_ptr();

        if n > len {
            None
        } else {
            unsafe {
                let out = slice::from_raw_parts_mut(ptr, n);
                *self = slice::from_raw_parts_mut(ptr.add(n), len - n);
                Some(out)
            }
        }
    }
}

macro_rules! impl_num_put {
    ($($fn_name:ident, $typ:tt::$conv:tt,)*) => {
        $(
            fn $fn_name(&mut self, val: $typ) {
                self.put(&$typ::$conv(val))
            }
        )*
    }
}

macro_rules! impl_mint_put {
    ($($fn_name:ident, $typ:ty, [ $($c:expr),* ],)*) => {
        $(
            fn $fn_name (&mut self, x: $typ) {
                let mut i = std::array::IntoIter::new(x.into());
                $($c(self, i.next().unwrap());)*
            }
        )*
    }
}

pub(crate) trait BytesMut: AsRef<[u8]> + AsMut<[u8]> {
    fn put(&mut self, val: &[u8]);

    fn put_u8(&mut self, val: u8) {
        self.put(&[val])
    }

    fn put_i8(&mut self, val: i8) {
        self.put(&[val as u8])
    }

    impl_num_put! {
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

    impl_mint_put! {
        put_vector2_f32_le, [f32; 2], [Self::put_f32_le, Self::put_f32_le],
        put_vector3_f32_le, [f32; 3], [Self::put_f32_le, Self::put_f32_le, Self::put_f32_le],
        put_vector4_f32_le, [f32; 4], [
            Self::put_f32_le,
            Self::put_f32_le,
            Self::put_f32_le,
            Self::put_f32_le
        ],
        put_quaternion_f32_le, mint::Quaternion<f32>, [
            Self::put_f32_le,
            Self::put_f32_le,
            Self::put_f32_le,
            Self::put_f32_le
        ],
        put_colmatrix3x4_f32_le, mint::ColumnMatrix3x4<f32>, [
            Self::put_vector3_f32_le,
            Self::put_vector3_f32_le,
            Self::put_vector3_f32_le,
            Self::put_vector3_f32_le
        ],
        put_colmatrix4_f32_le, mint::ColumnMatrix4<f32>, [
            Self::put_vector4_f32_le,
            Self::put_vector4_f32_le,
            Self::put_vector4_f32_le,
            Self::put_vector4_f32_le
        ],
    }
}

impl BytesMut for &mut [u8] {
    fn put(&mut self, val: &[u8]) {
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

/// A 3d vector where X and Y are 11-bit floats and Z is a 10-bit float.
#[derive(Copy, Clone)]
pub struct Vector3Packed(u32);

impl Vector3Packed {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3Packed(
            ((y * 1023.0) as u32 & 0x7ff | ((z * 511.0) as u32) << 11) << 11
                | (x * 1023.0) as u32 & 0x7ff,
        )
    }
    pub fn new_unchecked(x: u32) -> Self {
        Vector3Packed(x)
    }
}

impl From<[f32; 3]> for Vector3Packed {
    fn from(x: [f32; 3]) -> Vector3Packed {
        Vector3Packed::new(x[0], x[1], x[2])
    }
}

impl From<Vector3Packed> for [f32; 3] {
    fn from(x: Vector3Packed) -> [f32; 3] {
        [
            (((((u32::MAX - (x.0 & 0x400)) != 0) as u32) & 0xfffff800) | (x.0 & 0x7ff)) as f32
                * -0.00097752,
            (((((u32::MAX - (x.0 >> 0xb & 0x400)) != 0) as u32) & 0xfffff800)
                | (x.0 >> 0xb & 0x7ff)) as f32
                * 0.00097752,
            (((((u32::MAX - (x.0 >> 0x16 & 0x200)) != 0) as u32) & 0xfffffc00) | (x.0 >> 0x16))
                as f32
                * 0.00195695,
        ]
    }
}

/// A 16-bit float but the only thing you can do with it is make a 32-bit float
#[derive(Copy, Clone)]
pub struct F16(u16);

impl F16 {
    pub fn new(x: f32) -> Self {
        F16(((x + 8.0) * 2048.0) as u16)
    }
    pub fn new_unchecked(x: u16) -> Self {
        F16(x)
    }
}

impl From<f32> for F16 {
    fn from(x: f32) -> F16 {
        F16::new(x)
    }
}

impl From<F16> for f32 {
    fn from(x: F16) -> f32 {
        x.0 as f32 * 0.00048828 - 8.00000000
    }
}
