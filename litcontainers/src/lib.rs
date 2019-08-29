#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate litcontainers_derive;

extern crate itertools;

#[macro_use] pub mod macros;
pub mod format;
#[macro_use]
pub mod iterator;
pub mod storage;
pub mod container;
pub mod slice;
pub mod ops;

pub use format::*;
pub use iterator::*;
pub use storage::*;
pub use container::*;
pub use slice::*;
pub use ops::*;

// Documentation inlines
#[doc(inline)] pub use format::{Scalar, StorageSize, Strided};
#[doc(inline)] pub use storage::{Storage, StorageMut, StorageConstructor, Ownable, DynamicColStorage, DynamicRowStorage};
#[doc(inline)] pub use slice::{Slice, SliceMut};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
