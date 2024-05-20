use crate::monopoly::Player;

// TODO: Make cards trigger the spot they land on.
pub const CHANCE_CARDS: [fn(&mut Vec<Player>, usize); 16] = [
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(0, true);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(24, true);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(11, true);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        if player.position > 12 && player.position < 28 {
            player.move_to(28, true);
        } else {
            player.move_to(12, true);
        }
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
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
    |players, user_idx| {
        let player = &mut players[user_idx];
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
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(50);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.is_in_jail = false;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(player.position - 3, false);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(10, false);
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
        player.pay(cost);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(5, true);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.pay(15);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(39, false);
    },
    |players, user_idx| {
        let cost = (players.len() as i32 - 1) * 50;
        let payed = players[user_idx].pay(cost);
        let mut reward = 50;
        if payed != cost {
            let reward = payed / (players.len() - 1) as i32;
        }
        for idx in 0..players.len() {
            if idx != user_idx {
                players[idx].collect(reward);
            }
        }
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(150);
    },
];

pub const COMMUNITY_CARDS: [fn(&mut Vec<Player>, usize); 16] = [
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(100);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(50);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.pay(50);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.is_in_jail = false;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        let mut gain = 0;
        for idx in 0..players.len() {
            if idx != user_idx {
                gain += players[idx].pay(10);
            }
        }
        player.collect(gain);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(10, false);
        player.is_in_jail = true;
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(20);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(100);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(100);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.pay(100);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.move_to(0, true);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(200);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(50);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        let mut cost = 0;
        for idx in 0..player.propeties.len() {
            if player.propeties[idx].houses == 4 {
                cost += 115;
            } else {
                cost += 40 * player.propeties[idx].houses;
            }
        }
        player.pay(cost);
    },
    |players, user_idx| {
        let player = &mut players[user_idx];
        player.collect(25);
    },
];
