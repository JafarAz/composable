
//! Autogenerated weights for `oracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `oracle`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> oracle::WeightInfo for WeightInfo<T> {
	// Storage: Oracle AssetsCount (r:1 w:1)
	// Storage: Oracle RewardTrackerStore (r:1 w:1)
	// Storage: Oracle AssetsInfo (r:1 w:1)
	fn add_asset_and_info() -> Weight {
		(37_951_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Oracle RewardTrackerStore (r:1 w:1)
	fn adjust_rewards() -> Weight {
		(28_819_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:1)
	// Storage: Oracle SignerToController (r:1 w:1)
	// Storage: Oracle OracleStake (r:1 w:1)
	fn set_signer() -> Weight {
		(123_546_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:0)
	// Storage: Oracle OracleStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_stake() -> Weight {
		(102_884_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:0)
	// Storage: Oracle OracleStake (r:1 w:1)
	// Storage: Oracle DeclaredWithdraws (r:0 w:1)
	fn remove_stake() -> Weight {
		(39_752_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle ControllerToSigner (r:1 w:1)
	// Storage: Oracle DeclaredWithdraws (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Oracle SignerToController (r:0 w:1)
	fn reclaim_stake() -> Weight {
		(48_588_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Oracle OracleStake (r:1 w:0)
	// Storage: Oracle Prices (r:1 w:0)
	// Storage: Oracle AssetsInfo (r:1 w:0)
	// Storage: Oracle AnswerInTransit (r:1 w:1)
	// Storage: Oracle PrePrices (r:1 w:1)
	fn submit_price(p: u32, ) -> Weight {
		(60_828_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((190_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle PrePrices (r:1 w:1)
	// Storage: Oracle AnswerInTransit (r:1 w:1)
	fn update_pre_prices(p: u32, ) -> Weight {
		(16_228_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((205_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Oracle PriceHistory (r:1 w:1)
	// Storage: Oracle SignerToController (r:1 w:0)
	// Storage: Oracle AnswerInTransit (r:1 w:1)
	// Storage: Oracle RewardTrackerStore (r:1 w:0)
	// Storage: Oracle Prices (r:0 w:1)
	// Storage: Oracle PrePrices (r:0 w:1)
	fn update_price(p: u32, ) -> Weight {
		(35_450_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((5_307_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
