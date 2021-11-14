
use ic_cdk::export::{candid::{CandidType, Deserialize}};
use crate::rando::Rand;

use ic_cdk::storage;
use crate::nft::NftData;


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Rush {
    pub classes: Vec<String>,
    pub races: Vec<String>,
    pub faction: Vec<String>,
    pub weapons: Vec<String>,
    pub chest: Vec<String>,
    pub waist: Vec<String>,
    pub head: Vec<String>,
    pub foot: Vec<String>,
    pub accessory: Vec<String>,
    pub prefixes: Vec<String>,

    pub suffixes: Vec<String>,//X
    pub name_prefixes: Vec<String>,//X
    pub name_suffixes: Vec<String>,//X


    
}

impl Rush {
    pub fn get_classes(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute2(&self.classes, seed + 14311, token_id)
    }

    pub fn get_races(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute2(&self.races, seed + 31761, token_id)
    }

    pub fn get_faction(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute2(&self.faction, seed + 13421, token_id)
    }

    pub fn get_level(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.level.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_xp(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.xp.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_attribute_points(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.attribute_points.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_attr_strength(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.attr_strength.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_attr_agility(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.attr_agility.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_attr_intelligence(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.attr_intelligence.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_adventurers_log(&self, token_id: u64) -> String {
        let _token_id:&u64 =  &token_id;
        let nft_data = storage::get_mut::<NftData>();
        let x = nft_data.adventurers_log.get(_token_id).unwrap();
        return x.to_string();
    }

    pub fn get_weapon(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let mut seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute(&self.weapons, seed + 32973, token_id)
    }

    pub fn get_chest(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let mut seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute(&self.chest, seed + 218971, token_id)
    }

    pub fn get_head(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let mut seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute(&self.head, seed + 18237, token_id)
    }

    pub fn get_waist(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute(&self.waist, seed + 43411, token_id)
    }
    pub fn get_foot(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute(&self.foot, seed + 69832, token_id)
    }
    pub fn get_accessory(&self, token_id: u64) -> String {
        let nft_data = storage::get_mut::<NftData>();
        let mut seed = (*nft_data.token_seeds.get(&token_id).unwrap()) % 1000000000;
        self.compute(&self.accessory, seed +17233, token_id)
    }


    pub fn get_prefix(&self, rand: u64) -> String {
        return self.prefixes[rand as usize % &self.prefixes.len()].clone();
    }

    pub fn get_name_prefix(&self, rand: u64) -> String {
        return self.name_prefixes[rand as usize % &self.name_prefixes.len()].clone();
    }

    pub fn get_name_suffix(&self, rand: u64) -> String {
        return self.name_suffixes[rand as usize % &self.name_suffixes.len()].clone();
    }

    pub fn compute(&self, items: &Vec<String>, offset: u64, token_id: u64) -> String {

        let rand = Rand::new(token_id + offset).rand();
        let item_index = rand as usize % items.len();

        let mut output = items[item_index].clone();

        let greatness = rand % 21;

        if greatness > 14 {
            output = format!("({}) {}",self.get_prefix(rand), output);
        } 

        if greatness >= 18 {
            if greatness == 18 {
                output = format!("{} 🔥", output);
            } else if greatness == 19{
                      output = format!("{} ❄️", output);
                    }else if greatness == 20{
                            //output = format!("{}", output);
                    }
        } 
        return output;
    }


    pub fn compute2(&self, items: &Vec<String>, offset: u64, token_id: u64) -> String {

        let rand = Rand::new(token_id + offset).rand();
        let item_index = rand as usize % items.len();

        let output = items[item_index].clone();
        return output;
    }

    pub fn get_properties(&self, token_id: u64) -> Vec<(String, String)> {

        return vec![
            ("classes".to_string(), self.get_classes(token_id)),
            ("races".to_string(), self.get_races(token_id)),
            ("faction".to_string(), self.get_faction(token_id)),

            ("level".to_string(), self.get_level(token_id)),
            ("xp".to_string(), self.get_xp(token_id)),
            ("attribute_points".to_string(), self.get_attribute_points(token_id)),
            ("attr_strength".to_string(), self.get_attr_strength(token_id)),
            ("attr_agility".to_string(), self.get_attr_agility(token_id)),
            ("attr_intelligence".to_string(), self.get_attr_intelligence(token_id)),
            ("adventurers_log".to_string(), self.get_adventurers_log(token_id)),

            ("hand".to_string(), self.get_weapon(token_id)),
            ("body".to_string(), self.get_chest(token_id)),
            ("head".to_string(), self.get_head(token_id)),
            ("waist".to_string(), self.get_waist(token_id)),
            ("foot".to_string(), self.get_foot(token_id)),
            ("accessory".to_string(), self.get_accessory(token_id)),
        ]
    }

    pub fn generate(&self, token_id: u64) -> String {
        return format!(r#"
                <svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMinYMin meet" viewBox="0 0 420 420">
                    <style>
                    .base {{ fill: white; font-family: HelveticaNeue-Bold, Helvetica Neue; font-size: 14px; }} 
                    .classes {{ fill: yellow; font-family: HelveticaNeue-Bold, Helvetica Neue; font-size: 20px; }}
                    .slash {{ fill: red; font-family: HelveticaNeue-Bold, Helvetica Neue; font-size: 20px; }}
                    .equipments {{ fill: white; font-family: HelveticaNeue-Bold, Helvetica Neue; font-size: 14px; }}
                    .attributes {{ fill: olive; font-family: HelveticaNeue-Bold, Helvetica Neue; font-size: 14px; }}
                    </style>
                    <rect width="100%" height="100%"/>
                    <text x="10" y="30" class="classes">
                    {}
                    </text>
                    <text x="10" y="60" class="base">
                    Level: {}
                    </text>
                    <text x="10" y="80" class="base">
                    Races: {}
                    </text>
                    <text x="10" y="100" class="base">
                    Faction: {}
                    </text>
                    <text x="10" y="120" class="slash">
                    -----------
                    </text>
                    <text x="10" y="140" class="equipments">
                    {}
                    </text>
                    <text x="10" y="160" class="equipments">
                    {}
                    </text>
                    <text x="10" y="180" class="equipments">
                    {}
                    </text>
                    <text x="10" y="200" class="equipments">
                    {}
                    </text>
                    <text x="10" y="220" class="equipments">
                    {}
                    </text>
                    <text x="10" y="240" class="equipments">
                    {}
                    </text>
                    <text x="10" y="260" class="slash">
                    -----------
                    </text>
                    <text x="10" y="280" class="attributes">
                    Strength: {}
                    </text>
                    <text x="10" y="300" class="attributes">
                    Agility: {}
                    </text>
                    <text x="10" y="320" class="attributes">
                    Intelligence: {}
                    </text>
                </svg>
            "#, 
            self.get_classes(token_id), 
            self.get_level(token_id), 
            self.get_races(token_id),
            self.get_faction(token_id),
            self.get_weapon(token_id), 
            self.get_chest(token_id), 
            self.get_head(token_id),
            self.get_waist(token_id), 
            self.get_foot(token_id), 
            self.get_accessory(token_id),
            self.get_attr_strength(token_id),
            self.get_attr_agility(token_id),
            self.get_attr_intelligence(token_id),

        );
    }
}