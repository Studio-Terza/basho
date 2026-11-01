macro_rules! added {
    ($quantity:ident) => {
        impl std::ops::Add for $quantity {
            type Output = $quantity;

            #[inline]
            fn add(self, other: $quantity) -> $quantity {
                $quantity(self.0 + other.0)
            }
        }
    };
}

macro_rules! subtracted {
    ($quantity:ident) => {
        impl std::ops::Sub for $quantity {
            type Output = $quantity;

            #[inline]
            fn sub(self, other: $quantity) -> $quantity {
                $quantity(self.0 - other.0)
            }
        }
    };
}

macro_rules! signed {
    ($quantity:ident) => {
        impl std::ops::Neg for $quantity {
            type Output = $quantity;

            #[inline]
            fn neg(self) -> $quantity {
                $quantity(-self.0)
            }
        }

        impl $quantity {
            #[must_use]
            #[inline]
            pub fn abs(self) -> $quantity {
                $quantity(self.0.abs())
            }
        }
    };
}
