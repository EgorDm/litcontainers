#[macro_use]
extern crate derive_new;

extern crate itertools;

#[macro_use] pub mod macros;
pub mod format;
pub mod storage;
pub mod iterator;
pub mod container;
pub mod slice;
pub mod ops;
pub mod functions;
pub mod wrapper;

pub use format::*;
pub use storage::*;
pub use iterator::*;
pub use container::*;
pub use slice::*;
pub use ops::*;
pub use functions::*;
pub use wrapper::*;

// Documentation inlines
#[doc(inline)] pub use format::{Scalar};
#[doc(inline)] pub use storage::{Storage, StorageMut, StorageConstructor, Ownable, SizedStorage, DynamicColStorage, DynamicRowStorage};
#[doc(inline)] pub use container::{Container, ContainerCM, ContainerRM};
#[doc(inline)] pub use slice::{Slice, SliceMut};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
