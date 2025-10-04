use crate::{MAZE_HEIGHT_USIZE, MAZE_WIDTH_USIZE};
use heapless::Vec;
use crate::vec::Veci;

pub struct Map<V> {
    elements: Vec<Vec<V, MAZE_HEIGHT_USIZE>, MAZE_WIDTH_USIZE>,
}

impl<V: core::fmt::Debug> Map<V> {
    pub fn new() -> Self {
        Map {
            elements: Vec::new(),
        }
    }

    pub fn get(&self, vec: &Veci) -> Option<&V> {
        self.elements.get(vec.x as usize)?.get(vec.y as usize)
    }

    pub fn insert(&mut self, vec: Veci, value: V) -> Option<&V> {
        // Ensure the outer Vec has enough capacity
        while self.elements.len() <= vec.x as usize {
            self.elements.push(Vec::new()).ok()?;
        }

        let ys = self.elements.get_mut(vec.x as usize).unwrap();

        if (vec.y as usize) < ys.len() {
            ys[vec.y as usize] = value;
            Some(&ys[vec.y as usize])
        } else {
            None
        }
    }

    pub fn contains_key(&self, vec: &Veci) -> bool {
        self.elements
            .get(vec.x as usize)
            .and_then(|ys| ys.get(vec.y as usize))
            .is_some()
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