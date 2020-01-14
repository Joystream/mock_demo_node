use frame_support::{decl_module, decl_storage};

use super::discounts;
use sp_std::marker::PhantomData;
use sp_std::rc::Rc;
#[cfg(test)]
mod tests;

/// The module's configuration trait.
pub trait Trait: system::Trait + discounts::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as ComplexPrices {

        PriceById get(get_base_price): map u32 => Option<u32>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

impl<T: Trait> Module<T> {
    pub fn store_price(id: u32, price: u32, custom_discount: Option<u32>) {
        PriceById::insert(id, price);

        if let Some(discount) = custom_discount {
            Self::discounts().store_custom_discount(price, discount);
        }
    }
    pub fn calculate_price(id: u32) -> u32 {
        let base_price = PriceById::get(id).unwrap();

        let discount = Self::discounts().calculate_discount(base_price);

        base_price - discount
    }

    fn discounts() -> Rc<dyn DiscountProvider> {
        Rc::new(DefaultDiscountProvider::<T> {
            marker: PhantomData,
        })
    }
}

pub trait DiscountProvider {
    fn store_custom_discount(&self, price: u32, discount: u32);

    fn calculate_discount(&self, base_price: u32) -> u32;
}

struct DefaultDiscountProvider<T: Trait> {
    marker: PhantomData<T>,
}
impl<T: Trait> DiscountProvider for DefaultDiscountProvider<T> {
    fn store_custom_discount(&self, price: u32, discount: u32) {
        <discounts::Module<T>>::store_custom_discount(price, discount);
    }

    fn calculate_discount(&self, base_price: u32) -> u32 {
        <discounts::Module<T>>::calculate_discount(base_price)
    }
}
