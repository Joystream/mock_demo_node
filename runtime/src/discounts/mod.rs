use frame_support::{decl_module, decl_storage};

#[cfg(test)]
use mocktopus::macros::*;

#[cfg(test)]
mod tests;

/// The module's configuration trait.
pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Discounts {

        DiscountByItemId get(get_discount): map u32 => Option<u32>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

#[cfg_attr(test, mockable)]
impl<T: Trait> Module<T> {
    pub fn store_custom_discount(item_id: u32, discount: u32) {
        DiscountByItemId::insert(item_id, discount);
    }

    pub fn calculate_discount(item_id: u32, base_price: u32) -> u32 {
        let custom_discount = DiscountByItemId::get(item_id);

        if let Some(discount) = custom_discount {
            discount
        } else if base_price > 50 {
            20
        } else {
            0
        }
    }
}
