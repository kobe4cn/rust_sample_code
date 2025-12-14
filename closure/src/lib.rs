
#[derive(Debug, PartialEq, Eq, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_giveaway() {
        let store = Inventory {
            shirts: vec![
                ShirtColor::Red,
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Red,
            ],
        };
        let giveaway = store.giveaway(None);
        println!("giveaway none preference: {:?}", giveaway);
        assert_eq!(giveaway, ShirtColor::Red);
    }
    #[test]
    fn test_giveaway_with_preference() {
        let store = Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
        };
        let giveaway = store.giveaway(Some(ShirtColor::Red));
        println!("giveaway: {:?}", giveaway);
        assert_eq!(giveaway, ShirtColor::Red);
    }
}
