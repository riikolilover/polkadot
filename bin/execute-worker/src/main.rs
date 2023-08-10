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

//! Execute worker.

#[cfg(test)]
use polkadot_execute_worker::BINARY_NAME;

polkadot_node_core_pvf_common::decl_worker_main!(
	"execute-worker",
	polkadot_node_core_pvf_execute_worker::worker_entrypoint,
	env!("SUBSTRATE_CLI_IMPL_VERSION")
);

#[test]
fn name_test() {
	// If binary name ever needs to change, it must be changed in the package manifest
	// as well as in the constant value
	assert_eq!(env!("CARGO_BIN_NAME"), BINARY_NAME);
}