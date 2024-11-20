enum Monster {
    LochNess,
    Dracula,
    Bigfoot,
    Alien,
}

impl Monster {
    fn lives(&self) -> Option<Place> {
        use Monster::*;
        use Place::*;

        match self {
            LochNess => Some(Scotland),
            Dracula => Some(Transylvania),
            Bigfoot => None,
            Alien => None,
        }
    }

    fn eats(&self) -> Option<Food> {
        use Monster::*;
        match self {
            LochNess => Some(Food::Fish),
            Dracula => Some(Food::Blood),
            Bigfoot => Some(Food::Berries),
            Alien => Some(Food::Cows),
        }
    }
}

enum Place {
    Scotland,
    Transylvania,
}

enum Food {
    Blood,
    Cows,
    Berries,
    Fish,
}
