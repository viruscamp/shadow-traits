use core::marker::PhantomData;
use core::fmt::{Debug, Formatter, Result};

use type_tricks::NamedImplBase;
use type_tricks::WrapRef;

mod share;

use share::named_to_string::*;
use share::named_to_string_impls::*;
use share::named_debug_impls::*;

use type_tricks::debug::NamedDebug;
use type_tricks::is::Is;

pub struct MultipleImplSelector<T, N1, N2>
(PhantomData<T>, PhantomData<N1>, PhantomData<N2>);

impl<T, TS, D> NamedImplBase for MultipleImplSelector<T, TS, D>
where
    TS: NamedToString,
    TS::Target: Is<Type = T>,
    D: NamedDebug,
    D::Target: Is<Type = T>,
{
    type Target = T;
}

impl<'a, T, TS, D> ToString for WrapRef<'a, MultipleImplSelector<T, TS, D>>
where
    TS: NamedToString,
    TS::Target: Is<Type = T>,
    D: NamedDebug,
    D::Target: Is<Type = T>,
{
    fn to_string(&self) -> String {
        let a: &T = self.0;
        let b: &TS::Target = Is::from_ref(a);
        TS::to_string(b)
    }
}

impl<'a, T, TS, D> Debug for WrapRef<'a, MultipleImplSelector<T, TS, D>>
where
    TS: NamedToString,
    TS::Target: Is<Type = T>,
    D: NamedDebug,
    D::Target: Is<Type = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let a: &T = self.0;
        let b: &D::Target = Is::from_ref(a);
        D::fmt(b, f)
    }
}

#[test]
fn test_named_impl_multiple() {
    let num = 42;

    let a1 = WrapRef::<MultipleImplSelector::<i32, NamedToString1, NamedDebug1>>::new(&num);
    assert_eq!(a1.to_string(), "NamedToString1");
    assert_eq!(format!("{a1:?}"), "NamedDebug1");

    let a2 = WrapRef::<MultipleImplSelector::<i32, NamedToStringProxy<i32>, NamedDebug1>>::new(&num);
    assert_eq!(a2.to_string(), "Pre 42 Post");
    assert_eq!(format!("{a2:?}"), "NamedDebug1");

    let a3 = WrapRef::<MultipleImplSelector::<i32, DefaultToString<i32>, NamedDebugProxy<i32>>>::new(&num);
    assert_eq!(a3.to_string(), "42");
    assert_eq!(format!("{a3:?}"), "Debug Pre 42 Post");
}
