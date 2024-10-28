pub struct Cache<T>
  where T: Fn(u32) -> u32
{
  query: T,
  value: Option<u32>,
}

impl<T> Cache<T>
  where T: Fn(u32) -> u32
{
  pub fn new (query: T) -> Cache<T> {
    Cache {
      query,
      value: None
    }
  }

  pub fn value (&mut self, args: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.query)(args);
        self.value = Some(v);
        v
      }
    }
  }
}