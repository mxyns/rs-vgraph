use crate::converter::{ConversionGraph, Cost, Version};

pub fn version_diff<F>(_graph: &ConversionGraph<F>, from: Version, to: Version) -> Cost {
    (if from > to { from - to } else { to - from }) as Cost
}