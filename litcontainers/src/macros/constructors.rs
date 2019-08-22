#[macro_export]
macro_rules! rvec_value {
	($d: expr; $v: expr) => { RowVec::from_value(Size::new(U1, $d), $v) };
	($d: expr; $v: expr; $T: ty) => {  <RowVec<$T, _>>::from_value(Size::new(U1, $d), $v) }
}

#[macro_export]
macro_rules! rvec_zeros {
	($d: expr) => { RowVec::zeros(Size::new(U1, $d)) };
	($d: expr; $T: ty) => {  <RowVec<$T, _>>::zeros(Size::new(U1, $d)) }
}

#[macro_export]
macro_rules! rvec {
	($d: expr, $v: expr) => { RowVec::from_vec(Size::new(U1, $d), $v) };
	($d: expr, $v: expr; $T: ty) => { <RowVec<$T, _>>::from_vec(Size::new(U1, $d), $v) };
	($v: expr) => { RowVec::from_vec(Size::new(U1, Dynamic::new($v.len())), $v) };
}

#[macro_export]
macro_rules! cvec_zeros {
	($d: expr) => { ColVec::zeros(Size::new($d, U1)) };
	($d: expr; $T: ty) => {  <ColVec<$T, _>>::zeros(Size::new($d, U1)) }
}

#[macro_export]
macro_rules! cvec_value {
	($d: expr; $v: expr) => { ColVec::from_value(Size::new($d, U1), $v) };
	($d: expr; $v: expr; $T: ty) => {  <ColVec<$T, _>>::from_value(Size::new($d, U1), $v) }
}

#[macro_export]
macro_rules! cvec {
	($d: expr, $v: expr) => { ColVec::from_vec(Size::new($d, U1), $v) };
	($d: expr, $v: expr; $T: ty) => { <ColVec<$T, _>>::from_vec(Size::new($d, U1), $v) };
	($v: expr) => { cvec!(Dynamic::new($v.len()), $v) };
}

#[macro_export]
macro_rules! D {
	($d: expr) => { Dynamic::new($d) };
}
