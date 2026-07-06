use crate::{ShadowTrait, Is, Wrap, Named};

use bytemuck::TransparentWrapper;
use core::fmt::{Display, Formatter, Result};
use core::marker::PhantomData;

pub trait DisplayProvider: ShadowTrait
where
    Self::Impl: ShadowTrait,
    Self::Target: Is<Type = <Self::Impl as ShadowTrait>::Target>,
    Named<Self::Impl>: Display
{
    type Impl;
}

impl<N> DisplayProvider for N
where
    N: ShadowTrait,
    Named<N>: Display
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> Display for Wrap<NP, ImplDeref>
where
    NP: DisplayProvider,
    NP::Impl: ShadowTrait,
    NP::Target: Is<Type = <NP::Impl as ShadowTrait>::Target>,
    Named<NP::Impl>: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let a = &self.0;
        let b = <NP::Target as Is>::to_ref_right(a);
        let c: &Named<NP::Impl> = Named::<NP::Impl>::wrap_ref(b);
        Named::<NP::Impl>::fmt(c, f)
    }
}

pub struct DefaultDisplay<T: Display + ?Sized>(PhantomData<T>);
impl<T: Display + ?Sized> ShadowTrait for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display + ?Sized> Display for Named<DefaultDisplay<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(&self.0, f)
    }
}
