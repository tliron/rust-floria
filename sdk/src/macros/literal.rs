/// Creates a [Any](super::super::dispatch_bindings::Any) from a bare primitive expression.
#[macro_export]
macro_rules! normal (
    ( $value:expr ) => ( $crate::data::any::Any::from($value) );
);

/// Creates a [Any::NestedList](super::super::dispatch_bindings::Any::NestedList) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_list (
    () => (
        $crate::data::any::Any::NestedList(
            $crate::data::any::NestedList::new(
                $crate::data::any::List::default()
            )
        )
    );

    ( $( $value:expr ),+ $( , )? ) => (
        $crate::data::any::Any::NestedList(
            $crate::data::any::NestedList::new(
                $crate::data::any::List::new_with(
                    [ $( $crate::normal!( $value ) ),+ ]
                )
            )
        )
    );
);

/// Creates a [Any::NestedMap](super::super::dispatch_bindings::Any::NestedMap) from a sequence of key-value tuples.
#[macro_export]
macro_rules! normal_map (
    () => (
        $crate::data::any::Any::Map(
            $crate::data::any::NestedMap::new(
                $crate::data::any::Map::default()
            )
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $( , )? ) => (
        $crate::any::Any::NestedMap(
            $crate::any::NestedMap::new(
                $crate::any::Map::new_with(
                    [ $( ( $crate::normal!( $key ), $crate::normal!( $value ) ) ),+ ]
                )
            )
        )
    );
);

/// Creates a [Vec]<[Any](super::super::dispatch_bindings::Any)> from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! normal_vec (
    ( $( $value:expr ),* $( , )? ) => (
        vec![ $( $crate::normal!( $value ) ),* ]
    );
);
