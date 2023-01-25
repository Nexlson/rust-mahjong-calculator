use std::collections::HashMap;
use calculator::Combinations;

pub fn tiles_biding() -> HashMap<String, u8>{
    HashMap::from([
        (String::from("一筒"), 1),
        (String::from("二筒"), 2),
        (String::from("三筒"), 3),
        (String::from("四筒"), 4),
        (String::from("五筒"), 5),
        (String::from("六筒"), 6),
        (String::from("七筒"), 7),
        (String::from("八筒"), 8),
        (String::from("九筒"), 9),

        (String::from("一条"), 11),
        (String::from("二条"), 12),
        (String::from("三条"), 13),
        (String::from("四条"), 14),
        (String::from("五条"), 15),
        (String::from("六条"), 16),
        (String::from("七条"), 17),
        (String::from("八条"), 18),
        (String::from("九条"), 19),

        (String::from("一万"), 21),
        (String::from("二万"), 22),
        (String::from("三万"), 23),
        (String::from("四万"), 24),
        (String::from("五万"), 25),
        (String::from("六万"), 26),
        (String::from("七万"), 27),
        (String::from("八万"), 28),
        (String::from("九万"), 29),

        (String::from("东风"), 31),
        (String::from("南风"), 32),
        (String::from("西风"), 33),
        (String::from("北风"), 34),
        (String::from("红中"), 35),
        (String::from("发财"), 36),
        (String::from("白班"), 37),

        (String::from("花牌"), 41),
        (String::from("鬼牌"), 42),
    ])
}

pub fn characters_biding() -> HashMap<String, u8>{
    HashMap::from([
        (String::from("筒"), 0),
        (String::from("条"), 10),
        (String::from("万"), 20),
        (String::from("字"), 30),
    ])
}

pub fn element_biding() -> HashMap<String, u8>{
    HashMap::from([
        (String::from("碰"), 50),
        (String::from("吃"), 51),
        (String::from("杠"), 52),
    ])
}

