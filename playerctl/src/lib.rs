#[derive(Debug)]
pub struct PlayerManager {
    selected_player: Option<String>,
}

impl PlayerManager {
    pub fn new(player: Option<String>) -> Self {
        Self {
            selected_player: player,
        }
    }

    pub fn run(&self) {
        println!("Player {:?}", self.selected_player);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
