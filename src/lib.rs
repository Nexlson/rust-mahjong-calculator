use std::{collections::HashMap, vec, f32::consts::PI, ops::ControlFlow};
use itertools::Itertools;

pub struct Result {
    winning_combinations: Vec<String>,
    scores: u8,
    ignored_combinations: Vec<u8>
}

pub struct Combinations{
    pub name: String,
    pub id: u8,
    pub score: u8,
    pub ignore: Vec<u8>,
    pub example: Vec<u8>,
    pub descriptions: String,
}

pub struct PlayerHand {
    grouped_tiles: Vec<Combo>,
    own_draw: bool,
    flowers: u8,
}

pub struct Combo {
    tiles: Vec<u8>,
    own: bool 
}

// Methods
impl Combo {
    fn check_all_same_characters(&self) -> (u8, bool){
        let mut counts : HashMap<u8, i32>= HashMap::from([
            (0, 0),
            (10, 0),
            (20, 0),
            (30, 0),
        ]);
        for tile in &self.tiles {
            match tile {
                1..=9 => {
                    counts.entry(0).and_modify(|v| *v += 1);
                }
                11..=19 => {
                    counts.entry(10).and_modify(|v| *v += 1);
                }
                21..=29 => {
                    counts.entry(20).and_modify(|v| *v += 1);
                }
                31..=37 => {
                    counts.entry(30).and_modify(|v| *v += 1);
                }
                _ => {
                    println!("Error identifying characters")
                }
            }
        };
        for (key, value) in &counts {
            if *value == 3 {
                return (*key, true)
            }
        }
        (0, false)
    }

    fn check_chi_combination(&self) -> (bool){
        if self.tiles.len() == 3 {
            return (self.tiles[2] - self.tiles[1] == 1) && (self.tiles[1] - self.tiles[0] == 1)
        }
        false
    }

    fn check_pair_combination(&self) -> (u8, bool){
        if self.tiles.len() == 2 {
            return (self.tiles[0], self.tiles[0] == self.tiles[1])
        }
        (0, false)
    }

    fn check_pong_combination(&self) -> (u8, bool){
        if self.tiles.len() == 3 {
            return(self.tiles[0],
            self.tiles[0] == self.tiles[1] && self.tiles[0] == self.tiles[2])
       }
       (0, false)
    }

    fn check_kong_combination(&self) -> (u8, bool){
        if self.tiles.len() == 4 {
            return(self.tiles[0],
                self.tiles[0] == self.tiles[1] && 
                self.tiles[0] == self.tiles[2] && self.tiles[0] == self.tiles[3])
        }
        (0, false)
    }

    fn is_123(&self) -> bool{
        if self.tiles.len() == 3 {
            if self.tiles[0] == 1 && self.tiles[1] == 2 && self.tiles[2] == 3 {
                return true;
            } else if self.tiles[0] == 11 && self.tiles[1] == 12 && self.tiles[2] == 13 {
                return true
            } else if self.tiles[0] == 21 && self.tiles[1] == 22 && self.tiles[2] == 23 {
                return true
            } else {
                return false;
            }
        }
        false
    }

    fn is_789(&self) -> bool{
        if self.tiles.len() == 3 {
            if self.tiles[0] == 7 && self.tiles[1] == 8 && self.tiles[2] == 9 {
                return true;
            } else if self.tiles[0] == 17 && self.tiles[1] == 18 && self.tiles[2] == 19 {
                return true
            } else if self.tiles[0] == 27 && self.tiles[1] == 28 && self.tiles[2] == 29 {
                return true
            } else {
                return false;
            }
        }
        false
    }
}

impl PlayerHand {
    // common functions 
    fn check_ignored (list_ignored: Vec<u8>, id: u8) -> bool {
        for ignore in list_ignored.iter(){
            if id == *ignore {
                true;
            }
        }
        false
    }

    fn return_result(name_list:Vec<String>, cur_name: &str, won_score: u8, 
        cur_score: u8, ignore_list: Vec<u8>, cur_ignore: &Vec<u8>) -> Result {
        Result { 
            winning_combinations: [name_list, vec![cur_name.to_string()]].concat(), 
            scores: won_score + cur_score,
            ignored_combinations: [ignore_list, cur_ignore.to_vec()].concat() 
        }
    }

