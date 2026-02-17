pub fn talking(text: &str) -> &str {
    if text.is_empty(){
        return "Just say something!";
    }

    let is_question = text.ends_with("?");
    let has_letter = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    
    if is_question && is_yelling {
        return "Quiet, I am thinking!";
    }else if is_question {
        return "Sure.";
    }else if is_yelling && has_letter {
        return "There is no need to yell, calm down!";
    }else {
        return "Interesting";
    }
}
