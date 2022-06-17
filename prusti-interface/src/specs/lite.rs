use crate::specs::typed::{
    DefSpecificationMap, ProcedureSpecification, SpecGraph, TypeSpecification,
    PrustiAssertion, PrustiAssumption
};

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use log::warn;

use rustc_macros::TyEncodable;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::{DefId, LocalDefId};

use prusti_serialize::encoder::DefSpecsEncoder;

use crate::rustc_serialize::Encodable;

/// A subset of [DefSpecificationMap]. Used for serialization of
/// specification data (of the currently compiled crate), loaded by dependent crates
/// that import external specification (from the current crate).
#[derive(TyEncodable)]
pub struct DefSpecificationMapLite {
    pub proc_specs: HashMap<DefId, SpecGraph<ProcedureSpecification>>,
    pub type_specs: HashMap<DefId, TypeSpecification>,
    pub prusti_assertions: HashMap<DefId, PrustiAssertion>,
    pub prusti_assumptions: HashMap<DefId, PrustiAssumption>,
}

impl DefSpecificationMapLite {
    pub(crate) fn from_def_spec(
        def_spec: &DefSpecificationMap
    ) -> DefSpecificationMapLite {
        DefSpecificationMapLite {
            // TODO: avoid cloning if possible
            proc_specs: def_spec.proc_specs.clone(),
            type_specs: def_spec.type_specs.clone(),
            prusti_assertions: def_spec.prusti_assertions.clone(),
            prusti_assumptions: def_spec.prusti_assumptions.clone(),
        }
    }
}

pub fn dump_def_specs_lite<'tcx>(
    tcx: TyCtxt<'tcx>,
    path: &Path,
    dep_info: DefSpecificationMapLite,
) -> Result<(), std::io::Error> {
    let mut encoder = DefSpecsEncoder::new(tcx);
    dep_info.encode(&mut encoder).unwrap();

    File::create(path).and_then(|mut file| file.write(&encoder.into_inner())).map_err(|err| {
        warn!("could not encode metadata for crate `{:?}`, error: {:?}", "LOCAL_CRATE", err);
        err
    })?;
    Ok(())
}
