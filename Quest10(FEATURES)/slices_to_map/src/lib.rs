use std::{collections::HashMap, hash::Hash};

pub fn slices_to_map<'a, T: Eq + Hash, U>(sl1: &'a [T], sl2: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map: HashMap<&'a T, &'a U> = HashMap::new();
    let smallest = sl1.len().min(sl2.len());
    for i in 0..smallest {
        map.insert(&sl1[i], &sl2[i]);
    }
    map
}
