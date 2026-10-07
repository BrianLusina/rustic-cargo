use puzzles::bob::bob::*;

#[cfg(test)]
mod bob_tests {
    use parameterized::parameterized;
    use parameterized::ide;
    use puzzles::bob::bob::*;

    ide!();

    #[parameterized(
        message = {
            "Tom-ay-to, tom-aaaah-to.",
            "WATCH OUT!",
            "FCECDFCAAB",
            "Does this cryogenic chamber make me look fat?",
            "You are, what, like 15?",
            "fffbbcbeab?",
            "Hi there!",
            "It's OK if you don't want to go work for NASA.",
            "WHAT'S GOING ON?",
            "1, 2, 3 GO!"
        },
        expected = {
            "Whatever.",
            "Whoa, chill out!",
            "Whoa, chill out!",
            "Sure.",
            "Sure.",
            "Sure.",
            "Whatever.",
            "Whatever.",
            "Calm down, I know what I'm doing!",
            "Whoa, chill out!"
        }
    )]
    fn test_reply(message: &str, expected: &str) {
        let actual = reply(message);
        assert_eq!(expected, actual);
    }
}

#[test]
fn shouting_numbers() {
    assert_eq!(reply("1, 2, 3 GO!"), "Whoa, chill out!");
}

#[test]
fn no_letters() {
    assert_eq!(reply("1, 2, 3"), "Whatever.");
}

#[test]
fn question_with_no_letters() {
    assert_eq!(reply("4?"), "Sure.");
}

#[test]
fn shouting_with_special_characters() {
    assert_eq!(
        reply("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"),
        "Whoa, chill out!"
    );
}

#[test]
fn shouting_with_no_exclamation_mark() {
    assert_eq!(reply("I HATE THE DENTIST"), "Whoa, chill out!");
}

#[test]
fn statement_containing_question_mark() {
    assert_eq!(reply("Ending with ? means a question."), "Whatever.");
}

#[test]
fn non_letters_with_question() {
    assert_eq!(reply(":) ?"), "Sure.");
}

#[test]
fn prattling_on() {
    assert_eq!(reply("Wait! Hang on. Are you going to be OK?"), "Sure.");
}

#[test]
fn silence() {
    assert_eq!(reply(""), "Fine. Be that way!");
}

#[test]
fn prolonged_silence() {
    assert_eq!(reply("          "), "Fine. Be that way!");
}

#[test]
fn alternate_silence() {
    assert_eq!(reply("\t\t\t\t\t\t\t\t\t\t"), "Fine. Be that way!");
}

#[test]
fn starting_with_whitespace() {
    assert_eq!(reply("         hmmmmmmm..."), "Whatever.");
}

#[test]
fn ending_with_whitespace() {
    assert_eq!(reply("Okay if like my  spacebar  quite a bit?   "), "Sure.");
}

#[test]
fn other_whitespace() {
    assert_eq!(reply("\n\r \t"), "Fine. Be that way!");
}

#[test]
fn non_question_ending_with_whitespace() {
    assert_eq!(
        reply("This is a statement ending with whitespace      "),
        "Whatever."
    );
}

#[test]
fn multiple_line_question() {
    assert_eq!(
        reply("\nDoes this cryogenic chamber make\n me look fat?"),
        "Sure."
    );
}
