use std::fmt;

pub struct LazyFmt<F>(pub F)
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result;

impl<F> fmt::Display for LazyFmt<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

pub fn iter_fmt<T, F, I>(iterable: T, fmt_fn: F) -> LazyFmt<impl Fn(&mut fmt::Formatter<'_>) -> fmt::Result>
where
    T: IntoIterator<Item = I> + Clone,
    F: Fn(&mut fmt::Formatter<'_>, I) -> fmt::Result,
{
    LazyFmt(move |f| {
        for item in iterable.clone() {
            if let Err(err) = fmt_fn(f, item) {
                return Err(err);
            }
        }
        Ok(())
    })
}
