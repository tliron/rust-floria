// TODO!

/// Traverse a value by calling [Any::get](crate::data::Any::get) recursively.
///
/// The first argument is the starting [any::Any](crate::data::Any). The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-collection or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Any](crate::data::Any) or anything that implements
/// [Into]<[Any](crate::data::Any)>, which includes all the supported primitive types.
#[macro_export]
macro_rules! traverse(
    ( $value:expr $(,)? ) => ( ::std::option::Option::<&$crate::functions::Any>::Some(&$value) );

    ( $value:expr, $key:expr $(,)? ) => ( $value.into_get($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $(,)? ) => (
        match $crate::traverse!( $value, $key ) {
            ::std::option::Option::Some(value) => $crate::traverse!( value $( , $next_key )+ ),
            ::std::option::Option::None => ::std::option::Option::None,
        }
    );
);

/// Traverse a value by calling [Any::get_mut](crate::data::Any::get_mut) recursively.
///
/// The first argument is the starting [Any](crate::data::Any). The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-collection or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Any](crate::data::Any) or anything that implements
/// [Into]<[Any](crate::data::Any)>, which includes all the supported primitive types.
#[macro_export]
macro_rules! traverse_mut(
    ( $value:expr $(,)? ) => ( ::std::option::Option::<&mut $crate::functions::Any>::Some($value) );

    ( $value:expr, $key:expr $(,)? ) => ( $value.into_get_mut($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $(,)? ) => (
        match $crate::traverse_mut!( $value, $key ) {
            ::std::option::Option::Some(value) => $crate::traverse_mut!( value $( , $next_key )+ ),
            ::std::option::Option::None => ::std::option::Option::None,
        }
    );
);
