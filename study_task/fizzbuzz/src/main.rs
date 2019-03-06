fn is_fizz(x: i32) -> bool {
    x % 3 == 0
}

fn is_buzz(x: i32) -> bool {
    x % 5 == 0
}

fn is_fizzbuzz(x: i32) -> bool {
    is_fizz(x) && is_buzz(x)
}

// 一般的なやつ
// fn fizzbuzz(x: i32) -> String {
//     if is_fizzbuzz(x) {
//         "FizzBuzz".to_string()
//     } else if is_fizz(x) {
//         "Fizz".to_string()
//     } else if is_buzz(x) {
//         "Buzz".to_string()
//     } else {
//         x.to_string()
//     }
// }

// パターンマッチ使ったやつ
fn fizzbuzz(x: i32) -> String {
    return match x {
        n if is_fizzbuzz(n) => "FizzBuzz".to_string(),
        n if is_fizz(n) => "Fizz".to_string(),
        n if is_buzz(n) => "Buzz".to_string(),
        n => n.to_string()
    }
}

fn main() {
    for x in 1..100 {
        println!("{}", fizzbuzz(x));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_fizz() {
        let (x, y, z) = (3, 5, 15);
        assert_eq!(is_fizz(x), true);
        assert_eq!(is_fizz(y), false);
        assert_eq!(is_fizz(z), true);
    }

    #[test]
    fn test_is_buzz() {
        let (x, y, z) = (3, 5, 15);
        assert_eq!(is_buzz(x), false);
        assert_eq!(is_buzz(y), true);
        assert_eq!(is_buzz(z), true);
    }

    #[test]
    fn test_is_fizzbuzz() {
        let (a, b, c) = (8, 11, 91);
        let (x, y, z) = (3, 5, 15);
        assert_eq!(is_fizzbuzz(a), false);
        assert_eq!(is_fizzbuzz(b), false);
        assert_eq!(is_fizzbuzz(c), false);
        assert_eq!(is_fizzbuzz(x), false);
        assert_eq!(is_fizzbuzz(y), false);
        assert_eq!(is_fizzbuzz(z), true);
    }

    #[test]
    fn test_fizzbuzz() {
        let (a, b, c) = (8, 11, 91);
        let (x, y, z) = (3, 5, 15);
        assert_eq!(fizzbuzz(a), a.to_string());
        assert_eq!(fizzbuzz(b), b.to_string());
        assert_eq!(fizzbuzz(c), c.to_string());
        assert_eq!(fizzbuzz(x), "Fizz".to_string());
        assert_eq!(fizzbuzz(y), "Buzz".to_string());
        assert_eq!(fizzbuzz(z), "FizzBuzz".to_string());
    }
}