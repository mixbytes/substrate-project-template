#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::{
    codec::{Decode, Encode},
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    sp_runtime::traits::AtLeast32Bit,
    traits::Get,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Change role type if requires more than 8 distinct values
pub type AccountRole = u8;
/// Structure, specific for each role
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default)]
pub struct Account<Moment> {
    pub roles: AccountRole,
    pub create_time: Moment,
}

impl<Moment> Account<Moment> {
    pub fn is_admin(&self) -> bool {
        self.roles & ADMIN_ROLE == ADMIN_ROLE
    }
    pub fn is_enable(&self) -> bool {
        self.roles > 0
    }
}
// implement type-specific methods
impl<Moment: Default + AtLeast32Bit + Copy> Account<Moment> {
    #[allow(dead_code)]
    pub fn age(&self, now: Moment) -> Moment {
        now - self.create_time
    }

    pub fn new_admin() -> Self {
        Account {
            roles: ADMIN_ROLE,
            create_time: Default::default(),
        }
    }
}

pub type AccountOf<T> = Account<<T as pallet_timestamp::Trait>::Moment>;

/// Account roles . Add additional values if required
#[allow(dead_code)]
const NONE_ROLE: AccountRole = 0x00;
pub const ADMIN_ROLE: AccountRole = 0x01;
#[allow(dead_code)]
const USER_ROLE: AccountRole = 0x02;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + pallet_timestamp::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    // Describe pallet constants.
    // Lean more https://substrate.dev/docs/en/knowledgebase/runtime/metadata
    type AdminRole: Get<AccountRole>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        Something get(fn something): Option<u32>;

        AccountRegistry
            get(fn account_registry)
            config(genesis_account_registry):
            map hasher(blake2_128_concat) T::AccountId => AccountOf<T>;

    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, AccountId),
        /// New account has been created [who, account, role]
        AccountCreated(AccountId, AccountId, AccountRole),
        /// Account has been disabled [who, account]
        AccountDisabled(AccountId, AccountId),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Error names should be descriptive.
        NoneValue,
        /// Operation is not valid
        InvalidAction,
        /// Origin do not have sufficient privileges to perform the operation
        NotAuthorized,
        /// Account doesn't exist
        NotExists,

        // add additional errors below
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Make module constants visible in metadata
        const AdminRole: AccountRole = T::AdminRole::get();

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn account_add(origin, account: T::AccountId, role: AccountRole) -> dispatch::DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;
            ensure!(Self::account_is_admin(&who), Error::<T>::NotAuthorized);
            // Get current timestamp using pallet-timestamp module
            let now = <pallet_timestamp::Module<T>>::get();
            // Update storage.
            AccountRegistry::<T>::insert(&account,
                Account { roles: role, create_time: now }
            );

            // Emit an event.
            Self::deposit_event(RawEvent::AccountCreated(who, account, role));
            // Return a successful DispatchResult
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn account_disable(origin, whom: T::AccountId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            // Ensure origin has associated account with admin privileges.
            ensure!(Self::account_is_admin(&who), Error::<T>::NotAuthorized);
            // Self disabling is prohibited.
            ensure!(who != whom, Error::<T>::InvalidAction);
            // Raise error if account doesn't exist or has been disabled already.
            ensure!(AccountRegistry::<T>::contains_key(&whom), Error::<T>::NotExists);
            // Delete a value from storage.
            AccountRegistry::<T>::remove(&whom);
            // or just disable it by assigning 0 value to the role
            // AccountRegistry::<T>::mutate(&whom, |item|{
            // 	item.roles = NONE_ROLE;
            // });
            Self::deposit_event(RawEvent::AccountDisabled(who, whom));
            Ok(())
        }

        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            // Update storage.
            Something::put(something);

            // Emit an event.
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            // Return a successful DispatchResult
            Ok(())
        }

    }
}

impl<T: Trait> Module<T> {
    pub fn account_is_admin(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).is_admin()
    }
}
