// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The traits for putting freezes within a single fungible token class.

use super::*;

/// Trait for inspecting a fungible asset which can be frozen. Freezing is essentially setting a
/// minimum balance bellow which the total balance (inclusive of any funds placed on hold) may not
/// be normally allowed to drop. Generally, freezers will provide an "update" function such that
/// if the total balance does drop below the limit, then the freezer can update their housekeeping
/// accordingly.
pub trait InspectFreeze<AccountId>: Inspect<AccountId> {
	/// An identifier for a freeze.
	type Id: codec::Encode + TypeInfo + 'static;

	/// Amount of funds held in reserve by `who` for the given `id`.
	fn balance_frozen(id: &Self::Id, who: &AccountId) -> Self::Balance;

	/// The amount of the balance which can become frozen. Defaults to `total_balance()`.
	fn balance_freezable(who: &AccountId) -> Self::Balance {
		Self::total_balance(who)
	}

	/// Returns `true` if it's possible to introduce a freeze for the given `id` onto the
	/// account of `who`. This will be true as long as the implementor supports as many
	/// concurrent freeze locks as there are possible values of `id`.
	fn can_freeze(id: &Self::Id, who: &AccountId) -> bool;
}

/// Trait for introducing, altering and removing locks to freeze an account's funds so they never
/// go below a set minimum.
pub trait MutateFreeze<AccountId>: InspectFreeze<AccountId> {
	/// Prevent the balance of the account of `who` from being reduced below the given `amount` and
	/// identify this restriction though the given `id`. Unlike `extend_freeze`, any outstanding
	/// freezes in place for `who` under the `id` are dropped.
	///
	/// Note that more funds can be locked than the total balance, if desired.
	fn set_freeze(
		id: &Self::Id,
		who: &AccountId,
		amount: Self::Balance,
	) -> Result<(), DispatchError> {
		Self::thaw(id, who);
		Self::extend_freeze(id, who, amount)
	}

	/// Prevent the balance of the account of `who` from being reduced below the given `amount` and
	/// identify this restriction though the given `id`. Unlike `set_freeze`, this does not
	/// counteract any pre-existing freezes in place for `who` under the `id`.
	///
	/// Note that more funds can be locked than the total balance, if desired.
	fn extend_freeze(id: &Self::Id, who: &AccountId, amount: Self::Balance) -> Result<(), DispatchError>;

	/// Remove an existing lock.
	fn thaw(id: &Self::Id, who: &AccountId);
}