

use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use std::collections::BTreeMap;


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct NftData {
    pub total_supply: u64,
    pub mint_flag: bool,
    pub adventure_gap: u64,
    pub tokens: BTreeMap<u64, Principal>,
    pub controllers: Vec<Principal>,
    pub claim_index: u64,
    //+++++++++++++++++++++++++
    pub mintlimit: BTreeMap<Principal, u64>,
    pub xp: BTreeMap<u64, u64>,
    pub level: BTreeMap<u64, u64>,

    pub attribute_points: BTreeMap<u64, u64>,
    pub attr_strength: BTreeMap<u64, u64>,
    pub attr_agility: BTreeMap<u64, u64>,
    pub attr_intelligence: BTreeMap<u64, u64>,

    pub adventurers_log: BTreeMap<u64, u64>,
    //+++++++++++++++++++++++++++
    pub token_seeds: BTreeMap<u64, u64>
}


impl NftData {

    pub fn user_tokens(&self, user: &Principal) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();
        for (token_id, user_id) in &self.tokens {
            if user_id == user {
                results.push(*token_id);
            }
        }
        return results;
    }

    pub fn owner_of(&self, token_id: &u64) -> Option<Principal> {
        match self.tokens.get(token_id) {
            Some(owner_id) => return Some(owner_id.clone()),
            None => {
                return None;
            }
        }
    }

    pub fn is_owner_of(&self, user: Principal, token_id: &u64) -> bool {
        match self.owner_of(&token_id) {
            Some(owner_id) => return user == owner_id,
            None => {
                return false;
            }
        }
    }

    pub fn is_controller(&self, user: &Principal) -> bool {
        return self.controllers.contains(user);
    }

    pub fn add_controller(&mut self, user: &Principal) -> bool {
        if !self.is_controller(user) {
            self.controllers.push(user.clone());
            return true;
        }
        return false;
    }

    pub fn remove_controller(&mut self, user: &Principal) -> bool {
        if self.is_controller(user) {
            let index = self.controllers.iter().position(|x| x == user).unwrap();
            self.controllers.remove(index);
            return true;
        }
        return false;
    }

    pub fn transfer_to(&mut self, user: Principal, token_id: u64) -> bool {
        if let Some(token_owner) = self.tokens.get(&token_id) {
            if token_owner == &ic_cdk::caller() {
                self.tokens.insert(token_id, user);
                return true;
            }
        }
        return false;
    }

    pub fn remaining(&self) -> u64 {
        return self.total_supply - self.claim_index;
    }

    pub fn is_claimed(&self, token_id: &u64) -> bool {
        return self.tokens.contains_key(token_id);
    }


    pub fn claim(&mut self, user_id: Principal) -> Result<u64, String> {

        if self.claim_index >= self.total_supply {
            return Err("No more claims left".to_string());
        }
        match self.mintlimit.get(&user_id) {
            Some(_) => return Err("You have claimed one".to_string()),
            None => {
                self.claim_index += 1;

                match self.tokens.get(&self.claim_index) {
                    Some(_) => return Err("This token already claimed".to_string()),
                    None => {
                        
                        self.token_seeds.insert(self.claim_index, ic_cdk::api::time() as u64);
                        self.tokens.insert(self.claim_index, user_id);
                        self.level.insert(self.claim_index, 1 );
                        self.xp.insert(self.claim_index, 0 );
                        self.adventurers_log.insert(self.claim_index, 0 );
                        self.attribute_points.insert(self.claim_index, 5 );
                        let seed1 = (self.token_seeds.get(&self.claim_index).unwrap()) % 1000000000 + (self.claim_index)*( self.claim_index + 17) + self.claim_index;
                        let seed2 = seed1 * ( self.claim_index + 23 ) + self.claim_index + 67;
                        let seed3 = seed1 * ( self.claim_index) * ( self.claim_index + 61) + self.claim_index + 7 ;
                        ic_cdk::println!("seed1 {:?}", seed1);
                        ic_cdk::println!("seed2 {:?}", seed2);
                        ic_cdk::println!("seed3 {:?}", seed3);

                        let _strength  = (seed1  % 7 ) + 4 ;
                        let _agility  = (seed2  % 7 ) + 4 ;
                        let _intelligence  = (seed3  % 7 ) + 4 ;
                        self.attr_strength.insert(self.claim_index, _strength );
                        self.attr_agility.insert(self.claim_index, _agility );
                        self.attr_intelligence.insert(self.claim_index, _intelligence );

                        self.mintlimit.insert(user_id, 1);
                        return Ok(self.claim_index);
                    }
                }
            }
        }
    }


    pub fn adventure(&mut self, token_id: u64) -> Result<String, String> {
        
        if let Some(token_owner) = self.tokens.get(&token_id) {
            if token_owner == &ic_cdk::caller() {
                let mut _timelimit = ic_cdk::api::time() as u64;
                ic_cdk::println!("_timelimit , {:?}", _timelimit.to_string());
                if *self.adventurers_log.get(&token_id).unwrap() < _timelimit{

                    let mut _xp = self.xp.get(&token_id).unwrap();
                    let mut _level = self.level.get(&token_id).unwrap();
                    let mut _points = self.attribute_points.get(&token_id).unwrap();

                    self.adventurers_log.insert(token_id, _timelimit + self.adventure_gap);
                    let mut _xp2  = _xp + 150;
                    if _xp2 >= 1000{
                        _xp2  = _xp2 - 1000;
                        let mut _level2 =  _level + 1;
                        let mut _points2 =  _points + 3;
                        self.level.insert(token_id, _level2 );
                        self.attribute_points.insert(token_id, _points2);
                    }
                    self.xp.insert(token_id, _xp2 );
                    
                    return Ok("Adventure complete ! ".to_string());
                }else{
                    return Err("The interval hasn't come yet. have a rest".to_string());
                }
            }else{
                return Err("You are not the NFT owner!".to_string());
            }
        }else{
            return Err("Nobody own this NFT.".to_string());
        }

    }


    pub fn add_points(&mut self, token_id: u64,attribute: u64,amounts:u64) -> Result<String, String> {
            
        if let Some(token_owner) = self.tokens.get(&token_id) {
            if token_owner == &ic_cdk::caller() {
                let mut _points = *self.attribute_points.get(&token_id).unwrap();
                if (_points >= amounts) && (_points > 0) {
                    _points  = _points - amounts;
                    if attribute  == 0{
                        let old_attribute_amounts = *self.attr_strength.get(&token_id).unwrap();
                        self.attribute_points.insert(token_id, _points);
                        self.attr_strength.insert(token_id, old_attribute_amounts + amounts);
                        return Ok("Attributes(strength) alloc sucess ! ".to_string());
                    }else if attribute  == 1{
                            let old_attribute_amounts = *self.attr_agility.get(&token_id).unwrap();
                            self.attribute_points.insert(token_id, _points);
                            self.attr_agility.insert(token_id, old_attribute_amounts + amounts);
                        return Ok("Attributes(agility) alloc sucess ! ".to_string());
                    }else if  attribute  == 2{
                            let old_attribute_amounts = *self.attr_intelligence.get(&token_id).unwrap();
                            self.attribute_points.insert(token_id, _points);
                            self.attr_intelligence.insert(token_id, old_attribute_amounts +  amounts);
                            return Ok("Attributes(intelligence) alloc sucess ! ".to_string());
                    }else{
                        return Err("Please input true code !".to_string());
                    }
                    
                
                }else{
                    return Err("Your Attributes Points not enough !".to_string());
                }
            }else{
                return Err("You are not the NFT owner!".to_string());
            }
        }else{
            return Err("Nobody own this NFT.".to_string());
        }

    }


    //////////////////////////////
    pub fn mint_on(&mut self) -> bool {
        self.mint_flag = true;
        return self.mint_flag;
    }

    pub fn mint_off(&mut self) -> bool {
        self.mint_flag = false;
        return self.mint_flag;
    }

    pub fn get_mint_flag(&mut self) -> bool {
        return self.mint_flag;
    }
    //////////////////////////////

    pub fn querygap(&mut self, token_id: u64) -> u64 {
        return *self.adventurers_log.get(&token_id).unwrap();
    }

    pub fn queryxp(&mut self, token_id: u64) -> u64 {
            return *self.xp.get(&token_id).unwrap();
    }

    pub fn querypoint(&mut self, token_id: u64) -> u64 {
        return *self.attribute_points.get(&token_id).unwrap();
    }

    pub fn querylevel(&mut self, token_id: u64) -> u64 {
        return *self.level.get(&token_id).unwrap();
    }

    pub fn querycaller(&mut self,user_id: Principal) -> Option<Principal> {
        ic_cdk::println!("Caller is ->  {:?}", user_id.to_string());
        return Some(user_id.clone());
    }

    //++++++++++++++++++++++++++++++++++++++++


}