    fn all_combo_same_character(grouped_tiles:&Vec<Combo>) -> (u8, bool) {
        let mut list_characters: Vec<u8> = Vec::new();
        
        // extract each combo character
        for combo in grouped_tiles.iter() {
            let (character, same_character) = combo.check_all_same_characters();
            if !same_character {
                return (0, false);
            }
            list_characters.push(character);
        }
    
        if list_characters.iter().unique().count() == 1 {
            return (list_characters[0], true);
        }
    
        (0, false)
    }

    fn decouple_combo(grouped_tiles:&Vec<Combo>) -> Vec<u8> {
        let mut list_of_tiles: Vec<u8> = Vec::new();
        for combo in grouped_tiles.iter() {
            list_of_tiles = [list_of_tiles, combo.tiles.to_vec()].concat() 
        }
        list_of_tiles
    }
    
    fn all_pair(grouped_tiles:&Vec<Combo>) -> bool {
    
        let tiles = Self::decouple_combo(grouped_tiles);
        if tiles.len() == 14 {
            if tiles.iter().unique().count() == 7 {
                return true
            }
        }
        false
    }
    
    fn same_vector(vector_one: &Vec<u8>, vector_two: &Vec<u8>) -> bool {
        if vector_one.len() == vector_two.len() {
            for index in 0..vector_one.len() -1  {
                if !(vector_one.contains(&vector_two[index])){
                    return false;
                }
            }
        }
        true
    }
    
    fn pong_related_extraction(grouped_tiles:&Vec<Combo>) -> (Vec<u8>, bool) {
        let mut list_characters: Vec<u8> = Vec::new();
        let mut all_pong: bool = true;
    
        for combo in grouped_tiles.iter() {
            if combo.tiles.len() == 3 {
                let (char, pong) = combo.check_pong_combination();
                if !pong {
                    all_pong = false;
                }
                list_characters.push(char);
            } else if combo.tiles.len() == 2 {
                let (char, pair) = combo.check_pair_combination();
                if !pair {
                    all_pong = false;
                }
                list_characters.push(char)
            }
        }
    
        (list_characters, all_pong)
    }
    
    fn all_kong(grouped_tiles:&Vec<Combo>) -> (u8, bool) {
        
        let mut kong_count: u8 = 0;
    
        for combo in grouped_tiles.iter() {
            if combo.tiles.len() == 4 {
                let (char, kong) = combo.check_kong_combination();
                if !kong {
                    return (0, false);
                }
                kong_count += 1;
            } else if combo.tiles.len() == 2 {
                let (char, pair) = combo.check_pair_combination();
                if !pair {
                    return (0, false);
                }
            } else {
                return (0, false);;
            }
        }
        (kong_count, true)
    }

    fn valid_winning_hand(grouped_tiles:&Vec<Combo>) -> bool {
        let mut valid_count: u8 = 0;
        for combo in grouped_tiles.iter() {
            if combo.tiles.len() == 3 {
                if combo.check_chi_combination(){
                    valid_count += 1;
                } else if combo.check_pong_combination().1 {
                    valid_count += 1;
                } else if combo.check_kong_combination().1 {
                    valid_count += 1;
                }
            }
            if combo.tiles.len() == 2 {
                if combo.check_pair_combination().1 {
                    valid_count += 1;
                }
            }
        }

        // must be 5 
        if valid_count == 5 {
            return true;
        }else {
            return false;
        }
    }

