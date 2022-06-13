use prusti_serialize::metadata::BinaryMetadata;
use crate::specs::typed::DefSpecificationMap;

use rustc_hir::def_id::{CrateNum, DefId, DefIndex};

pub fn metadata_from_def_spec(
    def_spec: &DefSpecificationMap
) -> BinaryMetadata {
    let keys = def_spec.proc_specs.clone().into_keys();
    let l: Vec<DefId> = keys.into_iter().collect();
    BinaryMetadata {
        terms: l
    }
}