pub fn generate_combinations() -> Vec<Combinations>{
    vec![
        Combinations{
            name: String::from("大四喜"),
            id: 1,
            score: 88,
            ignore: vec![],
            example: vec![31, 31, 31, 32, 32, 32, 33, 33, 33, 34, 34, 34, 1, 1],
            descriptions: String::from("和牌型中，有东南西北4副刻（杠）子。不计圈风刻、门风刻、三风刻、碰碰和、混一色。")
        },

        Combinations{
            name: String::from("大三元"),
            id: 2,
            score: 88,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌型中，有中发白3副刻（杠）子。不计箭刻、双箭刻。")
        },

        Combinations{
            name: String::from("绿一色"),
            id: 3,
            score: 88,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由23468条及发中的任何牌组成顺子、刻子、将的和牌。如有“发”组成的牌，可计混一色；如无“发”字组成的牌，可计清一色、断幺。")
        },

        Combinations{
            name: String::from("九莲宝灯"),
            id: 4,
            score: 88,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由一种花色序数牌子按1112345678999组成的听牌型，见同花色任何1张序数牌即成和牌。不计清一色、门前清。")
        },

        Combinations{
            name: String::from("四杠"),
            id: 5,
            score: 88,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌型中，有四个杠子。不计碰碰和、三杠、双暗杠、双明杠、明杠、暗杠、单钓将。")
        },

        Combinations{
            name: String::from("连七对"),
            id: 5,
            score: 88,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由同一种花色的序数牌组成序数相连的7个对子的特殊和牌型。不计清一色、门前清、单钓将、连六、一般高。若为2345678的连七对可加计断幺九")
        },

        Combinations{
            name: String::from("十三幺"),
            id: 5,
            score: 88,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("含有3种序数牌的一、九牌，7种字牌且其中一种成对组成的特殊和牌型。不计混幺九、五门齐、门前清、单钓将。")
        },

        Combinations{
            name: String::from("清幺九"),
            id: 6,
            score: 64,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由序数牌一、九刻子组成的和牌。不计碰碰和、三同刻、双同刻、无字")
        },

        Combinations{
            name: String::from("小四喜"),
            id: 7,
            score: 64,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌时有风牌的3副刻子及将牌。不计三风刻、混一色")
        },

        Combinations{
            name: String::from("小三元"),
            id: 8,
            score: 64,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌时有箭牌的两副刻子及将牌。不计箭刻、双箭刻")
        },

        Combinations{
            name: String::from("字一色"),
            id: 9,
            score: 64,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由字牌的刻子(杠)、将组成的和牌。不计碰碰和")
        },

        Combinations{
            name: String::from("四暗刻"),
            id: 10,
            score: 64,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("4个暗刻(暗杠)。不计门前清、碰碰和")
        },

        Combinations{
            name: String::from("一色双龙会"),
            id: 11,
            score: 64,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("一种花色的两个老少副，5为将牌。不计平和、七对、清一色")
        },
        
        Combinations{
            name: String::from("一色四同顺"),
            id: 12,
            score: 48,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("一种花色4副序数相同的顺子，不计一色三节高、一般高、四归一、平胡")
        },

        Combinations{
            name: String::from("一色四节高"),
            id: 13,
            score: 48,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("一种花色4副依次递增一位数的刻子不计一色三同顺、碰碰和")
        },

        Combinations{
            name: String::from("一色四步高"),
            id: 14,
            score: 32,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("一种花色4副依次递增一位数或依次递增二位数的顺子 。不计平胡")
        },

        Combinations{
            name: String::from("三杠"),
            id: 15,
            score: 32,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("3个杠。不计双暗杠，双明杠，暗杠，明杠")
        },

        Combinations{
            name: String::from("混幺九"),
            id: 16,
            score: 32,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由字牌和序数牌一、九的刻子用将牌组成的和牌。不计碰碰和")
        },

        Combinations{
            name: String::from("七对"),
            id: 17,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由7个对子组成和牌。不计门前清、单钓")
        },

        Combinations{
            name: String::from("七星不靠"),
            id: 18,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("必须有7个单张的东西南北中发白，加上3种花色，数位按147、258、369中的7张序数牌组成没有将牌的和牌。不计五门齐、门前清、单钓")
        },

        Combinations{
            name: String::from("全双刻"),
            id: 19,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由2、4、6、8序数牌的刻子、将牌组成的和牌。不计碰碰和、断幺")
        },

        Combinations{
            name: String::from("清一色"),
            id: 20,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由一种花色的序数牌组成和各牌。不计无字")
        },

        Combinations{
            name: String::from("一色三同顺"),
            id: 21,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌时有一种花色3副序数相同的顺子。不计一色三节高。")
        },

        Combinations{
            name: String::from("全大"),
            id: 22,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由序数牌789组成的顺子、刻子(杠)、将牌的和牌。不计无字")
        },

        Combinations{
            name: String::from("全中"),
            id: 23,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由序数牌456组成的顺子、刻子(杠)、将牌的和牌。不计断幺、无字")
        },

        Combinations{
            name: String::from("全小"),
            id: 24,
            score: 24,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("由序数牌123组成的顺子、刻子(杠)、将牌的和牌。不计无字")
        },

        Combinations{
            name: String::from("清龙"),
            id: 25,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌时，有一种花色1-9相连接的序数牌")
        },

        // here
        Combinations{
            name: String::from("三色双龙会"),
            id: 26,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("种花色2个老少副、另一种花色5作将的和牌。不计喜相逢、老少副、无字、平和")
        },

        Combinations{
            name: String::from("一色三步高"),
            id: 27,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("和牌时，有一种花色3副依次递增一位或依次递增二位数字的顺子")
        },

        Combinations{
            name: String::from("全带五"),
            id: 28,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("每副牌及将牌必须有5的序数牌。不计断幺")
        },

        Combinations{
            name: String::from("三同刻"),
            id: 29,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("3个序数相同的刻子(杠)")
        },

        Combinations{
            name: String::from("三暗刻"),
            id: 30,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("3个暗刻。不计双暗刻")
        },

        Combinations{
            name: String::from(""),
            id: 24,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("")
        },

        Combinations{
            name: String::from(""),
            id: 24,
            score: 16,
            ignore: vec![],
            example: vec![],
            descriptions: String::from("")
        },

    ]
}