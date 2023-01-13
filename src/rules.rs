use std::collections::HashMap;

// Biding
fn tiles_biding() {
    HashMap::from([
        ("一筒", 1),
        ("二筒", 2),
        ("三筒", 3),
        ("四筒", 4),
        ("五筒", 5),
        ("六筒", 6),
        ("七筒", 7),
        ("八筒", 8),
        ("九筒", 9),

        ("一条", 11),
        ("二条", 12),
        ("三条", 13),
        ("四条", 14),
        ("五条", 15),
        ("六条", 16),
        ("七条", 17),
        ("八条", 18),
        ("九条", 19),

        ("一万", 21),
        ("二万", 22),
        ("三万", 23),
        ("四万", 24),
        ("五万", 25),
        ("六万", 26),
        ("七万", 27),
        ("八万", 28),
        ("九万", 29),

        ("东风", 31),
        ("南风", 32),
        ("西风", 33),
        ("北风", 34),
        ("红中", 35),
        ("发财", 36),
        ("白班", 37),

        ("花牌", 41),
        ("鬼牌", 42),
    ]);
}

// Combinations
pub struct Combinations{
    name: String,
    id: u8,
    kind: String,
    score: u8,
    func: fn() -> (bool, Vec<u8>),
    ignore: Vec<u8>,
    example: Vec<u8>,
    descriptions: String,
}

fn generate_combinations() -> Vec<Combinations>{
    vec![
        Combinations{
            name: String::from("大四喜"),
            id: 1,
            kind: String::from("duplicate"),
            score: 88,
            func: check_大四喜,
            ignore: vec![2,3],
            example: vec![31, 31, 31, 32, 32, 32, 33, 33, 33, 34, 34, 34, 1, 1],
            descriptions: String::from("和牌型中，有东南西北4副刻（杠）子。不计圈风刻、门风刻、三风刻、碰碰和、混一色。")
        },

        Combinations {

        }
    ]
}



// Winning functions 
fn check_大四喜() -> (bool, Vec<u8>) {
    // only 31 - 37 no

    (true, vec![1,2])
}
