use super::RawEvent;
use crate::{Module, Trait};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use frame_support::{assert_ok, assert_err, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system as system;

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for TestRuntime {
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

mod simple_map {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		simple_map<T>,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type SimpleMap = Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		TestExternalities::from(storage)
	}
}

#[test]
fn set_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(1), 19));

		let expected_event = TestEvent::simple_map(RawEvent::EntrySet(1, 19));

		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn get_works() {
	ExtBuilder::build().execute_with(|| {
		assert_err!(
			SimpleMap::get_single_entry(Origin::signed(1), 2),
			"an entry does not exist for this user"
		);

		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::get_single_entry(Origin::signed(1), 2));

		let expected_event = TestEvent::simple_map(RawEvent::EntryGot(1, 19));

		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn take_works() {
	ExtBuilder::build().execute_with(|| {
		assert_err!(
			SimpleMap::take_single_entry(Origin::signed(2)),
			"an entry does not exist for this user"
		);

		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::take_single_entry(Origin::signed(2)));

		let expected_event = TestEvent::simple_map(RawEvent::EntryTook(2, 19));

		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn increase_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::increase_single_entry(Origin::signed(2), 2));

		let expected_event = TestEvent::simple_map(RawEvent::IncreaseEntry(19, 21));

		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn cas_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));

		assert_err!(
			SimpleMap::compare_and_swap_single_entry(Origin::signed(2), 18, 32),
			"cas failed bc old_entry inputted by user != existing_entry"
		);

		assert_ok!(SimpleMap::compare_and_swap_single_entry(Origin::signed(2), 19, 32));

		let expected_event = TestEvent::simple_map(RawEvent::CAS(19, 32));

		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}