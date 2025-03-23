//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.0.0
//! DATE: 2025-03-23 (Y/M/D)
//! HOSTNAME: `kalan-x1x`, CPU: `12th Gen Intel(R) Core(TM) i7-12800H`
//!
//! SHORT-NAME: `block`, LONG-NAME: `BlockExecution`, RUNTIME: `liganite-runtime`
//! WARMUPS: `10`, REPEAT: `100`
//! WEIGHT-PATH: `./runtime/src/weights`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1.0`, WEIGHT-ADD: `0`

// Executed Command:
//   frame-omni-bencher
//   v1
//   benchmark
//   overhead
//   --runtime
//   ./target/production/wbuild/liganite-runtime/liganite_runtime.compact.compressed.wasm
//   --weight-path
//   ./runtime/src/weights

use sp_core::parameter_types;
use sp_weights::{constants::WEIGHT_REF_TIME_PER_NANOS, Weight};

parameter_types! {
    /// Weight of executing an empty block.
    /// Calculated by multiplying the *Average* with `1.0` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 76_867, 115_041
    ///   Average:  82_354
    ///   Median:   80_263
    ///   Std-Dev:  6610.2
    ///
    /// Percentiles nanoseconds:
    ///   99th: 114_282
    ///   95th: 92_754
    ///   75th: 83_755
    pub const BlockExecutionWeight: Weight =
        Weight::from_parts(WEIGHT_REF_TIME_PER_NANOS.saturating_mul(82_354), 0);
}

#[cfg(test)]
mod test_weights {
    use sp_weights::constants;

    /// Checks that the weight exists and is sane.
    // NOTE: If this test fails but you are sure that the generated values are fine,
    // you can delete it.
    #[test]
    fn sane() {
        let w = super::BlockExecutionWeight::get();

        // At least 100 µs.
        // assert!(
        // 	w.ref_time() >= 100u64 * constants::WEIGHT_REF_TIME_PER_MICROS,
        // 	"Weight should be at least 100 µs."
        // );
        // At most 50 ms.
        assert!(
            w.ref_time() <= 50u64 * constants::WEIGHT_REF_TIME_PER_MILLIS,
            "Weight should be at most 50 ms."
        );
    }
}
