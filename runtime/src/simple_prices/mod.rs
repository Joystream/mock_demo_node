use frame_support::{decl_module, decl_storage};

use super::discounts;

#[cfg(test)]
mod tests;

/// The module's configuration trait.
pub trait Trait: system::Trait + discounts::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as SimplePrices {

        PriceByItemId get(get_base_price): map u32 => Option<u32>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

impl<T: Trait> Module<T> {
    pub fn store_price(item_id: u32, price: u32) {
        PriceByItemId::insert(item_id, price);
    }
    pub fn calculate_price(item_id: u32) -> u32 {
        let base_price = PriceByItemId::get(item_id).unwrap();

        let discount = <discounts::Module<T>>::calculate_discount(item_id, base_price);

        base_price - discount
    }
}
