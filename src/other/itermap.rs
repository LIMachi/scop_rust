use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

///stupid optimization of a hashmap to make iteration over values a lot faster
///(the idea is: for an increased cost of insert/remove, we try to squeeze as much speed as possible on iterations)
///iteration order is not guaranteed at all! (should stay mostly in order of insertion, but removes will shuffle some values around)
pub struct IterMap<K: Eq + Hash + Clone, V> {
    map: HashMap<K, usize>,
    vec: Vec<(K, V)>
}

impl <K: Eq + Hash + Clone, V> IterMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new()
        }
    }
    
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    
    pub fn clear(&mut self) {
        self.vec.clear();
        self.map.clear();
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
    
    ///less than twice the time of the standard map
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(&i) = self.map.get(&key) {
            Some(std::mem::replace(&mut self.vec[i].1, value))
        } else {
            let i = self.vec.len();
            self.map.insert(key.clone(), i);
            self.vec.push((key, value));
            None
        }
    }
    
    ///less than twice the time of the standard map
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(i) = self.map.remove(key) {
            let l = self.vec.len() - 1;
            if i != l {
                let p = self.vec.pop().unwrap();
                self.map.insert(p.0.clone(), i);
                Some(std::mem::replace(&mut self.vec[i], p))
            } else {
                self.vec.pop()
            }.map(|(_, v)| v)
        } else {
            None
        }
    }
    
    ///almost identical speed
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|i| &self.vec[*i].1)
    }

    ///almost identical speed
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get(key).map(|i| &mut self.vec[*i].1)
    }

    ///four times faster than iterating the standard map
    pub fn iter_values(&self) -> impl Iterator<Item = &V> {
        self.vec.iter().map(|(_, v)| v)
    }

    ///four times faster than iterating the standard map
    pub fn iter_values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.vec.iter_mut().map(|(_, v)| v)
    }
    
    ///four times faster than iterating the standard map
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.vec.iter()
    }

    ///four times faster than iterating the standard map
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.vec.iter_mut()
    }
}

impl <K: Eq + Hash + Clone + Debug, V: Debug> Debug for IterMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.vec.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::time;
    use super::IterMap;
    
    #[test]
    fn test() {
        let mut map = IterMap::new();
        map.insert("test".to_string(), 2);
        map.insert("ok".to_string(), 1);
        map.insert("ter".to_string(), 3);
        dbg!(map.remove(&"ok".to_string()));
        map.insert("back".to_string(), 4);
        dbg!(&map);
        dbg!(map.iter_values().collect::<Vec<&i32>>());
        let mut amap = HashMap::new();
        time!(for i in 0..10000 {
            map.insert(format!("t{i}"), i + 100);
        });
        time!(for i in 0..10000 {
            amap.insert(format!("t{i}"), i + 100);
        });
        time!(for i in 0..100 {
            map.remove(&format!("t{}", i + 1000));
        });
        time!(for i in 0..100 {
            amap.remove(&format!("t{}", i + 1000));
        });
        time!(map.iter_values().for_each(|_| ()));
        time!(amap.values().for_each(|_| ()));
        time!(for i in 0..100 {
            map.get(&format!("t{}", i + 100));
        });
        time!(for i in 0..100 {
            amap.get(&format!("t{}", i + 100));
        });
        time!(for i in 0..100 {
            map.get_mut(&format!("t{}", i + 300));
        });
        time!(for i in 0..100 {
            amap.get_mut(&format!("t{}", i + 300));
        });
        time!(map.iter_values_mut().for_each(|_| ()));
        time!(amap.values_mut().for_each(|_| ()));
        time!(map.iter().for_each(|_| ()));
        time!(amap.iter().for_each(|_| ()));
        time!(map.iter_mut().for_each(|_| ()));
        time!(amap.iter_mut().for_each(|_| ()));
    }
}