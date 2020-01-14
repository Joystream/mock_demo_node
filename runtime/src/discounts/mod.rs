use frame_support::{decl_module, decl_storage};

#[cfg(test)]
use mocktopus::macros::*;

#[cfg(test)]
mod tests;

/// The module's configuration trait.
pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Discounts {

        DiscountByPrice get(get_discount): map u32 => Option<u32>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

#[cfg_attr(test, mockable)]
impl<T: Trait> Module<T> {
    pub fn store_custom_discount(price: u32, discount: u32) {
        DiscountByPrice::insert(price, discount);
    }

    pub fn calculate_discount(base_price: u32) -> u32 {
        let custom_discount = DiscountByPrice::get(base_price);

        if let Some(discount) = custom_discount {
            discount
        } else {
            if base_price > 50 {
                20
            } else {
                0
            }
        }
    }
}
