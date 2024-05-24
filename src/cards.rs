#![allow(dead_code)]
use crate::monopoly::{Player, PropertyColor};

pub const CHANCE_FN: [fn(&mut Vec<Player>, usize); 16] = [
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(0, true);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(24, true);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(11, true);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        if player.position > 12 && player.position < 28 {
            player.move_to(28, true);
        } else {
            player.move_to(12, true);
        }
    },
    |players, user_id| {
        let player = &mut players[user_id];
        if player.position < 5 || player.position > 35 {
            player.move_to(5, true);
        } else if player.position > 5 && player.position < 15 {
            player.move_to(15, false);
        } else if player.position > 15 && player.position < 25 {
            player.move_to(25, false);
        } else {
            player.move_to(35, false);
        }
    },
    |players, user_id| {
        let player = &mut players[user_id];
        if player.position < 5 || player.position > 35 {
            player.move_to(5, true);
        } else if player.position > 5 && player.position < 15 {
            player.move_to(15, false);
        } else if player.position > 15 && player.position < 25 {
            player.move_to(25, false);
        } else {
            player.move_to(35, false);
        }
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(50);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.chance_jail = true;
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(player.position - 3, false);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(10, false);
        player.is_in_jail = true;
    },
    |players, user_id| {
        let player = &mut players[user_id];
        let mut cost = 0;
        for id in 0..player.propeties.len() {
            if player.propeties[id].color == PropertyColor::Util
                || player.propeties[id].color == PropertyColor::Rail
            {
                continue;
            }

            if player.propeties[id].houses == 4 {
                cost += 100;
            } else {
                cost += 25 * player.propeties[id].houses as i32;
            }
        }
        player.pay(cost);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(5, true);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.pay(15);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(39, false);
    },
    |players, user_id| {
        let cost = (players.len() as i32 - 1) * 50;
        let payed = players[user_id].pay(cost);
        let mut reward = 50;
        if payed != cost {
            reward = payed / (players.len() - 1) as i32;
        }
        for id in 0..players.len() {
            if id != user_id {
                players[id].collect(reward);
            }
        }
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(150);
    },
];

pub const COMMUNITY_FN: [fn(&mut Vec<Player>, usize); 16] = [
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(100);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(50);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(10);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.pay(50);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.community_jail = true;
    },
    |players, user_id| {
        let mut gain = 0;
        for id in 0..players.len() {
            if id != user_id {
                gain += players[id].pay(10);
            }
        }
        let player = &mut players[user_id];
        player.collect(gain);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(10, false);
        player.is_in_jail = true;
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(20);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(100);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(100);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.pay(100);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.move_to(0, true);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(200);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(50);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        let mut cost = 0;
        for id in 0..player.propeties.len() {
            if player.propeties[id].color == PropertyColor::Util
                || player.propeties[id].color == PropertyColor::Rail
            {
                continue;
            }

            if player.propeties[id].houses == 4 {
                cost += 115;
            } else {
                cost += 40 * player.propeties[id].houses as i32;
            }
        }
        player.pay(cost);
    },
    |players, user_id| {
        let player = &mut players[user_id];
        player.collect(25);
    },
];
