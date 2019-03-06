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
