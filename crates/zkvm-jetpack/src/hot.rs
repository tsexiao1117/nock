use either::Either::*;
use nockvm::jets::hot::{HotEntry, K_138};
use crate::jets::*;

/// Jet 路径统一类型（可自定义为更通用结构）
pub type JetPathElem = either::Either<&'static [u8], &'static str>;

/// Jet 注册信息结构体，便于批量、并行处理
pub struct JetSpec {
    pub path: &'static [JetPathElem],
    pub arity: usize,
    pub func: fn(),
}

/// Jet 表统一收敛，便于批量注册和遍历
pub static JET_TABLE: &[JetSpec] = &[
    // XTRA_JETS
    JetSpec {
        path: &[
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"weld"),
        ],
        arity: 1,
        func: jets::mary_weld_jet,
    },
    JetSpec {
        path: &[
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"swag"),
        ],
        arity: 1,
        func: jets::mary_swag_jet,
    },
    JetSpec {
        path: &[
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"proof-lib"),
            Left(b"utils"),
            Left(b"fri"),
            Left(b"table-lib"),
            Left(b"stark-core"),
            Left(b"fock-core"),
            Left(b"pow"),
            Left(b"stark-engine"),
            Left(b"stark-verifier"),
            Left(b"evaluate-deep"),
        ],
        arity: 1,
        func: jets::evaluate_deep_jet,
    },
    JetSpec {
        path: &[
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"transpose"),
        ],
        arity: 1,
        func: jets::mary_transpose_jet,
    },
    // 可继续补充 EXTENSION_FIELD_JETS、BASE_FIELD_JETS、BASE_POLY_JETS、ZTD_JETS、KEYGEN_JETS、CURVE_JETS 等
];

/// 并行友好的批量查找（可选用 rayon 并行查找大表）
pub fn find_jet_by_path(path: &[JetPathElem]) -> Option<&'static JetSpec> {
    JET_TABLE.iter().find(|js| js.path == path)
    // 使用 rayon:
    // use rayon::prelude::*;
    // JET_TABLE.par_iter().find_any(|js| js.path == path)
}

/// 批量注册接口（通用）
pub fn register_all_jets<F: Fn(&JetSpec)>(register: F) {
    for jet in JET_TABLE {
        register(jet);
    }
}

/// 兼容原有接口，批量收集 HotEntry
pub fn produce_prover_hot_state() -> Vec<HotEntry> {
    JET_TABLE
        .iter()
        .map(|js| (js.path, js.arity, js.func))
        .collect()
}

/// jets 子模块（建议实际实现放 jets.rs 各 Jet 具体实现）
mod jets {
    pub fn mary_weld_jet() { /* ... */ }
    pub fn mary_swag_jet() { /* ... */ }
    pub fn evaluate_deep_jet() { /* ... */ }
    pub fn mary_transpose_jet() { /* ... */ }
    // ... 其他 Jet 实现
}
