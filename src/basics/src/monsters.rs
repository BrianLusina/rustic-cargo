enum Monster {
    LochNess,
    Dracula,
    Bigfoot,
    Alien,
}

impl Monster {
    fn lives(&self) -> Optional<Place> {
        use Monster::*;
        use Place::*;
        use Optional::*;

        match self {
            LochNess => Known(Scotland),
            Dracula => Known(Transylvania),
            Bigfoot => Unknown,
            Alien => Unknown,
        }
    }

    fn eats(&self) -> Option<Food> {
        use Monster::*;
        match self {
            LochNess => {Some(Food::Fish)}
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

enum Optional<T> {
    Known(T),
    Unknown,
}