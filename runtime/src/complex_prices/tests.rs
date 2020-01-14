#![cfg(test)]

use super::*;

use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

use mocktopus::mocking::*;
use sp_std::rc::Rc;

impl_outer_origin! {
    pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type ModuleToIndex = ();
}
impl Trait for Test {}

impl discounts::Trait for Test {}

type ComplexPrices = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[test]
fn calculate_price_succeeds() {
    new_test_ext().execute_with(|| {
        ComplexPrices::store_price(1, 100, Some(5));

        assert_eq!(ComplexPrices::calculate_price(1), 95);
    });
}

fn setup_discount_provider_mock(mock: Rc<dyn DiscountProvider>) {
    ComplexPrices::discounts.mock_safe(move || MockResult::Return(mock.clone()));
}

#[test]
fn calculate_price_succeeds_with_custom_discount_provider() {
    struct CustomDiscountProvider;
    impl DiscountProvider for CustomDiscountProvider {
        fn store_custom_discount(&self, _price: u32, _discount: u32) {}
        fn calculate_discount(&self, _item_id: u32, _base_price: u32) -> u32 {
            50
        }
    }

    new_test_ext().execute_with(|| {
        let custom_mock = Rc::new(CustomDiscountProvider {});
        setup_discount_provider_mock(custom_mock);

        ComplexPrices::store_price(1, 100, None);

        assert_eq!(ComplexPrices::calculate_price(1), 50);
    });
}
