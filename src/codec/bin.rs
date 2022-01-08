pub mod read {
    use dangerous::error::{ExpectedLength, WithContext};
    use dangerous::{ByteArray, BytesReader};

    macro_rules! impl_num {
        ($name:ident, $ty:ident::from_le_bytes) => {
            impl_num!(
                $name,
                $ty::from_le_bytes,
                concat!("Read a little-endian `", stringify!($ty), "`"),
                concat!(stringify!($ty), "-le")
            );
        };
        ($name:ident, $ty:ident::from_be_bytes) => {
            impl_num!(
                $name,
                $ty::from_be_bytes,
                concat!("Read a big-endian `", stringify!($ty), "`"),
                concat!(stringify!($ty), "-be")
            );
        };
        ($name:ident, $ty:ident::$from_bytes:ident, $doc:expr, $desc:expr) => {
            #[doc = $doc]
            pub fn $name<'i, E>(r: &mut BytesReader<'i, E>) -> Result<$ty, E>
            where
                E: WithContext<'i>,
                E: From<ExpectedLength<'i>>,
            {
                r.context($desc, |r| {
                    r.take_array()
                        .map(ByteArray::into_dangerous)
                        .map($ty::$from_bytes)
                })
            }
        };
    }

    pub fn u8<'i, E>(r: &mut BytesReader<'i, E>) -> Result<u8, E>
    where
        E: From<ExpectedLength<'i>>,
    {
        r.read()
    }

    pub fn i8<'i, E>(r: &mut BytesReader<'i, E>) -> Result<i8, E>
    where
        E: WithContext<'i>,
        E: From<ExpectedLength<'i>>,
    {
        r.context("i8", |r| {
            r.take_array()
                .map(ByteArray::into_dangerous)
                .map(i8::from_ne_bytes)
        })
    }

    impl_num!(u16_le, u16::from_le_bytes);
    impl_num!(u16_be, u16::from_be_bytes);
    impl_num!(i16_le, i16::from_le_bytes);
    impl_num!(i16_be, i16::from_be_bytes);
    impl_num!(u32_le, u32::from_le_bytes);
    impl_num!(u32_be, u32::from_be_bytes);
    impl_num!(i32_le, i32::from_le_bytes);
    impl_num!(i32_be, i32::from_be_bytes);
    impl_num!(u64_le, u64::from_le_bytes);
    impl_num!(u64_be, u64::from_be_bytes);
    impl_num!(i64_le, i64::from_le_bytes);
    impl_num!(i64_be, i64::from_be_bytes);
    impl_num!(u128_le, u128::from_le_bytes);
    impl_num!(u128_be, u128::from_be_bytes);
    impl_num!(i128_le, i128::from_le_bytes);
    impl_num!(i128_be, i128::from_be_bytes);

    pub fn array<'i, E, const N: usize>(r: &mut BytesReader<'i, E>) -> Result<[u8; N], E>
    where
        E: WithContext<'i>,
        E: From<ExpectedLength<'i>>,
    {
        r.take_array().map(ByteArray::into_dangerous)
    }

    pub fn array_opt<E, const N: usize>(r: &mut BytesReader<'_, E>) -> Option<[u8; N]> {
        r.take_array_opt().map(ByteArray::into_dangerous)
    }

    pub fn array_ref<'i, E, const N: usize>(r: &mut BytesReader<'i, E>) -> Result<&'i [u8; N], E>
    where
        E: WithContext<'i>,
        E: From<ExpectedLength<'i>>,
    {
        r.take_array().map(|array| array.as_dangerous())
    }

    pub fn array_ref_opt<'i, E, const N: usize>(r: &mut BytesReader<'i, E>) -> Option<&'i [u8; N]> {
        r.take_array_opt().map(|array| array.as_dangerous())
    }
}
