# stringy

A simple to use, immutable, clone-efficient `String` replacement for Rust

## Overview

Rust is great, but it's `String` type is not optimized for many typical use
cases, but is instead optimized as a mutable string buffer. Most string use 
cases don't modify their string contents, often need to copy strings around 
as if they were cheap like integers, typically concatenate instead of modify, 
and often end up being cloned with identical contents. Additionally, 
`String` isn't able wrap a string literal without additional allocation and 
copying. Rust really needs a 3rd string type to unify usage of both literals 
and allocated strings in typical use cases. This crate creates a new string 
type that is optimized for those use cases, while retaining the usage 
simplicity of `String`.

This type is not inherently "better" than `String`, but different. It is a 
higher level type, that can at times mean higher overhead. It really 
depends on the use case.

## Features

* Optimized for immutability and cheap cloning
* Allows for multiple ownership of the same string memory contents
* It is very simple to use
* Optional serde serialization support (feature = "serde")
* Allows for simple conditional ownership scenarios (borrows can turn into 
  ownership without allocation/copying)
* Serves as a universal string type (unifying literals and allocated strings)
* Doesn't allocate for literals and short strings (64-bit: up to 30 bytes)
* Provides easy access to `&str` via dereference
* Allows for easy wrapping/unwrapping of the native `String` type, when needed
* Isn't much more expensive than `String` in non-optimal use cases

## Types

* `Stringy`
    * Wrapper type for string literals (`&'static str`), inlined strings 
      (`InlineStringy`), or an `Rc` wrapped `String` 
    * NOT `Send` or `Sync` (due to usage of `Rc`)
* `AStringy`
    * Equivalent to `Stringy` but uses `Arc` instead of `Rc` for the wrapped 
      `String`
    * Both `Send` and `Sync`
* `InlineStringy`
    * Custom inline string type holding up to 30 bytes (on 64-bit platforms)
    * Used automatically as needed by `Stringy` and `AStringy` - not typically 
      used directly

## Usage

### Hello World

```rust
use stringy::Stringy;

fn main() {
  // Literal - no copying or allocation
  let hello: Stringy = "world!".into();
  
  println!("Hello {world}");
}
```

### Conversions

```rust
fn main() {
    // From literal - no copying or allocation
    let literal: Stringy = "literal".into();
    
    // From borrowed string - Copied into inline string
    let owned = "inlined".to_string();
    let str_to_inlined = (&owned).to_stringy();

    // From borrowed String - copied into `String` wrapped in `Rc`
    let owned = "A bit too long to be inlined!!!".to_string();
    let str_to_wrapped = (&owned).to_stringy();
    
    // From String - copied into inline string (`String` storage released)
    let inlined: Stringy = "inlined".to_string().into();

    // From String - original `String` wrapped in `Rc`
    let wrapped: Stringy = "A bit too long to be inlined!!!".to_string().into();

    // From String - original `String` wrapped in `Rc`
    let force_wrapped = Stringy::wrap("not inlined".to_string());
    
    // *** If you want a Send/Sync type you need `AStringy` instead ***

    // From Stringy wrapped literal - no copying or allocation
    let literal: AStringy = literal.into();
    
    // From Stringy inlined string - no allocation
    let inlined: AStringy = inlined.into();
    
    // From Stringy `Rc` wrapped `String` - original `String` wrapped in `Arc`
    let wrapped: AStringy = wrapped.into();
    
    // *** Round trip back to `Stringy` ***
    
    // From AStringy `Arc` wrapped `String` - copy of `String` wrapped in 
    // `Rc` (since multi owned)
    let wrapped = wrapped.clone();
    let wrapped: Stringy = wrapped.into();
}
```

### Borrowing

Works just like `String`

NOTE: The only benefit to passing as a `&str` is more flexibility in what is 
passed. By passing as a `&Stringy` instead, we retain the possibility of cheap 
multi ownership (see below).

```rust
fn my_func(str: &Stringy) {
    println!("Borrowed string: {str}");
}

fn main() {
    // Literal - no copy or allocation
    let str: Stringy = "my string".into();
    my_func(&str);
}
```

### Conditional Ownership

This has always been a confusing situation in Rust, but is easy with `Stringy` 
since multi ownership is cheap.

```rust
struct MyStruct {
    s: Stringy
}

impl MyStruct {
    fn to_own_or_not_to_own(s: &Stringy) -> Self {
        let s = if s == "own_me" {
            // Since a wrapped literal, no copy or allocation
            s.clone()
        } else {
            // Wrapped literal - no copy or allocation
            "own_me".into()
        };

        Self { s }
    }
}

fn main() {
    // Wrapped literals - no copy or allocation
    let s = "borrow me".into();
    let s2 = "own me".into();

    let struct1 = MyStruct::to_own_or_not_to_own(&s);
    let struct2 = MyStruct::to_own_or_not_to_own(&s2);

    assert_eq!(s2, struct1.str);
    assert_eq!(s2, struct2.str);
}
```

## Performance Characteristics

NOTE: No benchmarking has yet been done

* Clones are cheap and never allocate
    * At minimum, they are just a copy of the enum and at max an additional 
      reference count increment
* Literals are just wrapped when used with `into()` and never copied
* Calling `into()` on a `String` will result in an inline string (if 
  short, with dynamic storage released) otherwise wrapped in `Rc`/`Arc` 
  (which will allocate)
* Using `Stringy::wrap()` or `AStringy::wrap()` is recommended when there is 
  a need to wrap and unwrap (`into_string()` or `try_into_string()`) the source 
  `String` efficiently as it ensures the original `String` is preserved and not 
  inlined.
    * This will always allocate, however, as it creates a new `Rc` or `Arc`
* `to_stringy()` and `to_a_stringy()` are meant for the on-boarding of borrowed 
  strings and always copy into either an inline string (for short strings) or 
  an `Rc`/`Arc` wrapped `String` (which will allocate)
* `try_into_string` never allocates, but will only succeed in single 
  ownership scenarios using reference counted storage (`wrap()` or non-inlined 
  `into()`)
* `into_string` works like `try_into_string`, but will fall back to 
  copying into a new `String` instead of failing
* `to_string` always copies into a new `String`
* Conversions back and forth between `AStringy` and `Stringy` using `into()` 
  are cheap when using wrapped literals or inlined strings
    * Inlined strings and wrapped literals just create a new enum wrapper
    * Reference counted wrapped strings will always require an allocation for 
      the  new `Rc` or `Arc`
        * The `String` will have to be cloned if not exclusively owned

## Negatives

There is no free lunch:

* Due to being an enum wrapper + padding/alignment it ends up being 8 bytes
  larger than `String` on 64-bit platforms (24 vs 32 bytes)
  * NOTE: The extra space is used, when possible, for inline string data
* Due to usage of `Rc` (or `Arc`) it requires two allocations instead of one
  when using the reference counted enum variant
* Due to the enum wrapper, every string operation has the overhead of an extra
  branching operation
* Need to consider single-threaded (`Stringy`) and multi-threaded (`AStringy`) 
  use cases and convert accordingly

## Open Issues / TODO

* Consider a new reference count type that inlines string contents (to avoid
  double allocation)
  * This, however, prevents efficient unwrapping of `String` without another
    variant
* Reinvent common macros like `format!` (and `aformat!`) for creating
  strings to avoid need to go back and forth to `String`

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
