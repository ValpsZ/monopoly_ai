pub enum MonopolyErrors {
    MortgageWithHouseError,
    InsufficientHousesError,
}

pub struct Game<'g> {
    players: Vec<Player<'g>>,
    properties: Vec<Property>,
    chance_cards: Vec<Chance<'g>>,
    community_cards: Vec<Community<'g>>,
}

pub struct Player<'g> {
    pub id: i32,
    pub position: i32,
    pub is_in_jail: bool,
    money: i32,
    pub propeties: Vec<Property>,
    chance: Vec<Chance<'g>>,
    community: Vec<Community<'g>>,
    pub bankrupt: bool,
}

pub struct Property {
    id: i32,
    price: i32,
    pub houses: i32,
    rent: i32,
    owner_id: i32,
    pub house_price: i32,
    mortgaged: bool,
}

pub struct Chance<'g> {
    id: i32,
    can_save: bool,
    action: &'g dyn Fn(Player) -> (),
}

pub struct Community<'g> {
    id: i32,
    can_save: bool,
    action: &'g dyn Fn(Player) -> (),
}

impl Player<'_> {
    pub fn move_to(&mut self, position: i32, collect_go: bool) {
        if collect_go && (self.position < position || position == 0) {
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
}
