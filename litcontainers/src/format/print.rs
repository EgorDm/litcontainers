use std::fmt;

pub struct Fmt<F>(pub F) where F: Fn(&mut fmt::Formatter) -> Result<(), fmt::Error>;

impl<F> fmt::Display for Fmt<F>
	where F: Fn(&mut fmt::Formatter) -> Result<(), fmt::Error>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		(self.0)(f)
	}
}
