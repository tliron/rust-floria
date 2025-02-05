/// Creates a [Any](crate::data::Any) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    ( $value:expr $(,)? ) => ( $crate::data::Any::from($value) );
);

/// Creates a [Any::NestedList](crate::data::Any::NestedList) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => (
        $crate::data::Any::NestedList(
            $crate::data::NestedList::new(
                $crate::data::List::default()
            )
        )
    );

    ( $( $value:expr ),+ $(,)? ) => (
        $crate::data::Any::NestedList(
            $crate::data::NestedList::new(
                $crate::data::List::from(
                    [ $( $crate::normal!( $value ) ),+ ]
                )
            )
        )
    );
);

/// Creates a [Any::NestedMap](crate::data::Any::NestedMap) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => (
        $crate::data::Any::Map(
            $crate::data::NestedMap::new(
                $crate::data::Map::default()
            )
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $(,)? ) => (
        $crate::data::Any::NestedMap(
            $crate::data::NestedMap::new(
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
