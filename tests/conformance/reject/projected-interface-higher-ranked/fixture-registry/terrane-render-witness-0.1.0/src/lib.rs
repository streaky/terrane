pub trait HigherRanked {
    fn visit(&self, visitor: for<'a> fn(&'a str));
}
