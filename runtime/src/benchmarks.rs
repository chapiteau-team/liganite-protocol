frame_benchmarking::define_benchmarks!(
    [frame_benchmarking, BaselineBench::<Runtime>]
    [frame_system, SystemBench::<Runtime>]
    [frame_system_extensions, SystemExtensionsBench::<Runtime>]
    [pallet_balances, Balances]
    [pallet_timestamp, Timestamp]
    [pallet_transaction_payment, TransactionPayment]
    [pallet_sudo, Sudo]
    [liganite_publish, Publish]
    [liganite_games, Games]
);
