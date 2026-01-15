pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!";
    }

    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_question = text.ends_with('?');

    if is_yelling && is_question {
        "Quiet, I am thinking!"
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else if is_question {
        "Sure."
    } else {
        "Interesting"
    }
}
