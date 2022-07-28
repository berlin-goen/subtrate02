use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(DemoModule::create_student(Origin::signed(1), b"abc".to_vec(), 21));
		// Read pallet storage and assert an expected result.
		assert_eq!(DemoModule::student_id(), 1);
	});
}

#[test]
fn create_student_fail_if_age_is_lower_than_20() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			DemoModule::create_student(Origin::signed(1), b"student_name".to_vec(), 10),
			Error::<Test>::TooYoung
		);
	});
}