use std::array;
use std::hash::{Hash, Hasher};

use crate::display::CHROMATICITY_EPSILON;

#[derive(Clone, Copy, Default, Debug)]
pub struct Chromaticity(pub(crate) [f32; 2]);

impl PartialEq for Chromaticity {
    fn eq(&self, other: &Self) -> bool {
        let dx = (self.0[0] - other.0[0]).abs();
        let dy = (self.0[1] - other.0[1]).abs();

        dx <= CHROMATICITY_EPSILON && dy <= CHROMATICITY_EPSILON
    }
}

impl Eq for Chromaticity {}

impl Hash for Chromaticity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[0].to_bits().hash(state);
        self.0[1].to_bits().hash(state);
    }
}

impl IntoIterator for Chromaticity {
    type Item = f32;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
