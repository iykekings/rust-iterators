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
}
