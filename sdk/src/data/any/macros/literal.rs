/// Creates a [Any](crate::data::Any) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    ( $value:expr $(,)? ) => ( $crate::data::Any::from($value) );
);

/// Creates a [Any::AnyList](crate::data::Any::AnyList) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => (
        $crate::data::Any::AnyList(
            $crate::data::AnyList::new(
                $crate::data::List::default()
            )
        )
    );

    ( $( $value:expr ),+ $(,)? ) => (
        $crate::data::Any::AnyList(
            $crate::data::AnyList::new(
                $crate::data::List::from(
                    [ $( $crate::normal!( $value ) ),+ ]
                )
            )
        )
    );
);

/// Creates a [Any::AnyMap](crate::data::Any::AnyMap) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => (
        $crate::data::Any::Map(
            $crate::data::AnyMap::new(
                $crate::data::Map::default()
            )
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $(,)? ) => (
        $crate::data::Any::AnyMap(
            $crate::data::AnyMap::new(
                $crate::data::Map::from(
                    ::std::collections::BTreeMap::from(
                        [ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]
                    )
                )
            )
        )
    );
);

/// Creates a [Vec]<[Any](crate::data::Any)> from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_vec (
    ( $( $value:expr ),* $(,)? ) => (
        vec![ $( $crate::normal!( $value ) ),* ]
    );
);
