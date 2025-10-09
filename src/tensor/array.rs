use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

/**
 * Array of numbers.
 * */
pub struct NArray<R> {
    internal: Vec<R>,
}

impl<R> NArray<R> {
    pub fn new<F>(length: usize, init: F) -> Self
    where
        F: FnMut() -> R,
    {
        let mut internal = Vec::with_capacity(length);
        internal.resize_with(length, init);
        NArray { internal }
    }

    /**
     * Total length of the array.
     * */
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    // /**
    //  * Fills the array with provided function.
    //  * */
    // pub fn fill<F>(&mut self, init: F)
    // where
    //     F: FnMut() -> R,
    // {
    //     self.internal.fill_with(init);
    // }

    pub fn internal(&self) -> &Vec<R> {
        &self.internal
    }
}

// impl<R> IntoIterator for NArray<R> {
//     type Item = R;
//     type IntoIter = <Vec<R> as IntoIterator>::IntoIter;
// 
//     fn into_iter(self) -> Self::IntoIter {
//         self.internal.into_iter()
//     }
// }

impl<T, I> Index<I> for NArray<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.internal[index]
    }
}

impl<T, I> IndexMut<I> for NArray<T>
where
    I: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.internal[index]
    }
}
