// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		Weight::from_ref_time(19_637_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(32_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		Weight::from_ref_time(21_565_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(31_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		Weight::from_ref_time(21_637_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(46_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		Weight::from_ref_time(21_551_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(149_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:1)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		Weight::from_ref_time(22_510_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(41_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Membership Members (r:1 w:0)
	// Storage: Membership Prime (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(8_828_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(9_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Membership Prime (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(5_084_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