    // rules
    fn 大四喜(&self, result: Result, combination :Combinations) -> Result {

        if Self::check_ignored(result.ignored_combinations.clone(), combination.id){
            return result;
        }

        let (tiles, all_pong) = Self::pong_related_extraction(&self.grouped_tiles);

        // check 31, 32, 33, 34 in list
        if !(tiles.contains(& 31) && tiles.contains(& 32)
            && tiles.contains(& 33) && tiles.contains(& 34) 
            && all_pong){
                return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 大三元(&self,result: Result, combination :Combinations) -> Result {
        if Self::check_ignored(result.ignored_combinations.clone(), combination.id){
            return result;
        }

        let (tiles, _) = Self::pong_related_extraction(&self.grouped_tiles);

        if !(tiles.contains(& 35) && tiles.contains(& 36)
            && tiles.contains(& 37)){
                return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 绿一色(&self, result: Result, combination :Combinations) -> Result {

        for combo in self.grouped_tiles.iter() {
            for tile in combo.tiles.iter(){
                match tile {
                    12..=14 => {
                        continue;
                    }
                    16 => {
                        continue;
                    }
                    18 => {
                        continue;
                    }
                    36 => {
                        continue;
                    }
                    _=> {
                        return result;
                    }
                }
            }
        }
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 九莲宝灯(&self, result: Result, combination :Combinations) -> Result {
        let mut pong_of_one = false;
        let mut pong_of_nine = false;
        let mut chi_count = 0; // must be 2; 
        let mut pair_count = 0; // must be 1 

        let mut unique_tiles: Vec<u8> = Vec::new();

        // check if characters in combo is same
        for combo in self.grouped_tiles.iter() {
            let (char, pong) = combo.check_pong_combination();
            // check pong of one and nine
            if pong {
                match char {
                    1 => {
                        pong_of_one = !pong_of_one;
                    }
                    9 => {
                        pong_of_nine = !pong_of_nine;
                    }
                    11 => {
                        pong_of_one = !pong_of_one;
                    }
                    19 => {
                        pong_of_nine = !pong_of_nine;
                    }
                    21 => {
                        pong_of_one = !pong_of_one;
                    }
                    29 => {
                        pong_of_nine = !pong_of_nine;
                    }
                    _=> {
                    }
                }
            } else {
                // check if 234 567 8
                if combo.check_chi_combination() {
                    chi_count += 1;
                    unique_tiles = [unique_tiles, combo.tiles.to_vec()].concat();
                } else {
                    let (char, pair) = combo.check_pair_combination();
                    if pair {
                        pair_count += 1;
                        unique_tiles.push(char);
                    }
                }

            }
        }
    
        if !(chi_count == 2 && pair_count == 1 && pong_of_one && pong_of_nine && Self::all_combo_same_character(&self.grouped_tiles).1) {
            return result;
        }
    
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 四杠(&self, result: Result, combination: Combinations) -> Result {
        let mut kong_count = 0;

        for combo in self.grouped_tiles.iter() {
            let (char, kong) = combo.check_kong_combination();
            if kong {
                kong_count += 1;
            } else {
                return result;
            }
        }

        // if there is no 4 kong 
        if kong_count != 4 {
            return  result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 连七对(&self, result: Result, combination :Combinations) -> Result {
        // all same character
        if !(Self::all_combo_same_character(&self.grouped_tiles).1 && Self::all_pair(&self.grouped_tiles)) {
            return result;
        }
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 十三幺(&self, result: Result, combination :Combinations) -> Result {
        let unique: Vec<u8> = vec![1, 9, 11, 19, 21, 29, 31, 32, 33, 34, 35, 36, 37];

        // has all unique tiles + one of the duplicate 
        let combos = Self::decouple_combo(&self.grouped_tiles);
        let cleared_combos = combos.iter().unique();

        // duplicate should be +1 and must be 13 after unique clear
        if cleared_combos.count() != 13 {
            return result
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 清幺九(&self, result: Result, combination :Combinations) -> Result { 
        // let mut tiles: Vec<u8> = Vec::new();

        let (tiles, all_pong) = Self::pong_related_extraction(&self.grouped_tiles);

        // check if pong and pair are within 1-9 char
        let valid_list: Vec<u8> = vec![1, 9, 11, 19, 21, 29];
        if all_pong {
            for char in tiles.iter() {
                if !valid_list.contains(char) {
                    return result;
                }
            }
        } else {
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 小四喜(&self, result: Result, combination :Combinations) -> Result { 
        let valid_list: Vec<u8> = vec![31, 32, 33, 34];
        let mut valid_char_count: u8 = 0;
        let (tiles, _) = Self::pong_related_extraction(&self.grouped_tiles);

        for char in tiles.iter() {
            if !valid_list.contains(char) {
                return result;
            }
            valid_char_count += 1;
        }

        // only valid for 3 (31,32,33)
        if !(valid_char_count == 3){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 小三元(&self, result: Result, combination :Combinations) -> Result {

        // let mut tiles:Vec<u8> = Vec::new();
        let valid_list: Vec<u8> = vec![35, 36, 37];
        let mut valid_char_count: u8 = 0;

        let (tiles, _) = Self::pong_related_extraction(&self.grouped_tiles);

        for char in tiles.iter() {
            if !valid_list.contains(char) {
                return result;
            }
            valid_char_count += 1;
        }

        // only valid for 3 (31,32,33)
        if !(valid_char_count == 2){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 字一色(&self, result: Result, combination :Combinations) -> Result {
        let (char, same_character) = Self::all_combo_same_character(&self.grouped_tiles);

        if !(same_character && char == 30){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 四暗刻(&self, result: Result, combination :Combinations) -> Result {
        let (kong_count, all_kong) = Self::all_kong(&self.grouped_tiles);
        if !(all_kong && kong_count == 4){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 一色双龙会(&self, result: Result, combination: Combinations) -> Result {
        // condition: same character + 2 same consecutive (123, 789) of same color + normal pair
        let mut count_123: u8 = 0;
        let mut count_789: u8 = 0;

        // same characters 
        let (_, same_character) = Self::all_combo_same_character(&self.grouped_tiles);

        if !same_character {
            return result;
        }

        for combo in self.grouped_tiles.iter() {
            // check consecutive 
            if combo.check_chi_combination() {
                // is 123
                if combo.is_123(){
                    count_123 += 1;
                }
                // is 789
                if combo.is_789() {
                    count_789 += 1;
                }
            }
        }

        if !(count_123 == 2 && count_789 == 2) {
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 一色四同顺(&self, result: Result, combination: Combinations) -> Result {
        // same characters + 4 (same consecutive) + pair 
        let (_, same_character) = Self::all_combo_same_character(&self.grouped_tiles);
        let mut chi_count = 0;

        // check if same character 
        if !same_character {
            return result;
        }

        // check if chi is 4 
        for combo in self.grouped_tiles.iter() {
            if combo.check_chi_combination() {
                chi_count += 1;
            }
        }

        let tiles = Self::decouple_combo(&self.grouped_tiles);
        if !(chi_count == 4 && tiles.iter().unique().count() == 4){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 一色四节高(&self, result: Result, combination: Combinations) -> Result {
        // 4 pong + same characters + consecutive   
        let mut pong_count = 0;
        let mut pong_tiles: Vec<u8> = Vec::new();
        let (_, same_character) = Self::all_combo_same_character(&self.grouped_tiles);

        if !same_character {
            return result;
        }

        for combo in self.grouped_tiles.iter() {
            if combo.check_pong_combination().1 {
                pong_tiles.push(combo.tiles[0]);
                pong_count += 1;
            }
        }

        let (tiles, _) = Self::pong_related_extraction(&self.grouped_tiles);

        if !((tiles[3] - tiles[2] == 1) && (tiles[2] - tiles[1] == 1) 
            && (tiles[1] - tiles[0] == 1) && (pong_count == 4)) {
                return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 三杠(&self, result: Result, combination: Combinations) -> Result {
        let mut kong_count = 0;

        for combo in self.grouped_tiles.iter() {
            let (char, kong) = combo.check_kong_combination();
            if kong {
                kong_count += 1;
            } else {
                return result;
            }
        }

        // if there is no 4 kong 
        if kong_count != 3 {
            return  result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 混幺九(&self, result: Result, combination: Combinations) -> Result {
        // all 1,9 
        let tiles = Self::decouple_combo(&self.grouped_tiles);
        for tile in tiles.iter(){
            match tile {
                1 => {
                    continue;
                }
                9 => {
                    continue;
                }
                11 => {
                    continue;
                }
                19 => {
                    continue;
                }
                21 => {
                    continue;
                }
                29 => {
                    continue;
                }
                31..=37 => {
                    continue;
                }
                _=> {
                    return result;
                }
            }
        }
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 七对(&self, result: Result, combination: Combinations) -> Result {
        if !Self::all_pair(&self.grouped_tiles){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 七星不靠(&self, result: Result, combination: Combinations) -> Result {
        // must have 31 - 37, and combo of 
        let mut combo_word: Vec<u8> = Vec::new();
        // let mut combo_one: Vec<u8> = vec![vec![1, 4, 7], vec![11, 14, 17], vec![21, 24, 27]];
        let mut combo_one: Vec<u8> = Vec::new();
        // let mut combo_two: Vec<u8> = vec![vec![2, 5, 8], vec![12, 15, 18], vec![22, 25, 28]];
        let mut combo_two: Vec<u8> = Vec::new();
        // let mut combo_three: Vec<u8> = vec![vec![3, 6, 9], vec![13, 16, 19], vec![23, 26, 29]];
        let mut combo_three: Vec<u8> = Vec::new();
        let tiles = Self::decouple_combo(&self.grouped_tiles);

        if tiles.iter().unique().count() != 14 {
            return result;
        }

        for tile in tiles.iter() {
            match tile {
                1 => {
                    combo_one.push(*tile);
                }
                4 => {
                    combo_one.push(*tile);
                }
                7 => {
                    combo_one.push(*tile);
                }
                11 => {
                    combo_one.push(*tile);
                }
                14 => {
                    combo_one.push(*tile);
                }
                17 => {
                    combo_one.push(*tile);
                }
                21 => {
                    combo_one.push(*tile);
                }
                24 => {
                    combo_one.push(*tile);
                }
                27 => {
                    combo_one.push(*tile);
                }
                2 => {
                    combo_two.push(*tile);
                }
                5 => {
                    combo_two.push(*tile);
                }
                8 => {
                    combo_two.push(*tile);
                }
                12 => {
                    combo_two.push(*tile);
                }
                15 => {
                    combo_two.push(*tile);
                }
                18 => {
                    combo_two.push(*tile);
                }
                22 => {
                    combo_two.push(*tile);
                }
                25 => {
                    combo_two.push(*tile);
                }
                28 => {
                    combo_two.push(*tile);
                }
                3 => {
                    combo_three.push(*tile);
                }
                6 => {
                    combo_three.push(*tile);
                }
                9 => {
                    combo_three.push(*tile);
                }
                13 => {
                    combo_three.push(*tile);
                }
                16 => {
                    combo_three.push(*tile);
                }
                19 => {
                    combo_three.push(*tile);
                }
                23 => {
                    combo_three.push(*tile);
                }
                26 => {
                    combo_three.push(*tile);
                }
                29 => {
                    combo_three.push(*tile);
                }
                31..=37 => {
                    combo_word.push(*tile);
                }
                _=> {
                    return result;
                }
            }
        }

        if !(combo_word.len() == 7 && combo_one.len() <= 3 && combo_two.len() <= 3 && combo_three.len() <= 3) {
            return result;
        }
        
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 全双刻(&self, result: Result, combination: Combinations) -> Result {
        let valid_tiles: Vec<u8> = vec![2, 4, 6, 8, 12, 14, 16, 18, 22, 24, 26, 28];
        let (pong_tiles, all_pong) = Self::pong_related_extraction(&self.grouped_tiles);

        if !all_pong {
            return result;
        }

        for tile in pong_tiles.iter() {
            if !valid_tiles.contains(tile) {
                return result;
            }
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 清一色(&self, result: Result, combination: Combinations) -> Result {

        let (_, same_character) = Self::all_combo_same_character(&self.grouped_tiles);

        if !(same_character && Self::valid_winning_hand(&self.grouped_tiles)){
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 一色三同顺(&self, result: Result, combination: Combinations) -> Result {
        // same characters + 3 (same consecutive) + pair 
        let (_, same_character) = Self::all_combo_same_character(&self.grouped_tiles);
        let mut chi_count = 0;
        let mut pong_count = 0;
        let mut kong_count = 0;
        let mut chi_lists: Vec<u8> = Vec::new();

        // check if same character 
        if !same_character {
            return result;
        }

        for combo in self.grouped_tiles.iter() {
            if combo.check_chi_combination() {
                [chi_lists.clone(), combo.tiles.clone()].concat();
                chi_count += 1;
            }else if combo.check_kong_combination().1 {
                kong_count += 1;
            }else if combo.check_pong_combination().1 {
                pong_count += 1;
            }
        }
        // chi == 3
        if !(chi_count == 3 && (kong_count == 1 || pong_count == 1)){
            if !(chi_lists.iter().unique().count() == 3) {
                return result;
            }
        } else if !(chi_count == 4) {
            if !(chi_lists.iter().unique().count() == 6) {
                return result;
            }
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 全大(&self, result: Result, combination: Combinations) -> Result {
        if !Self::valid_winning_hand(&self.grouped_tiles) {
            return result;
        }
        let tiles = Self::decouple_combo(&self.grouped_tiles);
        for tile in tiles.iter() {
            match tile {
                7 => {
                    continue;
                }
                8 => {
                    continue;
                }
                9 => {
                    continue;
                }
                17 => {
                    continue;
                }
                18 => {
                    continue;
                }
                19 => {
                    continue;
                }
                27 => {
                    continue;
                }
                28 => {
                    continue;
                }
                29 => {
                    continue;
                }
                _ => {
                    return result;
                }
            }
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 全中(&self, result: Result, combination: Combinations) -> Result {
        if !Self::valid_winning_hand(&self.grouped_tiles) {
            return result;
        }
        let tiles = Self::decouple_combo(&self.grouped_tiles);
        for tile in tiles.iter() {
            match tile {
                4 => {
                    continue;
                }
                5 => {
                    continue;
                }
                6 => {
                    continue;
                }
                14 => {
                    continue;
                }
                15 => {
                    continue;
                }
                16 => {
                    continue;
                }
                24 => {
                    continue;
                }
                25 => {
                    continue;
                }
                26 => {
                    continue;
                }
                _ => {
                    return result;
                }
            }
        }
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 全小(&self, result: Result, combination: Combinations) -> Result {
        if !Self::valid_winning_hand(&self.grouped_tiles) {
            return result;
        }
        let tiles = Self::decouple_combo(&self.grouped_tiles);
        for tile in tiles.iter() {
            match tile {
                1 => {
                    continue;
                }
                2 => {
                    continue;
                }
                3 => {
                    continue;
                }
                11 => {
                    continue;
                }
                12 => {
                    continue;
                }
                13 => {
                    continue;
                }
                21 => {
                    continue;
                }
                22 => {
                    continue;
                }
                23 => {
                    continue;
                }
                _ => {
                    return result;
                }
            }
        }
        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn 清龙(&self, result: Result, combination: Combinations) -> Result {
        if !Self::valid_winning_hand(&self.grouped_tiles) {
            return result;
        }

        let mut 万_list: Vec<u8> = Vec::new();
        let mut 条_list: Vec<u8> = Vec::new();
        let mut 筒_list: Vec<u8> = Vec::new();
        let mut chi_count = 0;

        for combo in self.grouped_tiles.iter() {
            if combo.check_chi_combination() {
                if combo.tiles[0] < 10 {
                    for tile in combo.tiles.iter(){
                        筒_list.push(*tile);
                    }
                } else if combo.tiles[0] > 10 && combo.tiles[0] < 20 {
                    for tile in combo.tiles.iter(){
                        条_list.push(*tile);
                    }
                } else if combo.tiles[0] > 20 && combo.tiles[0] < 30 {
                    for tile in combo.tiles.iter(){
                        万_list.push(*tile);
                    }
                }
                chi_count += 1;
            }
        }

        if !(chi_count >= 3 && (筒_list.iter().unique().count() == 9 || 
        条_list.iter().unique().count() == 9 || 万_list.iter().unique().count() == 9)) {
            return result;
        }

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn hello(&self, result: Result, combination: Combinations) -> Result {

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }

    fn rules(&self, result: Result, combination: Combinations) -> Result {

        Self::return_result(result.winning_combinations, &combination.name, 
            result.scores, combination.score, 
            result.ignored_combinations, &combination.ignore)
    }
}