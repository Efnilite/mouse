use crate::vec::Vecu;
use crate::MAZE_SIZE;
use std::collections::HashMap;

/// Represents a custom Map implementation where the key is a [Vecu].
pub struct Map<V> {
    elements: HashMap<Vecu, V>,
}

impl<V: core::fmt::Debug> Map<V> {
    pub fn new() -> Self {
        Map {
            elements: HashMap::with_capacity(MAZE_SIZE),
        }
    }

    pub fn _get(&self, vec: &Vecu) -> Option<&V> {
        self.elements.get(vec)
    }

    pub fn insert(&mut self, vec: Vecu, value: V) -> Option<V> {
        self.elements.insert(vec, value)
    }

    pub fn contains_key(&self, vec: &Vecu) -> bool {
        self.elements.contains_key(vec)
    }
}

impl<V: core::fmt::Debug> Default for Map<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::map::Map;
    use crate::vec::Vecu;

    #[test]
    fn map() {
        let mut map: Map<u8> = Map::new();

        assert_eq!(None, map._get(&Vecu { x: 0, y: 0 }));
        assert_eq!(None, map.insert(Vecu { x: 0, y: 0 }, 1));
    }
}
