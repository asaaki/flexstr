use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::str;
use core::str::Utf8Error;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::Str;
use crate::traits::private;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

/// Empty string constant
pub const EMPTY: &str = "";

impl Str for str {
    type StringType = String;
    type HeapType = [u8];
    type ConvertError = Utf8Error;

    #[inline]
    fn from_inline_data(bytes: &[u8]) -> &Self {
        // SAFETY: This will always be previously vetted to ensure it is proper UTF8
        unsafe { core::str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn from_heap_data(bytes: &Self::HeapType) -> &Self {
        Self::from_inline_data(bytes)
    }

    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        str::from_utf8(bytes)
    }

    #[inline(always)]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(EMPTY)
        } else {
            None
        }
    }

    #[inline(always)]
    fn length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_heap_type(&self) -> &Self::HeapType {
        self.as_bytes()
    }

    #[inline(always)]
    fn as_inline_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

define_flex_types!("Str", str, [u8]);

macro_rules! impl_body {
    () => {
        /// An empty ("") static constant string
        pub const EMPTY: Self = Self::from_static(EMPTY);

        /// Creates a wrapped static string literal. This function is equivalent to using the macro and
        /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
        /// ```
        /// use flexstr::{FlexStrCore, LocalStr};
        ///
        /// const S: LocalStr = LocalStr::from_static("test");
        /// assert!(S.is_static());
        /// ```
        #[inline(always)]
        pub const fn from_static(s: &'static str) -> Self {
            Self(FlexStrInner::from_static(s))
        }

        /// Tries to create a wrapped static string literal from a raw byte slice. If it is successful, a
        /// [FlexStr] will be created using static wrapped storage. If unsuccessful (because encoding is
        /// incorrect) a [Utf8Error] is returned.
        /// ```
        /// use flexstr::{FlexStrCore, LocalStr};
        ///
        /// const S: &[u8] = b"test";
        /// let s = LocalStr::try_from_static_raw(S).unwrap();
        /// assert!(s.is_static());
        /// ```
        #[inline]
        pub fn try_from_static_raw(s: &'static [u8]) -> Result<Self, Utf8Error> {
            // `from_utf8` still const fn unstable - use trait for now
            let s = str::try_from_raw_data(s)?;
            Ok(Self(FlexStrInner::from_static(s)))
        }
    };
}

// *** FlexStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStr<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, str> for FlexStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<str>,
{
}

// *** FlexStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, str> for FlexStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<str>,
{
}
