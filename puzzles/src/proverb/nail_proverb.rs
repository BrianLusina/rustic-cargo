pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut stanzas = Vec::with_capacity(list.len());
    for index in 0..list.len() {
        if index == list.len() - 1 {
            stanzas.push(format!("And all for the want of a {}.", list[0]));
        } else {
            stanzas.push(format!(
                "For want of a {} the {} was lost.",
                list[index],
                list[index + 1]
            ));
        }
    }

    stanzas.join("\n")
}
