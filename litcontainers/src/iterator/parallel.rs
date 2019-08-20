use rayon::iter::plumbing::{UnindexedConsumer, Consumer, ProducerCallback, bridge, Producer};
use rayon::iter::{ParallelIterator, IndexedParallelIterator};

pub trait SplittableIterator: Sized + Iterator + ExactSizeIterator + DoubleEndedIterator {
	fn split_at(self, pos: usize) -> (Self, Self);
}

#[derive(Copy, Clone, Debug)]
pub struct Parallel<I> {
	iter: I,
}

impl<I> Parallel<I> {
	pub fn new(iter: I) -> Self { Self { iter } }
}

#[derive(Copy, Clone, Debug)]
struct ParallelProducer<I>(I);

impl<'a, I> ParallelIterator for Parallel<I>
	where I: SplittableIterator + Send, <I as Iterator>::Item: Send
{
	type Item = <I as Iterator>::Item;

	fn drive_unindexed<CO>(self, consumer: CO) -> <CO as Consumer<Self::Item>>::Result where
		CO: UnindexedConsumer<Self::Item> {
		bridge(self, consumer)
	}
}

impl<'a, I> IndexedParallelIterator for Parallel<I>
	where I: SplittableIterator + Send, <I as Iterator>::Item: Send
{
	fn len(&self) -> usize { self.iter.len() }

	fn drive<CO: Consumer<Self::Item>>(self, consumer: CO) -> <CO as Consumer<Self::Item>>::Result {
		bridge(self, consumer)
	}

	fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> <CB as ProducerCallback<Self::Item>>::Output {
		callback.callback(ParallelProducer(self.iter))
	}
}

impl<'a, I> Producer for ParallelProducer<I>
	where I: SplittableIterator + Send, <I as Iterator>::Item: Send
{
	type Item = <Self::IntoIter as Iterator>::Item;
	type IntoIter = I;

	fn into_iter(self) -> Self::IntoIter { self.0 }

	fn split_at(self, i: usize) -> (Self, Self) {
		let (a, b) = self.0.split_at(i);
		(ParallelProducer(a), ParallelProducer(b))
	}
}