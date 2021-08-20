use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, mem};

#[derive(Default)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq
{
    fn get_bucket(&self, k: &K) -> Option<usize> {
        if self.buckets.is_empty() {
            return None
        }

        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        Some((hasher.finish() % self.buckets.len() as u64) as usize)
    }

    fn grow_buckets(&mut self) {
        let new_size = if self.buckets.is_empty() { 1 } else { self.buckets.len() * 2 };
        let mut new_buckets = Vec::with_capacity(new_size);

        // Fill the buckets with empty vectors
        new_buckets.extend((0..new_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        drop(mem::replace(&mut self.buckets, new_buckets));
    }

    pub fn new() -> Self {
        Self { buckets: Vec::new(), len: 0}
    }

    pub fn insert(&mut self, k: K, v: V) {
        if self.buckets.is_empty() {
            self.grow_buckets();
        }

        let bucket = self.get_bucket(&k).unwrap();
        self.buckets[bucket].push((k, v));
        self.len += 1;
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        if self.empty() {
            return None
        }

        let bucket = self.get_bucket(k)?;
        let value = self.buckets[bucket]
                                        .iter()
                                        .find(|&(ek, _ev)| k == ek);

        if let Some((_k, v)) = value {
            Some(v)
        } else {
            None
        }       
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        if self.empty() {
            return None
        }

        let bucket = self.get_bucket(k)?;
        let bucket = &mut self.buckets[bucket];
        let index = bucket
                                    .iter()
                                    .position(|(ek, _ev)| k == ek);

        if let Some(i) = index {
            self.len -= 1;
            Some(bucket.remove(i).1)
        } else {
            None
        }  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        let mut h = HashMap::new();
        assert_eq!(h.len(), 0);
        assert!(h.empty());
        
        h.insert("Hi!".to_string(), 23);
        assert_eq!(h.len(), 1);
        assert!(!h.empty());
        assert_eq!(h.get(&"Hi!".to_string()), Some(&23));

        assert_eq!(h.remove(&"Hi!".to_string()), Some(23));
        assert_eq!(h.len(), 0);
        assert!(h.empty());
        assert_eq!(h.get(&"Hi!".to_string()), None);
    }
}
