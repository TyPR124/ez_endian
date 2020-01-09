macro_rules! impl_bitwise_ops {
    ($EndianType:ident) => {
        impl_bitwise_binary_ops!($EndianType: [
            BitAnd::bitand,
            BitOr::bitor,
            BitXor::bitxor,
        ]);
        impl_bitwise_binary_ops_assign!($EndianType: [
            BitAndAssign::bitand_assign,
            BitOrAssign::bitor_assign,
            BitXorAssign::bitxor_assign,
        ]);
        impl ::core::ops::Not for $EndianType {
            type Output = Self;
            fn not(self) -> Self {
                Self::from_bits(self.to_bits().not())
            }
        }
        impl_bitwise_shift_ops!($EndianType);
    };
}

macro_rules! impl_bitwise_binary_ops {
    ($EndianType:ident : [$($Op:ident :: $method:ident),*$(,)?]) => { $(
        impl ::core::ops::$Op for $EndianType {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self {
                Self::from_bits(self.to_bits().$method(&rhs.to_bits()))
            }
        }
    )* };
}

macro_rules! impl_bitwise_binary_ops_assign {
    ($EndianType:ident : [$($Op:ident :: $method:ident),*$(,)?]) => { $(
        impl ::core::ops::$Op for $EndianType {
            fn $method(&mut self, rhs: Self) {
                let mut v = self.to_bits();
                v.$method(rhs.to_bits());
                *self = Self::from_bits(v);
            }
        }
    )* };
}

macro_rules! impl_bitwise_shift_ops {
    ($EndianType:ident) => {
        impl_bitwise_shift_op_with_scalars!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize]; $EndianType: Shl::shl );
        impl_bitwise_shift_op_with_scalars!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize]; $EndianType: Shr::shr );
        impl_bitwise_shift_op_assign_with_scalars!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize]; $EndianType: ShlAssign::shl_assign );
        impl_bitwise_shift_op_assign_with_scalars!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize]; $EndianType: ShrAssign::shr_assign );

    };
}

macro_rules! impl_bitwise_shift_op_with_scalars {
    ([$($Scalar:ident),*$(,)?]; $EndianType:ident : $Op:ident :: $method:ident) => { $(
        impl ::core::ops::$Op<$Scalar> for $EndianType {
            type Output = Self;
            fn $method(self, count: $Scalar) -> Self {
                Self::from_bits(self.to_bits().$method(count))
            }
        }
    )* };
}

macro_rules! impl_bitwise_shift_op_assign_with_scalars {
    ([$($Scalar:ident),*$(,)?]; $EndianType:ident : $Op:ident :: $method:ident) => { $(
        impl ::core::ops::$Op<$Scalar> for $EndianType {
            fn $method(&mut self, count: $Scalar) {
                let mut v = self.to_bits();
                v.$method(count);
                *self = Self::from_bits(v)
            }
        }
    )* };
}
