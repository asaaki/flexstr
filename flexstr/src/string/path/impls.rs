// +-------------------------------------------------------------------------------------------------+
// | WARNING: This file has been auto-generated using FlexGen (https://github.com/nu11ptr/flexgen).  |
// | Any manual modifications to this file will be overwritten the next time this file is generated. |
// +-------------------------------------------------------------------------------------------------+

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::ops::Deref;
use std::path::Path;

use crate::custom::{PTR_SIZED_PAD, STRING_SIZED_INLINE};
use crate::inner::FlexStrInner;
use crate::storage::Storage;
use crate::traits::private::FlexStrCoreInner;
use crate::traits::{private, FlexStrCore};

// *** String Type Struct ***

/// A flexible string type that transparently wraps a string literal, inline string, or an
/// [`Rc<Path>`](std::rc::Rc)
#[repr(transparent)]
pub struct FlexPath<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>(
    pub(crate) FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, Path>,
);

// ###  Clone ###

impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Clone
    for FlexPath<'str, SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Storage<Path> + Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// ### Deref ###

impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Deref
    for FlexPath<'str, SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Storage<Path>,
{
    type Target = Path;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_str_type()
    }
}

// ### FlexStrCoreInner ###

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    private::FlexStrCoreInner<'str, SIZE, BPAD, HPAD, HEAP, Path>
    for FlexPath<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<Path>,
{
    type This = Self;
    #[inline(always)]
    fn wrap(inner: FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, Path>) -> Self::This {
        Self(inner)
    }
    #[inline(always)]
    fn inner(&self) -> &FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, Path> {
        &self.0
    }
}

// ### FlexStrCore ###

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, Path> for FlexPath<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<Path>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &Path {
        self.inner().as_str_type()
    }
}

// ### Const Fn Init Functions ###

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexPath<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    /// ```
    /// use std::path::Path;
    /// use flexstr::FlexStrCore;
    /// use flexstr::path::LocalPath;
    ///
    /// let s = LocalPath::from_static(Path::new("test"));
    /// assert!(s.is_static());
    /// ```
    #[inline(always)]
    pub const fn from_static(s: &'static Path) -> Self {
        Self(FlexStrInner::from_static(s))
    }
}

// ### Regular Init Functions ###

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexPath<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<Path>,
{
    /// Creates a new string from a `Path` reference. If the string is empty, an empty static string
    /// is returned. If at or under the inline length limit, an inline string will be returned.
    /// Otherwise, a heap based string will be allocated and returned. This is typically used to
    /// create strings from a non-static borrowed `Path` where you don't have ownership.
    ///
    /// # NOTE
    /// Don't use this for string literals or other `'static` strings. Use `from_static` or
    /// the macros instead. Those simply wrap instead of copy and/or allocate.
    /// ```
    /// use std::path::Path;
    /// use flexstr::FlexStrCore;
    /// use flexstr::path::LocalPath;
    ///
    /// let s = LocalPath::from_ref(Path::new(""));
    /// assert!(s.is_static());
    ///
    /// let s = LocalPath::from_ref(Path::new("inline"));
    /// assert!(s.is_inline());
    ///
    /// let s = LocalPath::from_ref(Path::new("This is too long to inline!"));
    /// assert!(s.is_heap());
    /// ```
    #[inline(always)]
    pub fn from_ref(s: impl AsRef<Path>) -> Self {
        Self(FlexStrInner::from_ref(s))
    }

    /// Attempts to create an inlined string. Returns a new inline string on success or the original
    /// source string if it will not fit.
    ///
    /// # Note
    /// Since the to/into/[from_ref](FlexPath::from_ref) functions will automatically inline when
    /// possible, this function is really only for special use cases.
    /// ```
    /// use std::path::Path;
    /// use flexstr::FlexStrCore;
    /// use flexstr::path::LocalPath;
    ///
    /// let s = LocalPath::try_inline(Path::new("inline")).unwrap();
    /// assert!(s.is_inline());
    /// ```
    #[inline(always)]
    pub fn try_inline<S: AsRef<Path>>(s: S) -> Result<Self, S> {
        FlexStrInner::try_inline(s).map(Self)
    }

    /// Force the creation of a heap allocated string. Unlike to/into/[from_ref](FlexPath::from_ref)
    /// functions, this will not attempt to inline first even if the string is a candidate for inlining.
    ///
    /// # Note
    /// Using this is only recommended when using the associated [to_heap](FlexPath::to_heap)
    /// and [try_to_heap](FlexPath::try_to_heap) functions.
    /// ```
    /// use std::path::Path;
    /// use flexstr::FlexStrCore;
    /// use flexstr::path::LocalPath;
    ///
    /// let s = LocalPath::from_ref_heap(Path::new("This is too long to inline!"));
    /// assert!(s.is_heap());
    /// ```
    #[inline(always)]
    pub fn from_ref_heap(s: impl AsRef<Path>) -> Self {
        Self(FlexStrInner::from_ref_heap(s))
    }
}

// *** Type Aliases ***

/// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type.
///
/// It is three machine words in size (3x usize) and can hold 22 bytes of inline string data on 64-bit platforms.
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
///
/// # Note 2
/// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
/// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
/// creation.
pub type FlexPath3USize<'str, HEAP> =
    FlexPath<'str, STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;

/// A flexible string type that transparently wraps a string literal, inline string, or
/// a/an [`Rc<Path>`](alloc::rc::Rc)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
pub type LocalPath = FlexPath3USize<'static, Rc<Path>>;

/// A flexible string type that transparently wraps a string literal, inline string,
/// a/an [`Rc<Path>`](alloc::rc::Rc), or borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
pub type LocalPathRef<'str> = FlexPath3USize<'str, Rc<Path>>;

/// A flexible string type that transparently wraps a string literal, inline string, or
/// a/an [`Arc<Path>`](alloc::sync::Arc)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
pub type SharedPath = FlexPath3USize<'static, Arc<Path>>;

/// A flexible string type that transparently wraps a string literal, inline string,
/// a/an [`Arc<Path>`](alloc::sync::Arc), or borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
pub type SharedPathRef<'str> = FlexPath3USize<'str, Arc<Path>>;

/// A flexible string type that transparently wraps a string literal, inline string, or
/// a/an [`Box<Path>`](alloc::boxed::Box)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
///
/// # Note 2
/// This type is included for convenience for those who need wrapped [`Box<Path>`](alloc::boxed::Box)
/// support. Those who do not have this special use case are encouraged to use `Local` or `Shared`
/// variants for much better clone performance (without copy or additional allocation)
pub type BoxedPath = FlexPath3USize<'static, Box<Path>>;

/// A flexible string type that transparently wraps a string literal, inline string,
/// a/an [`Box<Path>`](alloc::boxed::Box), or borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexPath]
///
/// # Note 2
/// This type is included for convenience for those who need wrapped [`Box<Path>`](alloc::boxed::Box)
/// support. Those who do not have this special use case are encouraged to use `Local` or `Shared`
/// variants for much better clone performance (without copy or additional allocation)
pub type BoxedPathRef<'str> = FlexPath3USize<'str, Box<Path>>;
