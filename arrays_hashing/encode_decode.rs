// Solution for: https://leetcode.com/problems/encode-and-decode-strings/
// Free link: https://www.lintcode.com/problem/659/

#[test]
fn test_string_encode() {
    assert_eq!(
        string_encode(
            vec!["lint", "code", "love", "you"]
                .iter()
                .map(|v| v.to_string())
                .collect()
        ),
        "lint:;code:;love:;you"
    );
}

pub fn string_encode(strs: Vec<String>) -> String {
    let mut output = strs[0].to_string();
    for s in &strs[1..] {
        output.push_str(":;");
        if s == ":" {
            output.push_str("::");
        } else {
            output.push_str(s);
        }
    }
    output
}

#[test]
fn test_string_decode() {
    assert_eq!(
        string_decode(String::from("we:;say:;:::;yes")),
        vec!["we", "say", ":", "yes"]
    );

    assert_eq!(string_decode(String::from("")), vec![""]);
    assert_eq!(string_decode(String::from("test")), vec!["test"]);
}

fn string_decode(message: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut chars = message.chars();

    let mut word = String::from("");
    while let Some(current) = chars.next() {
        if current != ':' {
            word.push(current);
            continue;
        }

        match chars.next() {
            Some(':') => word = String::from(":"),
            Some(';') => {
                output.push(word.clone());
                word.clear();
            }
            _ => (),
        }
    }
    output.push(word);

    output
}
