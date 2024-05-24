use crate::monopoly::PropertyColor;

pub const PROPERTY_COLORS: [PropertyColor; 28] = [
    PropertyColor::Brown,
    PropertyColor::Brown,
    PropertyColor::Rail,
    PropertyColor::LBlue,
    PropertyColor::LBlue,
    PropertyColor::LBlue,
    PropertyColor::Pink,
    PropertyColor::Util,
    PropertyColor::Pink,
    PropertyColor::Pink,
    PropertyColor::Rail,
    PropertyColor::Orange,
    PropertyColor::Orange,
    PropertyColor::Orange,
    PropertyColor::Red,
    PropertyColor::Red,
    PropertyColor::Red,
    PropertyColor::Rail,
    PropertyColor::Yellow,
    PropertyColor::Yellow,
    PropertyColor::Util,
    PropertyColor::Yellow,
    PropertyColor::Green,
    PropertyColor::Green,
    PropertyColor::Green,
    PropertyColor::Rail,
    PropertyColor::Blue,
    PropertyColor::Blue,
];

pub const PROPERTY_HOUSES: [i32; 28] = [
    50, 50, 0, 50, 50, 50, 100, 0, 100, 100, 0, 100, 100, 100, 150, 150, 150, 0, 150, 150, 0, 150,
    200, 200, 200, 0, 200, 200,
];

pub const PROPERTY_RENTS: [[i32; 6]; 28] = [
    [2, 10, 30, 90, 160, 250],
    [4, 20, 60, 180, 320, 450],
    [25, 50, 100, 200, -1, -1],
    [6, 30, 90, 270, 400, 550],
    [6, 30, 90, 270, 400, 550],
    [8, 40, 100, 300, 450, 600],
    [10, 50, 150, 450, 625, 750],
    [4, 10, -1, -1, -1, -1],
    [10, 50, 150, 450, 625, 750],
    [12, 60, 180, 500, 700, 900],
    [25, 50, 100, 200, -1, -1],
    [14, 70, 200, 550, 750, 950],
    [14, 70, 200, 550, 750, 950],
    [16, 80, 220, 600, 800, 1000],
    [18, 90, 250, 700, 875, 1050],
    [18, 90, 250, 700, 875, 1050],
    [20, 100, 300, 750, 925, 1100],
    [25, 50, 100, 200, -1, -1],
    [22, 110, 330, 800, 975, 1150],
    [22, 110, 330, 800, 975, 1150],
    [4, 10, -1, -1, -1, -1],
    [24, 120, 360, 850, 1025, 1200],
    [26, 130, 390, 900, 1100, 1275],
    [26, 130, 390, 900, 1100, 1275],
    [28, 150, 450, 1000, 1200, 1400],
    [25, 50, 100, 200, -1, -1],
    [35, 175, 500, 1100, 1300, 1500],
    [50, 200, 600, 1400, 1700, 2000],
];

pub const PROPERTY_PRICES: [i32; 28] = [
    60, 60, 100, 100, 200, 120, 140, 150, 140, 160, 200, 180, 180, 200, 220, 220, 240, 200, 260,
    260, 150, 280, 300, 300, 320, 200, 350, 400,
];

pub const PROPERTY_POSITIONS: [i32; 28] = [
    1, 3, 5, 6, 8, 9, 11, 12, 13, 14, 15, 16, 18, 19, 21, 23, 24, 25, 26, 27, 28, 29, 31, 32, 34,
    35, 37, 39,
];

pub const PROPERTY_AMOUNTS: [i32; 10] = [2, 3, 3, 3, 3, 3, 3, 2, 2, 4];
