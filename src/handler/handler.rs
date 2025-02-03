use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Handler {
    type K: Eq + Hash;
    type A: Asset;

    fn get_inner(&self) -> &HashMap<Self::K, Handle<Self::A>>;
    fn get_inner_mut(&mut self) -> &mut HashMap<Self::K, Handle<Self::A>>;

    fn add(&mut self, key: Self::K, asset: Handle<Self::A>) {
        self.get_inner_mut().insert(key, asset);
    }

    fn get(&self, key: Self::K) -> Handle<Self::A> {
        self.get_inner()
            .get(&key)
            .cloned()
            .expect("failed to get handle, hashmap must be initialized for every K")
    }
}
