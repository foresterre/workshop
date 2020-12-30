enum Fruit {
    Apple,
    Bramble(BrambleFruit),
    Pear,
}

trait NameOf {
    fn name_of(&self) -> &str;
}

impl NameOf for Fruit {
    fn name_of(&self) -> &str {
        match self {
            Fruit::Apple => "apple",
            Fruit::Bramble(fruit) => fruit.name_of(),
            Fruit::Pear => "pear",
        }
    }
}

enum BrambleFruit {
    Blackberry,
}

impl NameOf for BrambleFruit {
    fn name_of(&self) -> &str {
        match self {
            BrambleFruit::Blackberry => "blackberry",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(fruit = {
        Fruit::Apple, Fruit::Pear, Fruit::Bramble(BrambleFruit::Blackberry)
    }, name = {
        "apple", "pear", "blackberry"
    })]
    fn a_fruity_test(fruit: Fruit, name: &str) {
        assert_eq!(fruit.name_of(), name)
    }
}
