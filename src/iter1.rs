#[cfg(test)]

mod test {
  #[test]
  fn iter_skip_while() {
    // Creates an iterator that skips elements based on a predicate.

    let a = vec![-132, 0, 1];
    let mut iter = a.iter().skip_while(|&&x| x < 0);

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_take_while() {
    // Creates an iterator that yields elements based on a predicate.
    let a = vec![4, 5, 0, 2, 4, 5];
    let mut iter = a.iter().take_while(|&&x| x > 3);
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_skip() {
    // Creates an iterator that skips the first n elements.
    let a = vec![3, 4, 5, 6, 7, 8, 0];
    let mut iter = a.iter().skip(6);

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_take() {
    // Creates an iterator that yields its first n elements. Kind of opposite to skip
    let a = vec![2, 3, 4, 5, 6, 7];
    let mut iter = a.iter().take(1);
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_scan() {
    // An iterator adaptor similar to fold that holds internal state and produces a new iterator.
    let a = vec![1, 2, 3, 4, 5];
    let mut iter = a.iter().scan(0, |state, &x| {
      *state += x;
      Some(*state)
    });
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(15));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_flat_map() {
    // Creates an iterator that works like map, but flattens nested structure.

    let a = ["alpha", ",", " beta", " and", " gamma"];
    let merged: String = a.iter().flat_map(|x| x.chars()).collect();
    assert_eq!("alpha, beta and gamma", merged);
  }

  #[test]
  fn iter_flatten() {
    // Creates an iterator that flattens nested structure.
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);
  }

  #[test]
  fn iter_partition() {
    // Consumes an iterator, creating two collections from it.
    let a1 = [1, 2, 3, 4, 5, 6];
    let (even, odd): (Vec<u32>, Vec<u32>) = a1.iter().partition(|&x| x % 2 == 0);
    assert_eq!(even, vec![2, 4, 6]);
    assert_eq!(odd, vec![1, 3, 5]);
  }

  #[test]
  fn iter_try_fold() {
    // An iterator method that applies a function as long as it returns successfully, producing a single, final value.
    let a = [10, 20, 30, 100, 40, 50];
    let mut iter = a.iter();
    // checked-add Performs addition that returns None instead of wrapping around on overflow.
    let sum = iter.try_fold(0i8, |acc, &x| acc.checked_add(x));
    assert_eq!(sum, None);

    assert_eq!(iter.next(), Some(&40));
    assert_eq!(iter.next(), Some(&50));
  }

  #[test]
  fn iter_fold() {
    // An iterator method that applies a function, producing a single, final value. Think reduce
    let a: Vec<_> = (1..=100).collect();
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 5050);
  }

  #[test]
  fn iter_all() {
    // Tests if every element of the iterator matches a predicate.
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();
    assert!(!iter.all(|&x| x < 3));

    // test shor-circuiting
    assert_eq!(iter.next(), Some(&4))
  }

  #[test]
  fn iter_any() {
    // Tests if any element of the iterator matches a predicate.
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert!(iter.any(|&x| x != 2));
    // we can still use `iter`, as there are more elements.
    assert_eq!(iter.next(), Some(&2));
  }

  #[test]
  fn iter_find() {
    // Searches for an element of an iterator that satisfies a predicate.
    let a = [1, 2, 3, 4];
    assert_eq!(a.iter().find(|&&x| x == 3), Some(&3));
    assert_eq!(a.iter().find(|&&x| x == 4), Some(&4));
    assert_eq!(a.iter().find(|&&x| x == 5), None);
  }

  #[test]
  fn iter_find_map() {
    // Applies function to the elements of iterator and returns the first non-none result.
    // iter.find_map(f) is equivalent to iter.filter_map(f).next().

    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));
  }

  #[test]
  fn iter_position() {
    // Searches for an element in an iterator, returning its index.
    let a = [1, 2, 3, 4];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));
    assert_eq!(a.iter().position(|&x| x == 4), Some(3));
    assert_eq!(a.iter().position(|&x| x == 5), None);
  }

  #[test]
  fn iter_max() {
    // Returns the maximum element of an iterator.
    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();
    let c = ["a", "c", "b"];
    assert_eq!(a.iter().max(), Some(&3));
    assert_eq!(b.iter().max(), None);
    assert_eq!(c.iter().max(), Some(&"c"));
  }
  #[test]
  fn iter_min() {
    // Returns the maximum element of an iterator.
    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();
    let c = ["a", "c", "b"];
    assert_eq!(a.iter().min(), Some(&1));
    assert_eq!(b.iter().min(), None);
    assert_eq!(c.iter().min(), Some(&"a"));
  }
}
