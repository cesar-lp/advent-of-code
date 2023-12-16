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