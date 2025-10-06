use crate::vec::Veci;
use crate::MAZE_SIZE;
use std::collections::HashMap;

/// Represents a custom Map implementation where the key is a [Veci].
pub struct Map<V> {
    elements: HashMap<Veci, V>,
}

impl<V: core::fmt::Debug> Map<V> {
    pub fn new() -> Self {
        Map {
            elements: HashMap::with_capacity(MAZE_SIZE),
        }
    }

    pub fn get(&self, vec: &Veci) -> Option<&V> {
        self.elements.get(vec)
    }

    pub fn insert(&mut self, vec: Veci, value: V) -> Option<V> {
        self.elements.insert(vec, value)
    }

    pub fn contains_key(&self, vec: &Veci) -> bool {
        self.elements.contains_key(vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::map::Map;
    use crate::vec::Veci;

    #[test]
    fn map() {
        let mut map: Map<u8> = Map::new();

        assert_eq!(None, map.get(&Veci { x: 0, y: 0 }));
        assert_eq!(None, map.insert(Veci { x: 0, y: 0 }, 1));
    }

}