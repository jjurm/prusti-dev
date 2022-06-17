use crate::specs::typed::{DefSpecificationMap, ProcedureSpecification, SpecGraph};

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use log::warn;

use rustc_macros::TyEncodable;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::{DefId, LocalDefId};

use prusti_serialize::encoder::MetadataEncoder;

use crate::rustc_serialize::Encodable;

#[derive(TyEncodable)]
pub struct BinaryMetadata {
    pub proc_specs: HashMap<DefId, SpecGraph<ProcedureSpecification>>,
}

impl BinaryMetadata {}

pub fn dump_binary_metadata<'tcx>(
    tcx: TyCtxt<'tcx>,
    path: &Path,
    dep_info: BinaryMetadata,
) -> Result<(), std::io::Error> {
    let mut encoder = MetadataEncoder::new(tcx);
    dep_info.encode(&mut encoder).unwrap();

    File::create(path).and_then(|mut file| file.write(&encoder.into_inner())).map_err(|err| {
        warn!("could not encode metadata for crate `{:?}`, error: {:?}", "LOCAL_CRATE", err);
        err
    })?;
    Ok(())
}


pub fn metadata_from_def_spec(
    def_spec: &DefSpecificationMap
) -> BinaryMetadata {
    BinaryMetadata {
        proc_specs: def_spec.proc_specs.clone()
    }
}
