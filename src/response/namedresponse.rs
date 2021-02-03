use std::borrow::Cow;

pub trait NamedResponse {
    fn name<'a>() -> Cow<'a, str>;
}

impl<T> NamedResponse for Vec<T>
    where T: NamedResponse
{
    fn name<'a>() -> Cow<'a, str> {
        let mut n = String::with_capacity(15);
        n.push_str(<T as NamedResponse>::name().as_ref());
        n.push('s');
        n.shrink_to_fit();
        n.into()
    }
}
