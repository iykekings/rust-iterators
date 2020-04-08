#[cfg(test)]
mod test {
  #[test]
  fn iter_count() {
    // returns the count items in the iterator
    let vec = vec![1, 2, 3];
    assert_eq!(vec.iter().count(), 3);
  }

  #[test]
  fn iter_last() {
    // returns the last item of the iterator
    let vec = vec![1, 2, 3];
    assert_eq!(vec.iter().last(), Some(&3));
    assert_eq!(vec.into_iter().last(), Some(3));
  }

  #[test]
  fn nth_iter() {
    // Returns the nth element of the iterator.
    let vec = vec![3, 4, 5, 6];
    let mut iter = vec.iter();
    assert_eq!(iter.nth(1), Some(&4));
    assert_eq!(iter.nth(1), Some(&6));
    assert_eq!(iter.nth(1), None);
  }

  #[test]
  fn iter_step_by() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let mut iter = vec.iter().step_by(2);
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&7));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_chain() {
    // Takes two iterators and creates a new iterator over both in sequence.
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![5, 6, 7];
    let mut iter = vec1.iter().chain(vec2.iter());
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), Some(&7));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_zip() {
    // 'Zips up' two iterators into a single iterator of pairs.
    let a1 = [1, 2, 3];
    let a2 = [4, 3, 2, 1];
    let mut iter = a1.iter().zip(a2.iter());
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &3)));
    assert_eq!(iter.next(), Some((&3, &2)));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_map() {
    // Takes a closure and creates an iterator which calls that closure on each element.
    let a1 = [2, 4, 5, 6, 7];
    let mut iter = a1.iter().map(|x| x * 3);
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(12));
    assert_eq!(iter.next(), Some(15));
    assert_eq!(iter.next(), Some(18));
    assert_eq!(iter.next(), Some(21));
  }

  #[test]
  fn iter_filter() {
    // Creates an iterator which uses a closure to determine if an element should be yielded.
    let a = vec![1, 2, 3, 4, 5, 6];
    a.iter()
      .enumerate()
      .filter(|&(_, &x)| x % 2 == 0)
      .for_each(|(i, x)| assert_eq!(x, &a[i]));
  }

  #[test]
  fn iter_filter_map() {
    // Creates an iterator that both filters and maps.
    // closure must return an Option<T>
    let a = ["1", "lol", "3", "NaN", "5"];

    let mut iter = a.iter().filter_map(|s| s.parse().ok());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_peekable() {
    let a = vec![1, 2, 3, 4];
    let mut iter = a.iter().peekable();

    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.peek(), Some(&&3));
  }
}
