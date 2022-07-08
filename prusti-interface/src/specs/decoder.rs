use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::sync::{Lrc, MetadataRef};
use rustc_hir::def_id::{CrateNum, DefId, DefIndex, DefPathHash};
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_middle::ty::codec::TyDecoder;
pub use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::opaque;
use rustc_session::StableCrateId;

#[derive(Clone)]
pub struct DefSpecsBlob(pub Lrc<MetadataRef>);

// This is only safe to decode the metadata of a single crate or the `ty_rcache` might confuse shorthands (see #360)
pub struct DefSpecsDecoder<'a, 'tcx> {
    opaque: opaque::Decoder<'a>,
    tcx: TyCtxt<'tcx>,
    ty_rcache: FxHashMap<usize, Ty<'tcx>>,
}

impl<'a, 'tcx> DefSpecsDecoder<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, blob: &'a DefSpecsBlob) -> Self {
        DefSpecsDecoder {
            opaque: opaque::Decoder::new(&blob.0, 0),
            tcx,
            ty_rcache: Default::default(),
        }
    }

    // From rustc
    fn def_path_hash_to_def_id(&self, hash: DefPathHash) -> DefId {
        self.tcx.def_path_hash_to_def_id(hash, &mut || panic!("DefPathHash not found in the local crate"))
    }
}

// the following two instances are from rustc
// This impl makes sure that we get a runtime error when we try decode a
// `DefIndex` that is not contained in a `DefId`. Such a case would be problematic
// because we would not know how to transform the `DefIndex` to the current
// context.
impl<'a, 'tcx> Decodable<DefSpecsDecoder<'a, 'tcx>> for DefIndex {
    fn decode(_: &mut DefSpecsDecoder<'a, 'tcx>) -> DefIndex {
        panic!("trying to decode `DefIndex` outside the context of a `DefId`")
    }
}

// Both the `CrateNum` and the `DefIndex` of a `DefId` can change in between two
// compilation sessions. We use the `DefPathHash`, which is stable across
// sessions, to map the old `DefId` to the new one.
impl<'a, 'tcx> Decodable<DefSpecsDecoder<'a, 'tcx>> for DefId {
    fn decode(d: &mut DefSpecsDecoder<'a, 'tcx>) -> Self {
        let def_path_hash = DefPathHash::decode(d);
        d.def_path_hash_to_def_id(def_path_hash)
    }
}

impl<'a, 'tcx> Decodable<DefSpecsDecoder<'a, 'tcx>> for CrateNum {
    fn decode(d: &mut DefSpecsDecoder<'a, 'tcx>) -> CrateNum {
        let stable_id = StableCrateId::decode(d);
        d.tcx.stable_crate_id_to_crate_num(stable_id)
    }
}

//implement_ty_decoder!(DefSpecDecoder<'a, 'tcx>);

macro_rules! decoder_methods {
    ($($name:ident -> $ty:ty;)*) => {
        $(
            #[inline]
            fn $name(&mut self) -> $ty {
                self.opaque.$name()
            }
        )*
    }
}


impl<'a, 'tcx> Decoder for DefSpecsDecoder<'a, 'tcx> {
    decoder_methods! {
        read_u128 -> u128;
        read_u64 -> u64;
        read_u32 -> u32;
        read_u16 -> u16;
        read_u8 -> u8;
        read_usize -> usize;

        read_i128 -> i128;
        read_i64 -> i64;
        read_i32 -> i32;
        read_i16 -> i16;
        read_i8 -> i8;
        read_isize -> isize;

        read_bool -> bool;
        read_f64 -> f64;
        read_f32 -> f32;
        read_char -> char;
        read_str -> &str;
    }

    #[inline]
    fn read_raw_bytes(&mut self, len: usize) -> &[u8] {
        self.opaque.read_raw_bytes(len)
    }
}

impl<'a, 'tcx> TyDecoder<'tcx> for DefSpecsDecoder<'a, 'tcx> {
    const CLEAR_CROSS_CRATE: bool = true;

    #[inline]
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    #[inline]
    fn peek_byte(&self) -> u8 {
        self.opaque.data[self.opaque.position()]
    }

    #[inline]
    fn position(&self) -> usize {
        self.opaque.position()
    }

    fn cached_ty_for_shorthand<F>(&mut self, shorthand: usize, or_insert_with: F) -> Ty<'tcx>
        where
            F: FnOnce(&mut Self) -> Ty<'tcx>,
    {
        if let Some(&ty) = self.ty_rcache.get(&shorthand) {
            return ty;
        }

        let ty = or_insert_with(self);
        self.ty_rcache.insert(shorthand, ty);
        ty
    }

    fn with_position<F, R>(&mut self, pos: usize, f: F) -> R
        where
            F: FnOnce(&mut Self) -> R,
    {
        let new_opaque = opaque::Decoder::new(self.opaque.data, pos);
        let old_opaque = std::mem::replace(&mut self.opaque, new_opaque);
        let r = f(self);
        self.opaque = old_opaque;
        r
    }

    fn decode_alloc_id(&mut self) -> rustc_middle::mir::interpret::AllocId {
        unimplemented!("decode_alloc_id")
    }
}
