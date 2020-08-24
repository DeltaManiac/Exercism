fn is_uppercase(message: &str) -> bool {
    message.chars().all(|c| !c.is_lowercase())
        && (message.chars().filter(|x| x.is_alphabetic()).count() > 0)
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with("?") && is_uppercase(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_uppercase(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
