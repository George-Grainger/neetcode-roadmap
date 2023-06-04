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
        string_decode("we:;say:;:::;yes".to_string()),
        vec!["we", "say", ":", "yes"]
    );
}

fn string_decode(message: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    if message.len() < 2 {
        output.push(message);
        return output;
    }

    let mut chars = message.chars();
    let mut word = "".to_string();
    let mut prev: char = '\0';
    while let Some(current) = chars.next() {
        if current == ';' && prev == ':' {
            output.push(word.clone());
            word.clear();
        } else if current != ':' || (current == ':' && prev == ':') {
            word.push(current);

            if prev == ':' {
                chars.next();
            }
        }
        prev = current;
    }
    output.push(word);

    output
}
