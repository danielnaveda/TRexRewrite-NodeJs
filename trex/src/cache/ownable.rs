use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

pub trait Ownable<T> {
    fn into_owned(self) -> T;
}

impl<T> Ownable<T> for T {
    fn into_owned(self) -> T { self }
}

impl<'a, T: Clone> Ownable<T> for &'a T {
    fn into_owned(self) -> T { self.clone() }
}

impl<'a, T: Clone> Ownable<T> for &'a mut T {
    fn into_owned(self) -> T { self.clone() }
}

impl<T> Ownable<T> for Box<T> {
    fn into_owned(self) -> T { *self }
}

impl<T: Clone> Ownable<T> for Rc<T> {
    fn into_owned(self) -> T { Self::try_unwrap(self).unwrap_or_else(|it| (*it).clone()) }
}

impl<T: Clone> Ownable<T> for Arc<T> {
    fn into_owned(self) -> T { Self::try_unwrap(self).unwrap_or_else(|it| (*it).clone()) }
}

impl<'a, T: Clone> Ownable<T> for Cow<'a, T> {
    fn into_owned(self) -> T { Cow::into_owned(self) }
}

impl<'a> Ownable<String> for Cow<'a, str> {
    fn into_owned(self) -> String { Cow::into_owned(self) }
}

impl<'a, T: Clone> Ownable<Vec<T>> for Cow<'a, [T]> {
    fn into_owned(self) -> Vec<T> { Cow::into_owned(self) }
}
