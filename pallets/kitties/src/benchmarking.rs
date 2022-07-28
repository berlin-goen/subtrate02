//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use frame_benchmarking::vec;

benchmarks! { 
	// tên của benchmark
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas: Vec<u8> = b"123".to_vec();
		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dnas, 0)
	verify {
		assert_eq!(Quantity::<T>::get(), 1);
	}

	transfer_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas: Vec<u8> = b"1234".to_vec();
		let caller: T::AccountId = whitelisted_caller();
		let receiver: T::AccountId = whitelisted_caller();
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(), dnas.clone(), 0);
	}: transfer_kitty (RawOrigin::Signed(caller), dnas, receiver) 
	verify {}
 
	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}