#![deny(warnings)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! `const_list` provides a minimal linked-list which may be used at compile-time. For example:
//! 
//! ```rust
//! # use const_list::*;
//! const MY_LIST: ConstList<'static, i32> = ConstList::new()
//!     .push(2)
//!     .push(4)
//!     .push(8);
//! 
//! assert_eq!(8, *MY_LIST.pop().0.unwrap());
//! ```

/// A singly-linked list of items that may be created in `const` contexts.
#[derive(Copy, Clone, Debug)]
pub struct ConstList<'a, T: 'a>(Option<ConstListItem<'a, T>>);

impl<'a, T: 'a> ConstList<'a, T> {
    /// Creates a new, empty list.
    #[inline(always)]
    pub const fn new() -> Self {
        Self(None)
    }

    /// Gets a reference to the item at the provided index in this list, if any.
    #[inline(always)]
    pub const fn get(&self, index: usize) -> Option<&T> {
        if let Some(value) = &self.0 {
            if index == 0 {
                Some(&value.first)
            } else {
                value.rest.get(index - 1)
            }
        } else {
            None
        }
    }

    /// Determines the length of this list.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        if let Some(value) = &self.0 {
            value.rest.len() + 1
        } else {
            0
        }
    }

    /// Whether the list is empty.
    pub const fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Pushes a new item onto the beginning of this list,
    /// producing a new list head.
    #[inline(always)]
    pub const fn push(&'a self, value: T) -> Self {
        ConstList(Some(ConstListItem {
            first: value,
            rest: self,
        }))
    }

    /// Removes the first item (if any) from this list, and produces
    /// the rest of the list.
    #[inline(always)]
    pub const fn pop(&'a self) -> (Option<&T>, &'a Self) {
        if let Some(value) = &self.0 {
            (Some(&value.first), value.rest)
        } else {
            (None, self)
        }
    }

    /// Creates an iterator over the contents of the list.
    #[inline(always)]
    pub const fn iter(&self) -> ConstListIterator<T> {
        ConstListIterator { target: self }
    }
}

impl<'a, T> IntoIterator for &'a ConstList<'a, T> {
    type Item = &'a T;

    type IntoIter = ConstListIterator<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        ConstListIterator { target: self }
    }
}

/// A linked list node in a `ConstList`.
#[derive(Copy, Clone, Debug)]
struct ConstListItem<'a, T: 'a> {
    /// The item represented by this node.
    first: T,
    /// The rest of the list.
    rest: &'a ConstList<'a, T>,
}

/// Iterates over the contents of a `ConstList`.
pub struct ConstListIterator<'a, T> {
    /// The current list head.
    target: &'a ConstList<'a, T>,
}

impl<'a, T> Iterator for ConstListIterator<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.target.pop();
        self.target = rest;
        first
    }
}
