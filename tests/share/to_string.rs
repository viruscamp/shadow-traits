use core::marker::PhantomData;
use bytemuck::TransparentWrapper;
use shadow_traits::{Named, ShadowTrait, Wrap};

pub struct DefaultToString<T: ToString>(PhantomData<T>);
impl<T: ToString> ShadowTrait for DefaultToString<T> {
    type Target = T;
}
impl<T: ToString> ToString for Named<DefaultToString<T>> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct ToStringSelector<N>(PhantomData<N>)
where N: ShadowTrait, Named<N>: ToString;
impl<N> ShadowTrait for ToStringSelector<N>
where N: ShadowTrait, Named<N>: ToString
{
    type Target = N::Target;
}
// Because we cannot write `impl<N: ShadowToString> ToString for Wrap<N>`
impl<N> ToString for Wrap<ToStringSelector<N>>
where N: ShadowTrait, Named<N>: ToString
{
    fn to_string(&self) -> String {
        Named::to_string(Named::wrap_ref(&self.0))
    }
}
