use anchor_lang::prelude::*;

anchor_gen::generate_cpi_crate!("./idl.json");

declare_id!("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY");

pub const ASSET_PREFIX: &str = "asset";

pub fn get_asset_id(tree_id: &Pubkey, nonce: u64) -> Pubkey {
    Pubkey::find_program_address(
        &[
            ASSET_PREFIX.as_ref(),
            tree_id.as_ref(),
            &nonce.to_le_bytes(),
        ],
        &crate::id(),
    )
    .0
}

impl Default for LeafSchema {
    fn default() -> Self {
        Self::V1 {
            id: Default::default(),
            owner: Default::default(),
            delegate: Default::default(),
            nonce: 0,
            data_hash: [0; 32],
            creator_hash: [0; 32],
        }
    }
}

impl LeafSchema {
    pub fn new_v0(
        id: Pubkey,
        owner: Pubkey,
        delegate: Pubkey,
        nonce: u64,
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
    ) -> Self {
        LeafSchema::V1 {
            id,
            owner,
            delegate,
            nonce,
            data_hash,
            creator_hash,
        }
    }

    pub fn version(&self) -> Version {
        match self {
            LeafSchema::V1 { .. } => Version::V1,
        }
    }

    pub fn id(&self) -> Pubkey {
        match self {
            LeafSchema::V1 { id, .. } => *id,
        }
    }

    pub fn nonce(&self) -> u64 {
        match self {
            LeafSchema::V1 { nonce, .. } => *nonce,
        }
    }

    pub fn data_hash(&self) -> [u8; 32] {
        match self {
            LeafSchema::V1 { data_hash, .. } => *data_hash,
        }
    }
}

