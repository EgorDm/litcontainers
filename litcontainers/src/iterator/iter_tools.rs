pub fn assign_all<'a, I, T>(iter: I, val: T)
	where I: Iterator<Item=&'a mut T>, T: 'a + Copy
{
	for v in iter { *v = val }
}

pub fn copy_all<'a, IL, IR, T>(l: IL, r: IR)
	where IL: Iterator<Item=&'a mut T>, IR: Iterator<Item=&'a T>, T: 'a + Copy
{
	for (l, r) in l.zip(r){ *l = *r }
}
 // TODO: impl for iter