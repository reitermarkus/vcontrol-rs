use std::{
  collections::{BTreeMap, BTreeSet},
  fmt::Debug,
};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UniqueMapping<K, V> {
  pub mapping: BTreeMap<K, usize>,
  pub translations: BTreeMap<usize, V>,
}

impl<K, V> UniqueMapping<K, V> {
  pub fn create(map: BTreeMap<K, V>) -> Self
  where
    K: Debug + Clone + Ord + AsRef<str>,
    V: Debug + Ord,
  {
    let mut reversed = BTreeMap::new();
    for (k, v) in map {
      let keys = reversed.entry(v).or_insert_with(BTreeSet::new);
      keys.insert(k);
    }

    let mut mapping = BTreeMap::<K, usize>::new();
    let mut combined = BTreeMap::<usize, V>::new();

    let mut id_map = BTreeMap::<K, usize>::new();
    let mut id = 0;

    for (k, v) in reversed {
      let id_key = (&v).iter().max_by_key(|s| s.as_ref().len()).unwrap();
      let new_id = if let Some(existing_id) = id_map.get(id_key) {
        *existing_id
      } else {
        let new_id = id;
        id_map.insert(id_key.clone(), new_id);
        id += 1;
        new_id
      };

      for old_id in v {
        mapping.insert(old_id, new_id);
      }

      combined.insert(new_id, k);
    }

    Self { mapping, translations: combined }
  }
}
