use std::collections::HashMap;
use std::sync::Arc;

use crate::balance::default::DefaultLoadBalance;
use crate::balance::LoadBalanceBox;
use crate::LoadBalanceType;
use crate::balance::random::RandomLoadBalance;

pub fn get_all_load_balances() -> HashMap<LoadBalanceType, LoadBalanceBox> {
    let mut maps: HashMap<LoadBalanceType, LoadBalanceBox> = HashMap::new();
    maps.insert(LoadBalanceType::Default, Arc::new(Box::new(DefaultLoadBalance::new())));
    maps.insert(LoadBalanceType::Random, Arc::new(Box::new(RandomLoadBalance::new())));
    maps.insert(LoadBalanceType::Robin, Arc::new(Box::new(DefaultLoadBalance::new())));
    maps.insert(LoadBalanceType::Hash, Arc::new(Box::new(DefaultLoadBalance::new())));
    maps.insert(LoadBalanceType::Weighted, Arc::new(Box::new(DefaultLoadBalance::new())));
    maps.insert(LoadBalanceType::LRT, Arc::new(Box::new(DefaultLoadBalance::new())));
    return maps;
}
