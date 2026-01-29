#[derive(Debug)]
struct Player {
    name: &'static str,
}

fn duck_duck_goose(players: &[Player], goose: u32) -> &'static str {
    let len = players.len();
    &players[(goose as usize - 1) % len].name
}

#[cfg(test)]
mod tests {
    use super::{duck_duck_goose, Player};

    fn do_test(players: &[Player], idx: u32, expected: &str) {
        let actual = duck_duck_goose(players, idx);
        assert_eq!(actual, expected, "Expected the {idx}th player to be {expected:?}, but you returned {actual:?}\nPlayers:\n{players:?}\n")
    }

    #[test]
    fn fixed_tests() {
        let players: Vec<Player> = ["a", "b", "c", "d", "c", "e", "f", "g", "h", "z"]
            .into_iter()
            .map(|name| Player { name })
            .collect();

        do_test(&players, 1, "a");
        do_test(&players, 3, "c");
        do_test(&players, 10, "z");
        do_test(&players, 20, "z");
        do_test(&players, 30, "z");
        do_test(&players, 18, "g");
        do_test(&players, 28, "g");
        do_test(&players, 12, "b");
        do_test(&players, 2, "b");
        do_test(&players, 7, "f");
    }
}
