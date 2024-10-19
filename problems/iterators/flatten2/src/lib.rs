#![forbid(unsafe_code)]

pub struct FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: Fn(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    in_iter: InIter,
    function: F,
    out_iter: Option<OutIter::IntoIter>,
}

impl<InIter, F, OutIter> FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: Fn(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    fn new(outer: InIter, function: F) -> Self {
        FlatMap {
            in_iter: outer,
            function,
            out_iter: None,
        }
    }
}

impl<InIter, F, OutIter> Iterator for FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: Fn(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    type Item = OutIter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.out_iter.is_some() {
            if let Some(value) = self.out_iter.as_mut().unwrap().next() {
                return Some(value);
            }
        }

        if let Some(next_iter) = self.in_iter.next() {
            self.out_iter = Some((self.function)(next_iter).into_iter());
            return self.out_iter.as_mut().unwrap().next();
        }

        None
    }
}

pub fn flat_map<InputIterator, Mapping, OutputIterator>(
    iter: InputIterator,
    f: Mapping,
) -> FlatMap<InputIterator, Mapping, OutputIterator>
where
    InputIterator: Iterator,
    Mapping: Fn(InputIterator::Item) -> OutputIterator,
    OutputIterator: IntoIterator,
{
    FlatMap::new(iter, f)
}
