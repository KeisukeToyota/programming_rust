fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
  let mut v = Vec::<i32>::new();

  while left.len() > 0 && right.len() > 0 {
    let x: i32;

    if right[0] > left[0] {
      x = left[0];
      left = left[1..].to_vec();
    } else {
      x = right[0];
      right = right[1..].to_vec();
    }
    v.push(x);
  }

  v.append(&mut left);
  v.append(&mut right);
  v
}

fn merge_sort(v: Vec<i32>) -> Vec<i32> {
  if v.len() <= 1 {
    return v;
  }
  let (left, right) = v.split_at(v.len() / 2);
  return merge(merge_sort(left.to_vec()), merge_sort(right.to_vec()));
}

fn main() {
  let v = vec![23, 1, 4, 32, 8, 90, 73, 100];
  println!("{:?}", merge_sort(v));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge() {
    let (x, y) = (vec![1,2,3], vec![4,5,6]);
    let (a, b) = (vec![2, 4, 6], vec![1, 7, 9]);
    assert_eq!(merge(x, y), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(merge(a, b), vec![1, 2, 4, 6, 7, 9]);
  }

  #[test]
  fn test_merge_sort() {
    let x = vec![6, 5, 4, 3, 2, 1];
    let y = vec![1, 5, 2, 7, 9, 18];
    assert_eq!(merge_sort(x), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(merge_sort(y), vec![1, 2, 5, 7, 9, 18]);
  }
}