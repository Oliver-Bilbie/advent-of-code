pub trait Integer {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}

pub trait SignedInteger {}
impl SignedInteger for i8 {}
impl SignedInteger for i16 {}
impl SignedInteger for i32 {}
impl SignedInteger for i64 {}
impl SignedInteger for i128 {}
impl SignedInteger for isize {}

pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
macro_rules! impl_constants {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0
                }
            }
            impl One for $t {
                fn one() -> Self {
                    1
                }
            }
        )*
    };
}
impl_constants!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
