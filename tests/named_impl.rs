use std::marker::PhantomData;

use type_tricks::{NamedImplBase, Wrap};

pub trait NamedToString: NamedImplBase {
    fn to_string(this: &Self::Target) -> String;
}

struct ToStringImpl<N: NamedToString>(PhantomData<N>);
impl<N> NamedImplBase for ToStringImpl<N>
where
    N: NamedToString,
{
    type Target = N::Target;
}

impl<N> ToString for Wrap<ToStringImpl<N>>
    where N: NamedToString,
          N::Target: Sized
{
    fn to_string(&self) -> String {
        N::to_string(&self.value)
    }
}

struct DefaultToString;
impl NamedImplBase for DefaultToString {
    type Target = u32;
}
impl NamedToString for DefaultToString {
    fn to_string(this: &Self::Target) -> String {
        this.to_string()
    }
}

struct NamedToString1;
impl NamedImplBase for NamedToString1 {
    type Target = u32;
}
impl NamedToString for NamedToString1 {
    fn to_string(this: &Self::Target) -> String {
        "NamedToString1".to_string()
    }
}

struct NamedToStringProxy<T: ToString>(PhantomData<T>);
impl<T: ToString> NamedImplBase for NamedToStringProxy<T> {
    type Target = T;
}
impl<T: ToString> NamedToString for NamedToStringProxy<T> {
    fn to_string(this: &Self::Target) -> String {
        let a = this.to_string();
        format!("Pre {a} Post")
    }
}

#[test]
fn test_named_to_string() {
    let wrap1 = Wrap::<ToStringImpl<NamedToString1>>::from(123);
    assert_eq!(wrap1.to_string(), "NamedToString1");

    let wrap2 = Wrap::<ToStringImpl<NamedToStringProxy<i32>>>::from(456);
    assert_eq!(wrap2.to_string(), "Pre 456 Post");
}
