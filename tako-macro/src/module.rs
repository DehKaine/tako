#[macro_export]
macro_rules! pub_mod {
    [ $( $name:ident $(,)? )+ ] => {
        $(
            pub mod $name;
        )+
    };
}

#[macro_export]
macro_rules! flat_mod {
    [ $( $name:ident $(,)? )+ ] => {
        $(
            mod $name;
            pub use $name::*;
        )+
    };
}
