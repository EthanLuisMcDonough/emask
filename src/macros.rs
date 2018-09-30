macro_rules! internal_construct {
    ( $num:expr, $name:ident, { $label:ident, $( $item:ident ),* $(,)* } { $( { $list_label:ident } { $list_num:expr }; )* } { $($visibility:tt)* } ) => {
        internal_construct!(
            $num + 1,
            $name,
            { $( $item, )*  }
            { $( { $list_label } { $list_num }; )* { $label } { $num }; }
            { $( $visibility )* }
        );
    };
    ( $len:expr, $name:ident, {  } { $( { $label:ident } { $num:expr } );* $(;)* } { $($visibility:tt)* } ) => {
        #[derive(Clone, Debug, PartialEq)]
        $( $visibility )* enum $name {
            $(
                $label = (1isize << $num)
            ),*
        }
        impl $name {
            const VALUES: [Self; $len] = [ $( $name::$label, )* ];
        }

        impl From<$name> for isize {
            fn from(value: $name) -> isize {
                value as isize
            }
        }

        impl ::std::ops::Not for $name {
            type Output = MaskManager<Self>;

            fn not(self) -> Self::Output {
                !MaskManager::from(self)
            }
        }

        impl<I: Into<MaskManager<$name>>> ::std::ops::BitOr<I> for $name {
            type Output = MaskManager<Self>;

            fn bitor(self, rhs: I) -> Self::Output {
                self.into_manager() | rhs
            }
        }

        impl<I: Into<MaskManager<$name>>> ::std::ops::BitAnd<I> for $name {
            type Output = MaskManager<Self>;

            fn bitand(self, rhs: I) -> Self::Output {
                self.into_manager() & rhs
            }
        }

        impl<I: Into<MaskManager<$name>>> ::std::ops::BitXor<I> for $name {
            type Output = MaskManager<Self>;

            fn bitxor(self, rhs: I) -> Self::Output {
                self.into_manager() ^ rhs
            }
        }

        impl MaskValue for $name {
            fn owned_values() -> Vec<Self>{
                Self::VALUES.iter().cloned().collect::<>()
            }

            fn value(&self) -> isize {
                self.clone().into()
            }

            fn max_val() -> isize {
                $( (1isize << $num) + )* 0isize
            }
        }
    };
}

#[macro_export]
macro_rules! emask {
    ( $name:ident { $( $item:ident ),* $(,)* } ) => {
        internal_construct!(0, $name, { $( $item ),* } {  } {  });
    };
    ( pub $name:ident { $( $item:ident ),* $(,)* } ) => {
        internal_construct!(0, $name, { $( $item ),* } {  } { pub });
    };
}
