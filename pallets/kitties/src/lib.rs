#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

use pallet_timestamp;
use pallet_template::Limit;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		created_at: <T as pallet_timestamp::Config>::Moment
	}

	#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self {
			Gender::Male
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type KittiesLimit: Limit;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Quantity<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_from_dna)]
	pub(super) type KittyDNA<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_kitties)]
	pub(super) type OwnerKitties<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreated(Vec<u8>),
		KittyTransfered(Vec<u8>, T::AccountId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotKittyOwner,
		ExceedLimit
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			let mut gender = Gender::Female;
			if dna.len() % 2 == 0 {
				gender = Gender::Male;
			}

			let kitty = Kitty {
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
				created_at: <pallet_timestamp::Pallet<T>>::get()
			};

			let mut current_quantity = <Quantity<T>>::get();
			current_quantity += 1;
			Quantity::<T>::put(current_quantity);

			<KittyDNA<T>>::insert(dna.clone(), kitty.clone());

			let mut owner_kitty_list = <OwnerKitties<T>>::get(who.clone()).unwrap_or_else(|| Vec::new());
			ensure!(owner_kitty_list.len() < T::KittiesLimit::get() as usize, Error::<T>::ExceedLimit);
			owner_kitty_list.push(dna.clone()); 
			<OwnerKitties<T>>::insert(who.clone(), owner_kitty_list);
			// Emit an event.
			Self::deposit_event(Event::KittyCreated(dna.clone()));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(origin: OriginFor<T>, dna: Vec<u8>, receiver_id: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// ensure the dna of kitty is belong to the sender
			let mut sender_kitties_list = <OwnerKitties<T>>::get(sender.clone()).unwrap_or_else(|| Vec::new());
			let mut is_owner = false;
			let mut kitty_index = 0;
			for (idx, val) in sender_kitties_list.iter().enumerate() {
				if dna == *val {
					is_owner = true;
					kitty_index = idx;
					break;
				}
			}
			ensure!(is_owner == true, Error::<T>::NotKittyOwner);

			// remove kitty from OwnerKitties mapping with sender's accountId as key
			sender_kitties_list.remove(kitty_index);
			<OwnerKitties<T>>::insert(sender.clone(), sender_kitties_list);

			// insert kitty to receiver mapping
			let mut receiver_kitties_list = <OwnerKitties<T>>::get(receiver_id.clone()).unwrap_or_else(|| Vec::new());
			ensure!(receiver_kitties_list.len() < T::KittiesLimit::get() as usize, Error::<T>::ExceedLimit);
			receiver_kitties_list.push(dna.clone());
			<OwnerKitties<T>>::insert(receiver_id.clone(), receiver_kitties_list);

			// Emit transfer event
			Self::deposit_event(Event::KittyTransfered(dna.clone(), sender.clone(), receiver_id.clone()));

			Ok(())
		}
	}
}
