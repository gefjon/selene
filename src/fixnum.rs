use std::{fmt};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixnum(i32);

pub const fn fxn(i: i32) -> Fixnum {
    Fixnum(i)
}

impl fmt::LowerHex for Fixnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

macro_rules! derive_from_cast {
    ($ty:ty) => {
        impl From<$ty> for Fixnum {
            fn from(i: $ty) -> Self {
                Fixnum(i as _)
            }
        }
    };
    ($($ty:ty),*) => {
        $(derive_from_cast!($ty);)*
    };
}

derive_from_cast!(u8, u16, u32, i8, i16, i32, char);

macro_rules! derive_binop {
    ($trait:ident $func:ident) => {
        impl std::ops::$trait for Fixnum {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self {
                Fixnum((self.0).$func(rhs.0))
            }
        }
    };
    ($(($trait:ident $func:ident)),*) => {
        $(derive_binop!($trait $func);)*
    };
}

derive_binop!(
    (Add add),
    (Sub sub),
    (Mul mul),
    (Div div)
);

macro_rules! derive_assign_op {
    ($trait:ident $func:ident) => {
        impl std::ops::$trait for Fixnum {
            fn $func(&mut self, rhs: Self) {
                (self.0).$func(rhs.0)
            }
        }
    };
    ($(($trait:ident $func:ident)),*) => {
        $(derive_assign_op!($trait $func);)*
    }
}

derive_assign_op!(
    (AddAssign add_assign),
    (SubAssign sub_assign),
    (MulAssign mul_assign),
    (DivAssign div_assign)
);
