use frame_support::{decl_module, decl_storage};

use super::discounts;
use sp_std::marker::PhantomData;
use sp_std::rc::Rc;
#[cfg(test)]
mod tests;

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

/// The module's configuration trait.
pub trait Trait: system::Trait + discounts::Trait {
    type DiscountHandlerProvider: DiscountHandlerProvider;
}

decl_storage! {
    trait Store for Module<T: Trait> as ComplexPrices {
        PriceByItemId get(get_base_price): map u32 => Option<u32>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

impl<T: Trait> Module<T> {
    pub fn store_price(item_id: u32, price: u32, custom_discount: Option<u32>) {
        PriceByItemId::insert(item_id, price);

        if let Some(discount) = custom_discount {
            T::DiscountHandlerProvider::discounts().store_custom_discount(item_id, discount);
        }
    }
    pub fn calculate_price(item_id: u32) -> u32 {
        let base_price = PriceByItemId::get(item_id).unwrap();

        let discount =
            T::DiscountHandlerProvider::discounts().calculate_discount(item_id, base_price);

        base_price - discount
    }
}

#[cfg_attr(test, automock)]
pub trait DiscountHandler {
    fn store_custom_discount(&self, item_id: u32, discount: u32);

    fn calculate_discount(&self, item_id: u32, base_price: u32) -> u32;
}

pub(crate) struct DefaultDiscountHandler<T: Trait> {
    marker: PhantomData<T>,
}
impl<T: Trait> DiscountHandler for DefaultDiscountHandler<T> {
    fn store_custom_discount(&self, item_id: u32, discount: u32) {
        <discounts::Module<T>>::store_custom_discount(item_id, discount);
    }

    fn calculate_discount(&self, item_id: u32, base_price: u32) -> u32 {
        <discounts::Module<T>>::calculate_discount(item_id, base_price)
    }
}

pub trait DiscountHandlerProvider {
    fn discounts() -> Rc<dyn DiscountHandler>;
}

impl<T: Trait> DiscountHandlerProvider for Module<T> {
    fn discounts() -> Rc<dyn DiscountHandler> {
        Rc::new(DefaultDiscountHandler::<T> {
            marker: PhantomData,
        })
    }
}
