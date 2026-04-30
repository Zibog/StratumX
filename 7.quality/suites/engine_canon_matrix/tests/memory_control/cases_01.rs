#[test]
fn allocations_are_tracked_by_id_and_pool() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 50,
    });

    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 60,
            layout_class: Some(LayoutClass::Sparse),
        })
        .expect("heap reserve");
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 20,
            layout_class: None,
        })
        .expect("staging reserve");

    let heap = service
        .allocation(MemoryAllocationId(1))
        .expect("heap allocation tracked");
    assert_eq!(heap.pool, MemoryAllocationPool::Heap);
    assert_eq!(heap.bytes_reserved, 60);
    assert_eq!(heap.layout_class, Some(LayoutClass::Sparse));

    let staging = service
        .allocation(MemoryAllocationId(2))
        .expect("staging allocation tracked");
    assert_eq!(staging.pool, MemoryAllocationPool::Staging);
    assert_eq!(staging.bytes_reserved, 20);

    let metrics = service.metrics();
    assert_eq!(metrics.heap_bytes, 60);
    assert_eq!(metrics.staging_bytes, 20);
    assert_eq!(metrics.allocation_count, 2);
    assert_eq!(metrics.pressure, PressureClass::Healthy);
}

#[test]
fn release_path_rejects_wrong_size_unknown_and_double_release() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });

    let descriptor = AllocationDescriptor {
        allocation_id: 7,
        bytes: 40,
        layout_class: None,
    };
    let receipt = service
        .reserve_with_receipt(&descriptor)
        .expect("reservation succeeds");

    let unknown = service
        .release_with_receipt(MemoryAllocationId(999), 40)
        .unwrap_err();
    assert_eq!(
        unknown,
        EngineCoreError::InvalidDescriptor("memory release references unknown allocation")
    );

    let wrong_size = service
        .release_with_receipt(receipt.allocation_id, 41)
        .unwrap_err();
    assert_eq!(
        wrong_size,
        EngineCoreError::InvalidDescriptor("memory release size does not match reservation")
    );

    let released = service
        .release_with_receipt(receipt.allocation_id, descriptor.bytes)
        .expect("matching release succeeds");
    assert_eq!(released.pool, MemoryAllocationPool::Heap);
    assert_eq!(released.bytes_released, 40);
    assert!(service.allocation(receipt.allocation_id).is_none());

    let typed_double_release = service
        .try_release_with_receipt(receipt.allocation_id, descriptor.bytes)
        .unwrap_err();
    assert_eq!(
        typed_double_release.reason(),
        MemoryFailureReason::AlreadyReleased
    );

    let double_release = service
        .release_with_receipt(receipt.allocation_id, descriptor.bytes)
        .unwrap_err();
    assert_eq!(
        double_release,
        EngineCoreError::InvalidDescriptor("memory allocation already released")
    );
}

#[test]
fn typed_memory_failures_cover_reservation_and_release_law() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 10,
    });

    let zero = service
        .try_reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 0,
            layout_class: None,
        })
        .unwrap_err();
    assert_eq!(zero.reason(), MemoryFailureReason::ZeroAllocation);

    let over_budget = service
        .try_reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 11,
            layout_class: None,
        })
        .unwrap_err();
    assert_eq!(over_budget.reason(), MemoryFailureReason::OverBudget);

    service
        .try_reserve_heap(&AllocationDescriptor {
            allocation_id: 3,
            bytes: 40,
            layout_class: None,
        })
        .expect("typed heap reservation succeeds");

    let duplicate = service
        .try_reserve_heap(&AllocationDescriptor {
            allocation_id: 3,
            bytes: 10,
            layout_class: None,
        })
        .unwrap_err();
    assert_eq!(
        duplicate.reason(),
        MemoryFailureReason::DuplicateAllocationId
    );

    let unknown = service
        .try_release_heap(MemoryAllocationId(999), 40)
        .unwrap_err();
    assert_eq!(unknown.reason(), MemoryFailureReason::UnknownAllocation);

    let pool_mismatch = service
        .try_release_staging(MemoryAllocationId(3), 40)
        .unwrap_err();
    assert_eq!(pool_mismatch.reason(), MemoryFailureReason::PoolMismatch);

    let size_mismatch = service
        .try_release_heap(MemoryAllocationId(3), 41)
        .unwrap_err();
    assert_eq!(size_mismatch.reason(), MemoryFailureReason::SizeMismatch);

    service
        .try_release_heap(MemoryAllocationId(3), 40)
        .expect("typed matching release succeeds");
    let already_released = service
        .try_release_heap(MemoryAllocationId(3), 40)
        .unwrap_err();
    assert_eq!(
        already_released.reason(),
        MemoryFailureReason::AlreadyReleased
    );
}

#[test]
fn already_released_bridge_message_is_not_unknown_allocation() {
    let bridge: EngineCoreError = engine_memory_control::MemoryFailure::new(
        MemoryFailureReason::AlreadyReleased,
        "memory allocation already released",
    )
    .into();

    let EngineCoreError::InvalidDescriptor(message) = bridge else {
        panic!("expected invalid descriptor bridge");
    };
    assert!(message.contains("already released"));
    assert!(!message.contains("unknown allocation"));
}

