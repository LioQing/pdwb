use std::marker::PhantomData;

// Show function
pub trait Show {
    fn show() -> String;
}

// Add function
pub trait Add {
    type Return<X, Y>;
}

// Bool
pub struct Bool<const V: bool>;

impl<const V: bool> Show for Bool<V> {
    fn show() -> String {
        format!("{V}")
    }
}

// I32
pub struct I32<const V: i32>;

impl<const V: i32> Show for I32<V> {
    fn show() -> String {
        format!("{V}")
    }
}

pub trait AddI32I32 {
    type Return;
}

impl<const X: i32, const Y: i32> AddI32I32 for (I32<X>, I32<Y>) {
    type Return = I32<{X + Y}>;
}

impl<const X: i32, const Y: i32> Add for AddI32I32 {
    type Return<I32
}

// List
pub struct List<V>(PhantomData<V>);
pub struct Nil;
pub struct Cons<X, XS>(PhantomData<(X, XS)>);

impl<V: Show> Show for List<V> {
    fn show() -> String {
        format!("[{}]", V::show())
    }
}

impl Show for Nil {
    fn show() -> String {
        "".to_string()
    }
}

impl<X: Show, XS: Show> Show for Cons<X, XS> {
    fn show() -> String {
        format!("{}, {}", X::show(), XS::show())
    }
}