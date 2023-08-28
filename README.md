# const_list

[![Crates.io](https://img.shields.io/crates/v/const_list.svg)](https://crates.io/crates/const_list)
[![Docs.rs](https://docs.rs/const_list/badge.svg)](https://docs.rs/const_list)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`const_list` provides a minimal linked-list which may be used at compile-time. For example:

```rust
const MY_LIST: ConstList<'static, i32> = ConstList::new()
    .push(2)
    .push(4)
    .push(8);

assert_eq!(8, MY_LIST.pop().0.unwrap());
```