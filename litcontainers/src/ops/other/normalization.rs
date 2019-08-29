use crate::*;
use num_traits::pow::Pow;
use num_traits::{Float, Signed};
use num_complex::Complex;

pub fn norm_p1<T, S>(s: &S) -> T
	where T: Scalar + Signed, S: Storage<T>
{
	s.as_iter().fold(T::default(), |acc, x| acc + (*x).abs())
}

pub fn norm_p1_c<T, S>(s: &S) -> Complex<T>
	where T: Scalar + Float, S: Storage<Complex<T>>
{
	let ret = s.as_iter().fold(T::default(), |acc, x| acc + x.re.abs() + x.im.abs());
	Complex::new(ret, T::default())
}

pub fn norm_p2<T, S>(s: &S) -> T
	where T: Scalar + Float, S: Storage<T>
{
	// TODO: seen others do loopunrollings here by iterating 2 at a time
	s.as_iter().fold(T::default(), |acc, x| acc + *x * *x).sqrt()
}

pub fn norm_p2_c<T, S>(s: &S) -> Complex<T>
	where T: Scalar + Float, S: Storage<Complex<T>>
{
	let ret = s.as_iter().fold(T::default(), |acc, x| acc + x.re * x.re + x.im * x.im).sqrt();
	Complex::new(ret, T::default())
}

pub fn norm_p<T, S>(s: &S, k: i32) -> T
	where T: Scalar + Pow<i32, Output=T>, S: Storage<T>
{
	s.as_iter().fold(T::default(), |acc, x| acc + (*x).pow(k)).pow(-k)
}

pub fn norm_p_c<T, S>(s: &S, k: i32) -> Complex<T>
	where T: Scalar + Float + Pow<i32, Output=T>, S: Storage<Complex<T>>
{
	let ret = s.as_iter().fold(T::default(), |acc, x| acc + x.re.pow(k) + x.im.pow(k)).pow(-k);
	Complex::new(ret, T::default())
}
