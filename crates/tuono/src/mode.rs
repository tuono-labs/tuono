#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Mode {
    Prod,
    Dev,
}

impl Mode {
    pub fn as_str<'a>(&self) -> &'a str {
        if *self == Mode::Dev {
            return "Mode::Dev";
        }
        "Mode::Prod"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_correctly_print_the_mode_as_str() {
        let dev = Mode::Dev.as_str();
        let prod = Mode::Prod.as_str();
        assert_eq!(dev, "Mode::Dev");
        assert_eq!(prod, "Mode::Prod");
    }
}
