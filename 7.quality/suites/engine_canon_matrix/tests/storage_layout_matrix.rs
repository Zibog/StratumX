#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn storage_layout_chunkdense_valid_strategy() -> impl Strategy<Value = usize> {
    0usize..25
}

fn storage_layout_chunkdense_missing_chunk_strategy() -> impl Strategy<Value = usize> {
    0usize..15
}

fn storage_layout_columnar_missing_columns_strategy() -> impl Strategy<Value = usize> {
    0usize..15
}

fn storage_layout_columnar_valid_strategy() -> impl Strategy<Value = usize> {
    0usize..20
}

proptest! {
    #[test]
    fn storage_layout_all_chunkdense_valid_cases(|case in storage_layout_chunkdense_valid_strategy()) {
        let sig_count = match case % 4 {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 4,
        };
        let signature: smallvec::SmallVec<[ComponentTypeId; 8]> = (1..=sig_count as u64)
            .map(ComponentTypeId)
            .collect();
        let d = StorageLayoutDescriptor {
            layout_class: LayoutClass::ChunkDense,
            chunk: Some(ChunkDescriptor {
                signature,
                access_mode: ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE,
                invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
            }),
            columns: smallvec![],
            locality_class: LocalityClass::TraversalLane,
        };
        prop_assert!(d.check_invariants().is_ok());
    }

    #[test]
    fn storage_layout_all_chunkdense_missing_chunk_cases(|case in storage_layout_chunkdense_missing_chunk_strategy()) {
        let _case = case;
        let d = StorageLayoutDescriptor {
            layout_class: LayoutClass::ChunkDense,
            chunk: None,
            columns: smallvec![],
            locality_class: LocalityClass::TraversalLane,
        };
        prop_assert!(d.check_invariants().is_err());
    }

    #[test]
    fn storage_layout_all_columnar_missing_columns_cases(|case in storage_layout_columnar_missing_columns_strategy()) {
        let _case = case;
        let d = StorageLayoutDescriptor {
            layout_class: LayoutClass::Columnar,
            chunk: None,
            columns: smallvec![],
            locality_class: LocalityClass::Cache,
        };
        prop_assert!(d.check_invariants().is_err());
    }

    #[test]
    fn storage_layout_all_columnar_valid_cases(|case in storage_layout_columnar_valid_strategy()) {
        let fam_count = match case % 3 {
            0 => 1,
            1 => 2,
            _ => 3,
        };
        let family: smallvec::SmallVec<[ComponentTypeId; 8]> = (1..=fam_count as u64)
            .map(ComponentTypeId)
            .collect();
        let d = StorageLayoutDescriptor {
            layout_class: LayoutClass::Columnar,
            chunk: None,
            columns: smallvec![ColumnDescriptor { family }],
            locality_class: LocalityClass::Partition,
        };
        prop_assert!(d.check_invariants().is_ok());
    }
}
