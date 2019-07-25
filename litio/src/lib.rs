#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod format;
pub mod error;
pub mod serializers;
pub mod file;

pub use format::*;
pub use serializers::*;
pub use file::*;
pub use error::*;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
