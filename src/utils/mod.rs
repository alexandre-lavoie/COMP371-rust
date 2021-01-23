pub fn coerce_mut<S: ?Sized>(r: &mut Box<S>) -> &mut S {
    r
}

pub fn coerce<S: ?Sized>(r: &Box<S>) -> &S {
    r
}