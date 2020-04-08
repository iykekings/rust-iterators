// https://doc.rust-lang.org/std/iter/index.html#implementing-iterator

pub struct Counter {
  count: usize
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
      Counter {
        count: 0
      }
    }
}

impl Iterator for Counter {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;

    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn count_starts_at_zero() {
    let count = Counter::new();
    assert_eq!(0, count.count);
  }

  #[test]
  fn counts_from_1_to_4() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
  }

  #[test]
  fn count_works_with_iterator_adapters() {
     let counter = Counter::new();
      counter.enumerate().for_each(|(i, x)| assert_eq!(x, i + 1));
  }
}