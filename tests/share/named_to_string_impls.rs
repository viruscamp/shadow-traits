use core::marker::PhantomData;

use shadow_traits::{Named, ShadowTrait};

pub struct ToStringImpl1;
impl ShadowTrait for ToStringImpl1 {
    type Target = i32;
}
impl ToString for Named<ToStringImpl1> {
    fn to_string(&self) -> String {
        "ToStringImpl1".to_string()
    }
}

pub struct ToStringImplProxy<T: ToString>(PhantomData<T>);
impl<T: ToString> ShadowTrait for ToStringImplProxy<T> {
    type Target = T;
}
impl<T: ToString> ToString for Named<ToStringImplProxy<T>> {
    fn to_string(&self) -> String {
        let a = self.0.to_string();
        format!("Pre {a} Post")
    }
}
