fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn concatenate_str(s1: String, s2: String) -> String {
    let mut s1 = s1;

    // trim_newline(&mut s1);

    if s1.ends_with('\n') {
        s1.pop();
        if s1.ends_with('\r') {
            s1.pop();
        }
    }

    format!("{}{}", s1, s2)
}

pub fn is_vowel(c: &char) -> bool {
    c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u'
}

pub fn concatenate(input: Vec<u32>) -> u64 {
    return input
        .iter()
        .fold(0, |acc: u64, elem: &u32| acc * 10 + (*elem as u64));
}

pub fn lcm_of(input: Vec<u64>) -> u64 {
    let mut total = 1;

    input.into_iter().for_each(|value| {
        total = lcm(total, value);
    });

    return total;
}

pub fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
