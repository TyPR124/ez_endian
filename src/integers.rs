macro_rules! define_integer_quads {
    ($(
        $iNative:ident : $iBe:ident, $iLe:ident;
        $uNative:ident : $uBe:ident, $uLe:ident;
    )*) => { $(
         define_be_integers!($iNative: $iBe, $uNative: $uBe);
         define_le_integers!($iNative: $iLe, $uNative: $uLe);
         define_be_le_conversions!($iBe <-> $iLe, $uBe <-> $uLe);
         define_i_u_conversions!($iBe <-> $uBe, $iLe <-> $uLe);
    )* };
}

macro_rules! define_be_integers {
    ($($NativeType:ident : $BeType:ident),*$(,)?) => {
        $(define_endian_integer!($NativeType : $BeType : to_be : from_be);)*
    };
}

macro_rules! define_le_integers {
    ($($NativeType:ident : $LeType:ident),*$(,)?) => { $(
        define_endian_integer!($NativeType : $LeType : to_le : from_le);
    )* };
}

macro_rules! define_endian_integer {
    ($NativeType:ident : $EndianType:ident : $ToEndian:ident : $FromEndian:ident) => {
        #[repr(transparent)]
        #[derive(
            Clone,
            Copy,
            Default,
            Eq,
            Hash,
            PartialEq,
        )]
        pub struct $EndianType {
            v: $NativeType,
        }
        impl $EndianType {
            const SIZE: usize = ::core::mem::size_of::<Self>();
            pub const fn from_bits(v: $NativeType) -> Self {
                Self { v }
            }
            pub const fn to_bits(self) -> $NativeType {
                self.v
            }
            // from_ne_bytes not yet stable as const fn : same with to_be_bytes
            pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
                Self::from_bits($NativeType::from_ne_bytes(b))
            }
            pub fn to_bytes(self) -> [u8; Self::SIZE] {
                self.to_bits().to_ne_bytes()
            }
            pub const fn from_native(v: $NativeType) -> Self {
                Self {
                    v: $NativeType::$ToEndian(v),
                }
            }
            pub const fn to_native(self) -> $NativeType {
                $NativeType::$FromEndian(self.v)
            }
        }
        impl ::core::convert::From<$NativeType> for $EndianType {
            fn from(v: $NativeType) -> Self {
                Self::from_native(v)
            }
        }
        impl ::core::convert::Into<$NativeType> for $EndianType {
            fn into(self) -> $NativeType {
                self.to_native()
            }
        }
        impl_formatters!(
            $EndianType: [
                Binary,
                Debug,
                Display,
                LowerHex,
                Octal,
                UpperHex
            ]
        );
        impl_bitwise_ops!($EndianType);
    };
}

macro_rules! define_be_le_conversions {
    ($($BeType:ident <-> $LeType:ident),*$(,)?) => { $(

        impl $LeType {
            pub const fn to_be(self) -> $BeType {
                $BeType::from_native(self.to_native())
            }
            pub const fn from_be(be: $BeType) -> Self {
                Self::from_native(be.to_native())
            }
        }
        impl $BeType {
            pub const fn to_le(self) -> $LeType {
                $LeType::from_native(self.to_native())
            }
            pub const fn from_le(le: $LeType) -> Self {
                Self::from_native(le.to_native())
            }
        }
        impl From<$BeType> for $LeType {
            fn from(be: $BeType) -> Self {
                Self::from_be(be)
            }
        }
        impl From<$LeType> for $BeType {
            fn from(le: $LeType) -> Self {
                Self::from_le(le)
            }
        }

    )* };
}

macro_rules! define_i_u_conversions {
    ($($iType:ident <-> $uType:ident),*$(,)?) => { $(

        impl $iType {
            pub const fn to_u(self) -> $uType {
                $uType::from_bits(self.to_bits() as _)
            }
            pub const fn from_u(u: $uType) -> Self {
                Self::from_bits(u.to_bits() as _)
            }
        }
        impl $uType {
            pub const fn to_i(self) -> $iType {
                $iType::from_bits(self.to_bits() as _)
            }
            pub const fn from_i(i: $iType) -> Self {
                Self::from_bits(i.to_bits() as _)
            }
        }
        impl From<$iType> for $uType {
            fn from(i: $iType) -> Self {
                Self::from_i(i)
            }
        }
        impl From<$uType> for $iType {
            fn from(u: $uType) -> Self {
                Self::from_u(u)
            }
        }

    )* };
}

