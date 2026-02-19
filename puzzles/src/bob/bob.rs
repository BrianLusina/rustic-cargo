pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if question(message) && all_caps(message) {
        if all_caps(message) {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure"
    }

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if all_caps(message) {
        return "Whoa, chill out!";
    }

    "Whatever."
}

fn all_caps(message: &str) -> bool {
    message.eq(&message.to_uppercase())
}

fn question(message: &str) -> bool {
    message.ends_with("?")
}