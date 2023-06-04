#[cfg(test)]
pub trait ModifyBuilder {
    fn modify<D, F>(self, data: Option<D>, f: F) -> Self
    where
        F: Fn(Self, D) -> Self,
        Self: Sized,
    {
        if let Some(data) = data {
            f(self, data)
        } else {
            self
        }
    }
}
