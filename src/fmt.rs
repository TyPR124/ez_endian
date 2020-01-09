macro_rules! impl_formatters {
    ($EndianType:ident : [$($Fmt:ident),*$(,)?]) => { $(

        impl ::core::fmt::$Fmt for $EndianType {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::$Fmt::fmt(&self.to_native(), fmt)
            }
        }

    )* };
}

// macro_rules! impl_debug {
//     () => {

//     };
// }
