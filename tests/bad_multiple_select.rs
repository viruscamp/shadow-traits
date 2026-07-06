use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use bytemuck::TransparentWrapper;
use shadow_traits::wrap::Wrap;
use shadow_traits::{Named, ShadowTrait};
use shadow_traits::is::Is;

mod share;

use share::to_string::*;
use share::named_to_string_impls::*;
use share::named_debug_impls::*;


pub struct MultipleImplSelector<T, N1, N2>
(PhantomData<T>, PhantomData<N1>, PhantomData<N2>);

impl<T, TS, D> ShadowTrait for MultipleImplSelector<T, TS, D>
where
    TS: ShadowTrait,
    Named<TS>: ToString,
    TS::Target: Is<Type = T>,
    D: ShadowTrait,
    Named<D>: Debug,
    D::Target: Is<Type = T>,
{
    type Target = T;
}

impl<T, TS, D> ToString for Wrap<MultipleImplSelector<T, TS, D>>
where
    TS: ShadowTrait,
    Named<TS>: ToString,
    TS::Target: Is<Type = T>,
    D: ShadowTrait,
    Named<D>: Debug,
    D::Target: Is<Type = T>,
{
    fn to_string(&self) -> String {
        let a: &T = &self.0;
        let b = <TS::Target as Is>::to_ref_left(a);
        let c = Named::<TS>::wrap_ref(b);
        Named::to_string(c)
    }
}

impl<T, TS, D> Debug for Wrap<MultipleImplSelector<T, TS, D>>
where
    TS: ShadowTrait,
    Named<TS>: ToString,
    TS::Target: Is<Type = T>,
    D: ShadowTrait,
    Named<D>: Debug,
    D::Target: Is<Type = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let a: &T = &self.0;
        let b = <D::Target as Is>::to_ref_left(a);
        let c = Named::<D>::wrap_ref(b);
        Named::<D>::fmt(c, f)
    }
}

#[test]
fn test_named_impl_multiple() {
    let num = 42;

    let a1 = Wrap::<MultipleImplSelector::<i32, ToStringImpl1, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "ToStringImpl1");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");

    let a2 = Wrap::<MultipleImplSelector::<i32, ToStringImplProxy<i32>, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a2.to_string(), "Pre 42 Post");
    assert_eq!(format!("{a2:?}"), "DebugImpl1");

    let a3 = Wrap::<MultipleImplSelector::<i32, DefaultToString<i32>, DebugImplProxy<i32>>>::wrap_ref(&num);
    assert_eq!(a3.to_string(), "42");
    assert_eq!(format!("{a3:?}"), "Debug Pre 42 Post");
}
