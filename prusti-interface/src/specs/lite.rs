use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;

use log::warn;

use rustc_data_structures::owning_ref::OwningRef;
use rustc_data_structures::rustc_erase_owner;
use rustc_data_structures::sync::Lrc;
use rustc_hir::def_id::DefId;
use rustc_macros::{TyDecodable, TyEncodable};
use rustc_middle::mir;
use rustc_middle::ty::TyCtxt;

use crate::rustc_serialize::{Decodable, Encodable};
use crate::specs::typed::{
    DefSpecificationMap, ProcedureSpecification, PrustiAssertion, PrustiAssumption,
    SpecGraph, TypeSpecification,
};

use super::{
    decoder::{DefSpecsBlob, DefSpecsDecoder},
    encoder::DefSpecsEncoder,
};

/// A subset of [DefSpecificationMap]. Used for serialization of
/// specification data (of the currently compiled crate), loaded by dependent crates
/// that import external specification (from the current crate).
#[derive(TyEncodable, TyDecodable)]
pub struct DefSpecificationMapLite<'tcx> {
    proc_specs: HashMap<DefId, SpecGraph<ProcedureSpecification>>,
    type_specs: HashMap<DefId, TypeSpecification>,
    prusti_assertions: HashMap<DefId, PrustiAssertion>,
    prusti_assumptions: HashMap<DefId, PrustiAssumption>,

    local_mirs: HashMap<DefId, Rc<mir::Body<'tcx>>>,
}

impl<'tcx> DefSpecificationMapLite<'tcx> {
    pub(crate) fn from_def_spec(def_spec: &DefSpecificationMap<'tcx>) -> Self {
        Self {
            // TODO: avoid cloning if possible
            proc_specs: def_spec.proc_specs.clone(),
            type_specs: def_spec.type_specs.clone(),
            prusti_assertions: def_spec.prusti_assertions.clone(),
            prusti_assumptions: def_spec.prusti_assumptions.clone(),

            local_mirs: def_spec.local_mirs.clone(),
        }
    }

    pub(crate) fn read_from_file(tcx: TyCtxt<'tcx>, path: &Path) -> io::Result<Self> {
        let mut encoded = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut encoded)?;

        let encoded_owned = OwningRef::new(encoded);
        let metadat_ref: OwningRef<Box<_>, [u8]> = encoded_owned.map_owner_box();
        let blob = DefSpecsBlob(Lrc::new(rustc_erase_owner!(metadat_ref)));

        let mut decoder = DefSpecsDecoder::new(tcx, &blob);
        let def_spec_lite = Self::decode(&mut decoder);

        Ok(def_spec_lite)
    }

    pub(crate) fn extend(self, def_spec: &mut DefSpecificationMap<'tcx>) {
        def_spec.proc_specs.extend(self.proc_specs);
        def_spec.type_specs.extend(self.type_specs);
        def_spec.prusti_assertions.extend(self.prusti_assertions);
        def_spec.prusti_assumptions.extend(self.prusti_assumptions);

        def_spec.local_mirs.extend(self.local_mirs);
    }

    pub(crate) fn write_into_file(self, tcx: TyCtxt<'tcx>, path: &Path) -> Result<(), io::Error> {
        let mut encoder = DefSpecsEncoder::new(tcx);
        self.encode(&mut encoder).unwrap();

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        File::create(path).and_then(|mut file| file.write(&encoder.into_inner())).map_err(|err| {
            warn!("could not encode metadata for crate `{:?}`, error: {:?}", "LOCAL_CRATE", err);
            err
        })?;
        Ok(())
    }
}
