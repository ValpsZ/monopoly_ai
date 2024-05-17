use std::f32::consts::E;

use crate::monopoly::Player;

// TODO: Make cards triger the spot they land on.
pub const CHANCE_CARDS: [fn(&mut Vec<Player>, usize); 16] = [
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.position = 0;
        player.money += 200;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position > 24 {
            player.money += 200;
        }
        player.position = 24;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position > 11 {
            player.money += 200;
        }
        player.position = 11;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position > 12 && player.position < 28 {
            player.position = 28;
        } else {
            if player.position > 28 {
                player.money += 200;
            }
            player.position = 12;
        }
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position < 5 || player.position > 35 {
            if player.position > 35 {
                player.money += 200;
            }
            player.position = 5;
        } else if player.position > 5 && player.position < 15 {
            player.position = 15;
        } else if player.position > 15 && player.position < 25 {
            player.position = 25;
        } else {
            player.position = 35;
        }
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position < 5 || player.position > 35 {
            if player.position > 35 {
                player.money += 200;
            }
            player.position = 5;
        } else if player.position > 5 && player.position < 15 {
            player.position = 15;
        } else if player.position > 15 && player.position < 25 {
            player.position = 25;
        } else {
            player.position = 35;
        }
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.money += 50;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.is_in_jail = false;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.position -= 3;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.position = 10;
        player.is_in_jail = true;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        let mut cost = 0;
        for idx in 0..player.propeties.len() {
            if player.propeties[idx].houses == 4 {
                cost += 100;
            } else {
                cost += 25 * player.propeties[idx].houses;
            }
        }
        player.money -= cost;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position > 5 {
            player.money += 200;
        }
        player.position = 5;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.money -= 15;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.position = 39;
    },
    |players, user_idx| {
        let cost = (players.len() as i32 - 1) * 50;
        for idx in 0..players.len() {
            if idx != user_idx {
                players[idx].money += 50;
            } else {
                players[idx].money -= cost;
            }
        }
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.money += 150;
    },
];
pub const COMMUNITY_CARDS: Vec<&'_ dyn Fn(Player)> = vec![];
