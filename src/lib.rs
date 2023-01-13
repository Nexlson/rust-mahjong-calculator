// common functions

fn calculate_score() {
    // take in tiles in the format of 3 + 3 + 3 + 3 + 2
}

fn grouping() {

}

pub fn check_exactly_tile(combo: Vec<u8>) -> bool {
    combo[0] == combo[1] && combo[0] == combo[2]
}

// pub fn check_same_tile_pattern(combo: Vec<u32>) -> bool {
//     let mut processed_tiles = vec![];
//     for val in combo {
//        match val {
//         1..=9 => processed_tiles.push(val),
//         11..=19 => processed_tiles.push(val/10),
//         21..=29 => processed_tiles.push(val/20),
//         31..=39 => processed_tiles.push(val/30),
//         _ => println!("Wrong tile data"),
//        }
//     }
//     true
// }