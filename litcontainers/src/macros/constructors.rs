
#[macro_export]
macro_rules! rvec_zeros {
	($d: expr) => { RowVec::zeros(U1, $d) };
	($d: expr; $T: ty) => {  <RowVec<$T, _>>::zeros(U1, $d) }
}

#[macro_export]
macro_rules! rvec {
	($d: expr, $v: expr) => { RowVec::from_vec(U1, $d, $v) };
	($d: expr, $v: expr; $T: ty) => { <RowVec<$T, _>>::from_vec(U1, $d, $v) };
	($v: expr) => { RowVec::from_vec(U1, Dynamic::new($v.len()), $v) };
}

#[macro_export]
macro_rules! cvec_zeros {
	($d: expr) => { ColVec::zeros($d, U1) };
	($d: expr; $T: ty) => {  <ColVec<$T, _>>::zeros($d, U1) }
}

#[macro_export]
macro_rules! cvec {
	($d: expr, $v: expr) => { ColVec::from_vec($d, U1, $v) };
	($d: expr, $v: expr; $T: ty) => { <ColVec<$T, _>>::from_vec($d, U1, $v) };
	($v: expr) => { ColVec::from_vec(Dynamic::new($v.len()), U1, $v) };
}

#[macro_export]
macro_rules! D {
	($d: expr) => { Dynamic::new($d) };
}