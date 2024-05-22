use rand::Rng;

use crate::cards;

pub enum MonopolyErrors {
    MortgageWithHouseError,
    InsufficientHousesError,
}
pub enum PropertyType {
    Street,
    Rail,
    Util,
}

pub struct Game<'g> {
    players: Vec<Player>,
    properties: Vec<Property>,
    chance_cards: Vec<Card<'g>>,
    community_cards: Vec<Card<'g>>,
}

pub struct Player {
    pub id: i32,
    pub position: i32,
    pub is_in_jail: bool,
    pub money: i32,
    pub propeties: Vec<Property>,
    pub chance_jail: bool,
    pub community_jail: bool,
    pub bankrupt: bool,
}

pub struct Property {
    id: i32,
    price: i32,
    pub position: i32,
    pub prop_type: PropertyType,
    pub houses: i32,
    rent: i32,
    owner_id: Option<usize>,
    pub house_price: i32,
    mortgaged: bool,
}

pub struct Card<'g> {
    id: usize,
    can_save: bool,
    action: &'g dyn Fn(&mut Vec<Player>, usize) -> (),
}

impl Player {
    pub fn move_to(&mut self, position: i32, collect_go: bool) {
        if collect_go && (self.position > position || position == 0) {
            self.collect(200);
        }
        self.position = position;
    }

    pub fn collect(&mut self, balance: i32) {
        self.money += balance;
    }

    pub fn pay(&mut self, cost: i32) -> i32 {
        if self.money > cost {
            self.money -= cost;
            return cost;
        }
        let networth = self.calculate_networth();
        if networth < cost {
            self.bankrupt = true;
            self.sell_everything();
            self.money = 0;
            return networth;
        } else {
            self.money -= cost;
            return cost;
        }
    }

    pub fn calculate_networth(&self) -> i32 {
        let mut sum: i32 = 0;
        sum += self.money;
        sum += self
            .propeties
            .iter()
            .map(|property| (property.house_price / 2) * property.houses + (property.price / 2))
            .sum::<i32>();

        return sum;
    }

    fn sell_everything(&mut self) {
        for property in &mut self.propeties {
            property.sell_houses(property.houses);
            property.mortgage();
        }
    }
}

impl Property {
    pub fn mortgage(&mut self) -> Result<i32, MonopolyErrors> {
        if self.houses > 0 && self.house_price != 0 {
            return Err(MonopolyErrors::MortgageWithHouseError);
        }
        self.mortgaged = true;
        return Ok(self.price / 2);
    }

    pub fn sell_houses(&mut self, amount: i32) -> Result<i32, MonopolyErrors> {
        if self.houses < amount || self.house_price == 0 {
            return Err(MonopolyErrors::InsufficientHousesError);
        }
        self.houses -= amount;
        return Ok((self.house_price / 2) * amount);
    }

    pub fn get_rent(&self, user_idx: usize, dice_roll: i32) -> i32 {
        match self.owner_id {
            Some(id) => {
                if id == user_idx {
                    return 0;
                }

                match self.prop_type {
                    PropertyType::Street | PropertyType::Rail => return self.rent,
                    PropertyType::Util => return self.rent * dice_roll,
                }
            }
            _ => (),
        }
        return 0;
    }
}

impl Game<'_> {
    pub fn turn(&mut self, user_idx: usize, doubles: i32) {
        let mut rng = rand::thread_rng();
        let dice1: i32 = rng.gen_range(0..=6);
        let dice2: i32 = rng.gen_range(0..=6);
        if dice1 == dice2 && doubles == 2 {
            self.players[user_idx].move_to(10, false);
            self.players[user_idx].is_in_jail = true;
            return;
        }
        let new_position = (self.players[user_idx].position + dice1 + dice2) % 40;
        self.players[user_idx].move_to(new_position, true);
        self.handle_player_move(user_idx, dice1 + dice2);
        if dice1 == dice2 {
            self.turn(user_idx, doubles + 1)
        }
    }

    pub fn handle_player_move(&mut self, user_idx: usize, dice_roll: i32) {
        let mut property_idx: Option<usize> = None;
        for property_loop_idx in 0..self.properties.len() {
            if self.properties[property_loop_idx].position == self.players[user_idx].position {
                property_idx = Some(property_loop_idx);
                break;
            }
        }
        match property_idx {
            Some(idx) => match idx {
                2 | 17 | 33 => {
                    self.draw_community(user_idx);
                }
                7 | 22 | 36 => {
                    self.draw_chance(user_idx);
                }
                4 => {
                    self.players[user_idx].pay(200);
                }
                38 => {
                    self.players[user_idx].pay(100);
                }
                30 => {
                    self.players[user_idx].move_to(10, false);
                }
                0 | 10 | 20 => {}
                _ => {
                    let rent = self.properties[idx].get_rent(user_idx, dice_roll);
                    self.players[user_idx].pay(rent);
                }
            },
            _ => {}
        }
    }

    pub fn draw_community(&mut self, user_idx: usize) {
        match self.community_cards.pop() {
            Some(card) => {
                card.execute_action(&mut self.players, user_idx);
            }
            None => {
                self.shuffle_community();
                self.draw_community(user_idx);
            }
        };
    }

    pub fn draw_chance(&mut self, user_idx: usize) {
        match self.chance_cards.pop() {
            Some(card) => {
                card.execute_action(&mut self.players, user_idx);
            }
            None => {
                self.shuffle_chance();
                self.draw_chance(user_idx);
            }
        };
    }

    pub fn shuffle_community(&mut self) {
        for idx in 0..cards::COMMUNITY_FN.len() {
            if idx == 5 {
                if self.players.iter().any(|player| player.chance_jail) {
                    continue;
                }
                let action = &cards::COMMUNITY_FN[idx];
                self.community_cards
                    .push(Card::new(idx, true, action))
            } else {
                let action = &cards::COMMUNITY_FN[idx];
                self.community_cards
                    .push(Card::new(idx, false, action))
            }
        }
    }

    pub fn shuffle_chance(&mut self) {
        for idx in 0..cards::CHANCE_FN.len() {
            if idx == 8 {
                if self.players.iter().any(|player| player.chance_jail) {
                    continue;
                }

                let action = &cards::CHANCE_FN[idx];
                self.chance_cards.push(Card::new(idx, true, action))
            } else {
                let action = &cards::COMMUNITY_FN[idx];
                self.chance_cards.push(Card::new(idx, false, action))
            }
        }
    }
}

impl<'g> Card<'g> {
    pub fn new(id: usize, can_save: bool, action: &'g dyn Fn(&mut Vec<Player>, usize)) -> Card<'g> {
        Card {
            id,
            can_save,
            action,
        }
    }

    pub fn execute_action(&self, players: &mut Vec<Player>, user_idx: usize) {
        (self.action)(players, user_idx);
    }
}
