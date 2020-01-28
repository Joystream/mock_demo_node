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
use std::panic;
use std::rc::Rc;

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

// Intercepts panic method
// Returns: whether panic occurred
fn panics<F: std::panic::RefUnwindSafe + Fn()>(could_panic_func: F) -> bool {
    {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(|info| {
            println!("{}", info);
        }));

        // intercept panic
        let result = panic::catch_unwind(|| could_panic_func());

        //restore default behaviour
        panic::set_hook(default_hook);

        result.is_err()
    }
}

// Tests mock expectation and restores default behaviour
pub(crate) fn test_expectation_and_clear_mock() {
    setup_discount_provider_mock(Rc::new(super::DefaultDiscountProvider {
        marker: PhantomData::<Test> {},
    }));
}

// Intercepts panic in provided function, test mock expectation and restores default behaviour
pub(crate) fn handle_mock<F: std::panic::RefUnwindSafe + Fn()>(func: F) {
    let panicked = panics(func);

    test_expectation_and_clear_mock();

    assert!(!panicked);
}

fn setup_discount_provider_mock(mock: Rc<dyn DiscountProvider>) {
    ComplexPrices::discounts.mock_safe(move || MockResult::Return(mock.clone()));
}

#[test]
fn calculate_price_succeeds() {
    new_test_ext().execute_with(|| {
        ComplexPrices::store_price(1, 100, Some(5));

        assert_eq!(ComplexPrices::calculate_price(1), 95);
    });
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

#[test]
fn calculate_price_succeeds_with_feature_rich_mocks() {
    handle_mock(|| {
        new_test_ext().execute_with(|| {
            let mock = {
                let mut mock = super::MockDiscountProvider::new();
                mock.expect_calculate_discount()
                    .times(1)
                    .returning(|_, _| 70);

                mock.expect_store_custom_discount()
                    .times(1)
                    .returning(|_, _| ());

                Rc::new(mock)
            };
            setup_discount_provider_mock(mock.clone());

            ComplexPrices::store_price(1, 100, Some(70));

            assert_eq!(ComplexPrices::calculate_price(1), 30);
        })
    });
}
