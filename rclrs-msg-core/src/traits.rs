pub trait ZeroInit {
    fn zero_init() -> Self;
}

impl ZeroInit for String {
    fn zero_init() -> Self {
        "".into()
    }
}

impl<T> ZeroInit for Vec<T> {
    fn zero_init() -> Self {
        Self::new()
    }
}

pub trait FFIToRust {
    type Target;

    fn to_rust(&self) -> Self::Target;
}

pub trait FFIFromRust {
    type From;

    unsafe fn from_rust(from: &Self::From) -> Self;
}

macro_rules! impl_traits_to_primitive {
    ($type: ty) => {
        impl ZeroInit for $type {
            fn zero_init() -> Self {
                Self::default()
            }
        }

        impl FFIToRust for $type {
            type Target = Self;

            fn to_rust(&self) -> Self::Target {
                *self
            }
        }
    };
}

impl_traits_to_primitive!(i8);
impl_traits_to_primitive!(i16);
impl_traits_to_primitive!(i32);
impl_traits_to_primitive!(i64);
impl_traits_to_primitive!(u8);
impl_traits_to_primitive!(u16);
impl_traits_to_primitive!(u32);
impl_traits_to_primitive!(u64);
impl_traits_to_primitive!(f32);
impl_traits_to_primitive!(f64);
impl_traits_to_primitive!(bool);
