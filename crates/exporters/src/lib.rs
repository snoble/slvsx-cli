#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "dxf")]
pub mod dxf;

#[cfg(feature = "slvs")]
pub mod slvs;

#[cfg(feature = "stl")]
pub mod stl;

use slvsx_core::ir::ResolvedEntity;
use std::collections::HashMap;

pub trait Exporter {
    fn export(&self, entities: &HashMap<String, ResolvedEntity>) -> anyhow::Result<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Test that the module compiles - trait objects have size of 2 pointers
        assert_eq!(
            std::mem::size_of::<&dyn Exporter>(),
            std::mem::size_of::<[usize; 2]>()
        );
    }
}
