use core::marker::PhantomData;

use bytemuck::TransparentWrapper;
use type_tricks::NamedImplBase;
use type_tricks::Wrap;

mod share;

use share::named_display_impls::*;
use share::named_debug_impls::*;

use type_tricks::display::DefaultDisplay;
use type_tricks::display::{NamedDisplay, NamedDisplayProvider};
use type_tricks::debug::{NamedDebug, NamedDebugProvider};
use type_tricks::is::Is;

pub struct MultipleImplSelector<T, N1, N2>
(PhantomData<T>, PhantomData<N1>, PhantomData<N2>);

impl<T, N1, N2> NamedImplBase for MultipleImplSelector<T, N1, N2>
where
    N1: NamedDisplay,
    N1::Target: Is<Type = T>,
    N2: NamedDebug,
    N2::Target: Is<Type = T>,
{
    type Target = T;
}

impl<'a, T, N1, N2> NamedDisplayProvider for MultipleImplSelector<T, N1, N2>
where
    N1: NamedDisplay,
    N1::Target: Is<Type = T>,
    N2: NamedDebug,
    N2::Target: Is<Type = T>,
{
    type Impl = N1;
}

impl<'a, T, N1, N2> NamedDebugProvider for MultipleImplSelector<T, N1, N2>
where
    N1: NamedDisplay,
    N1::Target: Is<Type = T>,
    N2: NamedDebug,
    N2::Target: Is<Type = T>,
{
    type Impl = N2;
}

#[test]
fn test_named_impl_multiple() {
    let num = 42;

    // note: to_string() calls Display, format!("{:?}") calls Debug
    
    let a1 = Wrap::<MultipleImplSelector::<i32, NamedDisplay1, NamedDebug1>>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "NamedDisplay1");
    assert_eq!(format!("{a1:?}"), "NamedDebug1");

    let a2 = Wrap::<MultipleImplSelector::<i32, NamedDisplayProxy<i32>, NamedDebug1>>::wrap_ref(&num);
    assert_eq!(a2.to_string(), "Display Pre 42 Post");
    assert_eq!(format!("{a2:?}"), "NamedDebug1");

    let a3 = Wrap::<MultipleImplSelector::<i32, DefaultDisplay<i32>, NamedDebugProxy<i32>>>::wrap_ref(&num);
    assert_eq!(a3.to_string(), "42");
    assert_eq!(format!("{a3:?}"), "Debug Pre 42 Post");
}
