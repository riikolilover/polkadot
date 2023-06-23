use crate::configuration::Config;
use frame_support::traits::OnRuntimeUpgrade;

// mod v7 {}

// pub struct MigrateToV6<T>(sp_std::marker::PhantomData<T>);
// impl<T: Config> OnRuntimeUpgrade for MigrateToV6<T> {
// 	fn on_runtime_upgrade() -> frame_support::weights::Weight {
// 		frame_support::weights::Weight::zero()
// 	}
// }

// Copyright (C) Parity Technologies (UK) Ltd.
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

///! A module that is responsible for migration of storage.
use crate::configuration::{self, Pallet};
use frame_support::{
	pallet_prelude::*,
	traits::{Defensive, StorageVersion},
	weights::Weight,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec::Vec;

use primitives::SessionIndex;
#[cfg(feature = "try-runtime")]
use sp_std::prelude::*;

use super::v6::V6HostConfiguration;
// Change this once there is V8.
type V7HostConfiguration<BlockNumber> = configuration::HostConfiguration<BlockNumber>;

mod v6 {
	use super::*;

	#[frame_support::storage_alias]
	pub(crate) type ActiveConfig<T: Config> =
		StorageValue<Pallet<T>, V6HostConfiguration<BlockNumberFor<T>>, OptionQuery>;

	#[frame_support::storage_alias]
	pub(crate) type PendingConfigs<T: Config> = StorageValue<
		Pallet<T>,
		Vec<(SessionIndex, V6HostConfiguration<BlockNumberFor<T>>)>,
		OptionQuery,
	>;
}

mod v7 {
	use super::*;

	#[frame_support::storage_alias]
	pub(crate) type ActiveConfig<T: Config> =
		StorageValue<Pallet<T>, V7HostConfiguration<BlockNumberFor<T>>, OptionQuery>;

	#[frame_support::storage_alias]
	pub(crate) type PendingConfigs<T: Config> = StorageValue<
		Pallet<T>,
		Vec<(SessionIndex, V7HostConfiguration<BlockNumberFor<T>>)>,
		OptionQuery,
	>;
}

pub struct MigrateToV7<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateToV7<T> {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		log::trace!(target: crate::configuration::LOG_TARGET, "Running pre_upgrade() for MigrateToV7");
		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> Weight {
		log::info!(target: configuration::LOG_TARGET, "MigrateToV7 started");
		if StorageVersion::get::<Pallet<T>>() == 6 {
			let weight_consumed = migrate_to_v7::<T>();

			log::info!(target: configuration::LOG_TARGET, "MigrateToV7 executed successfully");
			StorageVersion::new(7).put::<Pallet<T>>();

			weight_consumed
		} else {
			log::warn!(target: configuration::LOG_TARGET, "MigrateToV7 should be removed.");
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		log::trace!(target: crate::configuration::LOG_TARGET, "Running post_upgrade()");
		ensure!(
			StorageVersion::get::<Pallet<T>>() >= 7,
			"Storage version should be >= 7 after the migration"
		);

		Ok(())
	}
}

fn migrate_to_v7<T: Config>() -> Weight {
	// Unusual formatting is justified:
	// - make it easier to verify that fields assign what they supposed to assign.
	// - this code is transient and will be removed after all migrations are done.
	// - this code is important enough to optimize for legibility sacrificing consistency.
	#[rustfmt::skip]
	let translate =
		|pre: V6HostConfiguration<BlockNumberFor<T>>| ->
		V7HostConfiguration<BlockNumberFor<T>>
	{
		V7HostConfiguration {
max_code_size                            : pre.max_code_size,
max_head_data_size                       : pre.max_head_data_size,
max_upward_queue_count                   : pre.max_upward_queue_count,
max_upward_queue_size                    : pre.max_upward_queue_size,
max_upward_message_size                  : pre.max_upward_message_size,
max_upward_message_num_per_candidate     : pre.max_upward_message_num_per_candidate,
hrmp_max_message_num_per_candidate       : pre.hrmp_max_message_num_per_candidate,
validation_upgrade_cooldown              : pre.validation_upgrade_cooldown,
validation_upgrade_delay                 : pre.validation_upgrade_delay,
max_pov_size                             : pre.max_pov_size,
max_downward_message_size                : pre.max_downward_message_size,
hrmp_max_paras_outbound_channels     : pre.hrmp_max_parachain_outbound_channels,
hrmp_sender_deposit                      : pre.hrmp_sender_deposit,
hrmp_recipient_deposit                   : pre.hrmp_recipient_deposit,
hrmp_channel_max_capacity                : pre.hrmp_channel_max_capacity,
hrmp_channel_max_total_size              : pre.hrmp_channel_max_total_size,
hrmp_max_paras_inbound_channels      : pre.hrmp_max_parachain_inbound_channels,
hrmp_channel_max_message_size            : pre.hrmp_channel_max_message_size,
code_retention_period                    : pre.code_retention_period,
parathread_cores                         : pre.parathread_cores,
parathread_retries                       : pre.parathread_retries,
group_rotation_frequency                 : pre.group_rotation_frequency,
paras_availability_period                : pre.chain_availability_period,
scheduling_lookahead                     : pre.scheduling_lookahead,
max_validators_per_core                  : pre.max_validators_per_core,
max_validators                           : pre.max_validators,
dispute_period                           : pre.dispute_period,
dispute_post_conclusion_acceptance_period: pre.dispute_post_conclusion_acceptance_period,
no_show_slots                            : pre.no_show_slots,
n_delay_tranches                         : pre.n_delay_tranches,
zeroth_delay_tranche_width               : pre.zeroth_delay_tranche_width,
needed_approvals                         : pre.needed_approvals,
relay_vrf_modulo_samples                 : pre.relay_vrf_modulo_samples,
pvf_checking_enabled                     : pre.pvf_checking_enabled,
pvf_voting_ttl                           : pre.pvf_voting_ttl,
minimum_validation_upgrade_delay         : pre.minimum_validation_upgrade_delay,
async_backing_params                     : pre.async_backing_params,
executor_params                          : pre.executor_params,
		}
	};

	let v6 = v6::ActiveConfig::<T>::get()
		.defensive_proof("Could not decode old config")
		.unwrap_or_default();
	let v7 = translate(v6);
	v7::ActiveConfig::<T>::set(Some(v7));

	let pending_v6 = v6::PendingConfigs::<T>::get()
		.defensive_proof("Could not decode old pending")
		.unwrap_or_default();
	let mut pending_v7 = Vec::new();

	for (session, v6) in pending_v6.into_iter() {
		let v7 = translate(v6);
		pending_v7.push((session, v7));
	}
	v7::PendingConfigs::<T>::set(Some(pending_v7.clone()));

	let num_configs = (pending_v7.len() + 1) as u64;
	T::DbWeight::get().reads_writes(num_configs, num_configs)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		configuration::HostConfiguration,
		mock::{new_test_ext, Test},
	};

	#[test]
	fn v7_deserialized_from_actual_data() {
		// Example how to get new `raw_config`:
		// We'll obtain the raw_config at a specified a block
		// Steps:
		// 1. Go to Polkadot.js -> Developer -> Chain state -> Storage: https://polkadot.js.org/apps/#/chainstate
		// 2. Set these parameters:
		//   2.1. selected state query: configuration; activeConfig(): PolkadotRuntimeParachainsConfigurationHostConfiguration
		//   2.2. blockhash to query at: 0xf89d3ab5312c5f70d396dc59612f0aa65806c798346f9db4b35278baed2e0e53 (the hash of the block)
		//   2.3. Note the value of encoded storage key -> 0x06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385 for the referenced block.
		//   2.4. You'll also need the decoded values to update the test.
		// 3. Go to Polkadot.js -> Developer -> Chain state -> Raw storage
		//   3.1 Enter the encoded storage key and you get the raw config.

		// This exceeds the maximal line width length, but that's fine, since this is not code and
		// doesn't need to be read and also leaving it as one line allows to easily copy it.
		let raw_config = hex_literal::hex!["0000300000800000080000000000100000c8000005000000050000000200000002000000000000000000000000005000000010000400000000000000000000000000000000000000000000000000000000000000000000000800000000200000040000000000100000b0040000000000000000000014000000040000000000000001010000000006000000640000000200000019000000000000000200000002000000000200000005000000"];

		let v7 =
			HostConfiguration::<primitives::BlockNumber>::decode(&mut &raw_config[..]).unwrap();

		// We check only a sample of the values here. If we missed any fields or messed up data types
		// that would skew all the fields coming after.
		assert_eq!(v7.max_code_size, 3_145_728);
		assert_eq!(v7.validation_upgrade_cooldown, 2);
		assert_eq!(v7.max_pov_size, 5_242_880);
		assert_eq!(v7.hrmp_channel_max_total_size, 8_192);
		assert_eq!(v7.hrmp_max_paras_outbound_channels, 4);
		assert_eq!(v7.paras_availability_period, 4);
		assert_eq!(v7.n_delay_tranches, 25);
		assert_eq!(v7.minimum_validation_upgrade_delay, 5); // This is the last field in the struct.
		assert_eq!(v7.group_rotation_frequency, 20);
	}

	#[test]
	fn test_migrate_to_v7() {
		// Host configuration has lots of fields. However, in this migration we only remove one field.
		// The most important part to check are a couple of the last fields. We also pick
		// extra fields to check arbitrarily, e.g. depending on their position (i.e. the middle) and
		// also their type.
		//
		// We specify only the picked fields and the rest should be provided by the `Default`
		// implementation. That implementation is copied over between the two types and should work
		// fine.
		let v6 = V6HostConfiguration::<primitives::BlockNumber> {
			needed_approvals: 69,
			thread_availability_period: 55,
			hrmp_recipient_deposit: 1337,
			max_pov_size: 1111,
			chain_availability_period: 33,
			minimum_validation_upgrade_delay: 20,
			..Default::default()
		};

		let mut pending_configs = Vec::new();
		pending_configs.push((100, v6.clone()));
		pending_configs.push((300, v6.clone()));

		new_test_ext(Default::default()).execute_with(|| {
			// Implant the v6 version in the state.
			v6::ActiveConfig::<Test>::set(Some(v6.clone()));
			v6::PendingConfigs::<Test>::set(Some(pending_configs));

			migrate_to_v7::<Test>();

			let v7 = v7::ActiveConfig::<Test>::get().unwrap();
			let mut configs_to_check = v7::PendingConfigs::<Test>::get().unwrap();
			configs_to_check.push((0, v7.clone()));

			for (_, v7) in configs_to_check {
				#[rustfmt::skip]
				{
					assert_eq!(v7.max_code_size                            , v6.max_code_size);
					assert_eq!(v7.max_head_data_size                       , v6.max_head_data_size);
					assert_eq!(v7.max_upward_queue_count                   , v6.max_upward_queue_count);
					assert_eq!(v7.max_upward_queue_size                    , v6.max_upward_queue_size);
					assert_eq!(v7.max_upward_message_size                  , v6.max_upward_message_size);
					assert_eq!(v7.max_upward_message_num_per_candidate     , v6.max_upward_message_num_per_candidate);
					assert_eq!(v7.hrmp_max_message_num_per_candidate       , v6.hrmp_max_message_num_per_candidate);
					assert_eq!(v7.validation_upgrade_cooldown              , v6.validation_upgrade_cooldown);
					assert_eq!(v7.validation_upgrade_delay                 , v6.validation_upgrade_delay);
					assert_eq!(v7.async_backing_params                     , v6.async_backing_params);
					assert_eq!(v7.max_pov_size                             , v6.max_pov_size);
					assert_eq!(v7.max_downward_message_size                , v6.max_downward_message_size);
					assert_eq!(v7.hrmp_max_paras_outbound_channels         , v6.hrmp_max_parachain_outbound_channels);
					assert_eq!(v7.hrmp_sender_deposit                      , v6.hrmp_sender_deposit);
					assert_eq!(v7.hrmp_recipient_deposit                   , v6.hrmp_recipient_deposit);
					assert_eq!(v7.hrmp_channel_max_capacity                , v6.hrmp_channel_max_capacity);
					assert_eq!(v7.hrmp_channel_max_total_size              , v6.hrmp_channel_max_total_size);
					assert_eq!(v7.hrmp_max_paras_inbound_channels          , v6.hrmp_max_parachain_inbound_channels);
					assert_eq!(v7.hrmp_channel_max_message_size            , v6.hrmp_channel_max_message_size);
					assert_eq!(v7.executor_params                          , v6.executor_params);
					assert_eq!(v7.code_retention_period                    , v6.code_retention_period);
					assert_eq!(v7.parathread_cores                         , v6.parathread_cores);
					assert_eq!(v7.parathread_retries                       , v6.parathread_retries);
					assert_eq!(v7.group_rotation_frequency                 , v6.group_rotation_frequency);
					assert_eq!(v7.paras_availability_period                , v6.chain_availability_period);
					assert_eq!(v7.scheduling_lookahead                     , v6.scheduling_lookahead);
					assert_eq!(v7.max_validators_per_core                  , v6.max_validators_per_core);
					assert_eq!(v7.max_validators                           , v6.max_validators);
					assert_eq!(v7.dispute_period                           , v6.dispute_period);
					assert_eq!(v7.dispute_post_conclusion_acceptance_period, v6.dispute_post_conclusion_acceptance_period);
					assert_eq!(v7.no_show_slots                            , v6.no_show_slots);
					assert_eq!(v7.n_delay_tranches                         , v6.n_delay_tranches);
					assert_eq!(v7.zeroth_delay_tranche_width               , v6.zeroth_delay_tranche_width);
					assert_eq!(v7.needed_approvals                         , v6.needed_approvals);
					assert_eq!(v7.relay_vrf_modulo_samples                 , v6.relay_vrf_modulo_samples);
					assert_eq!(v7.pvf_checking_enabled                     , v6.pvf_checking_enabled);
					assert_eq!(v7.pvf_voting_ttl                           , v6.pvf_voting_ttl);
					assert_eq!(v7.minimum_validation_upgrade_delay         , v6.minimum_validation_upgrade_delay);
				}; // ; makes this a statement. `rustfmt::skip` cannot be put on an expression.
			}
		});
	}
}