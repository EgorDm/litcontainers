pub mod parallel;
pub mod iter_tools;
pub mod axis;
#[macro_use]
pub mod full;
pub mod slice;


#[doc(inline)] pub use parallel::*;
#[doc(inline)] pub use axis::*;
#[doc(inline)] pub use full::*;
#[doc(inline)] pub use slice::*;