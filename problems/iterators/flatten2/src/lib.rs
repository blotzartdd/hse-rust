#![forbid(unsafe_code)]

pub struct FlatMap<InIter, F, OutIter> {
}

impl<InIter, F, OutIter> FlatMap<InIter, F, OutIter> {
    fn new(outer: InIter, function: F) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<InIter, F, OutIter> Iterator for FlatMap<InIter, F, OutIter>
{
    type Item = todo!();
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

pub fn flat_map<InputIterator, Mapping, OutputIterator>(
    iter: InputIterator,
    f: Mapping,
) -> FlatMap<InputIterator, Mapping, OutputIterator>
{
    // TODO: your code goes here.
    unimplemented!()
}
