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
    pub money: i32,
    pub propeties: Vec<Property>,
    pub chance: Vec<Chance<'g>>,
    pub community: Vec<Community<'g>>,
}

pub struct Property {
    id: i32,
    price: i32,
    pub houses: i32,
    rent: i32,
    owner_id: i32,
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
