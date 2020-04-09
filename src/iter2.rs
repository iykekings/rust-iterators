#[cfg(test)]
mod test {
  #[test]
  fn iter_max_by_key() {
    // Returns the element that gives the maximum value from the specified function.
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().max_by_key(|x| x.abs()).unwrap(), -10);
  }

  #[test]
  fn iter_max_by() {
    // Returns the element that gives the maximum value with respect to the specified comparison function.
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().max_by(|x, y| x.cmp(y)).unwrap(), 5);
  }

  // max_by and max_by_key also has min counterparts

  #[test]
  fn iter_rev() {
    // Reverses an iterator's direction.
    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_unzip() {
    // Converts an iterator of pairs into a pair of containers.
    let a = [(1, 2), (3, 4)];

    let (left, right): (Vec<_>, Vec<_>) = a.iter().cloned().unzip();
    assert_eq!(left, [1, 3]);
    assert_eq!(right, [2, 4]);
  }

  #[test]
  fn iter_cycle() {
    // Repeat an iterator endlessly
    let a = [1, 2, 3];

    let mut it = a.iter().cycle();

    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
  }

  #[test]
  fn iter_product() {
    // Iterates over the entire iterator, multiplying all the elements
    type Factorial = fn(u32) -> u32;
    let factorial: Factorial = |n| (1..=n).product();
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
  }

  #[test]
  fn iter_sum() {
    // Iterates over the entire iterator, adding all the elements
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().sum::<i32>(), 15);
  }
}
