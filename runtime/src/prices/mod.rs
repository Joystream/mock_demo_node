use frame_support::{decl_module, decl_storage};

use super::discounts;
mod tests;

/// The module's configuration trait.
pub trait Trait: system::Trait + discounts::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Prices {

        PriceById get(get_base_price): map u32 => Option<u32>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

impl<T: Trait> Module<T> {
    pub fn store_price(id: u32, price: u32) {
        PriceById::insert(id, price);
    }
    pub fn calculate_price(id: u32) -> u32 {
        let base_price = PriceById::get(id).unwrap();

        let discount = <discounts::Module<T>>::calculate_discount(base_price);

        base_price - discount
    }
}
