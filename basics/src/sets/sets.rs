// use std::collections::HashSet;

fn main() {
    use std::collections::HashSet;

    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(2);
    set_b.insert(4);
    set_b.insert(6);

    println!("Union: A ∪ B \n{:?}", set_a.union(&set_b));
    println!("Intersection: A ∩ B \n{:?}", set_a.intersection(&set_b));
    println!("Difference: A − B \n{:?}", set_a.difference(&set_b));
    println!(
        "Symmetric Difference: A Δ B \n{:?}",
        set_a.symmetric_difference(&set_b)
    );
}
