fn main() {
    let v = vec!['4', 'd', 'd', '4'];
    let s: Vec<f32>;
    calc(v, s);
}

fn calc(mut v: Vec<char>, mut s: Vec<f32>) -> f32 {
    let mut tmp;
    for c in v {
        if c.is_digit(10) {
            s.push(c.to_digit(10).unwrap() as f32);
        } else if c == '+' {
            s.push(s.pop() + s.pop());
        } else if c == '-' {
            tmp = s.pop();
            s.push(s.pop() - tmp);
        } else if c == '*' {
            s.push(s.pop() * s.pop());
        } else if c == '/' {
            tmp = s.pop();
            if tmp == 0.0 {
                return f32::max;
            } else {
                s.push(s.pop() / tmp);
            }
        }
    }
    return s.pop()
}
