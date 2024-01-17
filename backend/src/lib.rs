use std::borrow::Cow;
use std::cell::RefCell;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk_macros::{query, update};
use serde::Serialize;

const MAX_VALUE_SIZE: u32 = 100000;

type Memory = VirtualMemory<DefaultMemoryImpl>;
// type ProductStore = BTreeMap<Principal, ProductItem, StableBTreeMap>;

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
struct ProductItem {
    pub name: String,
    pub price: f32,
    pub id: u128,
    pub description: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
struct ProductItemCreateParams {
    pub name: String,
    pub price: f32,
    pub description: String,
}

impl Storable for ProductItem {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRODUCT_STORE: RefCell<StableBTreeMap<u128, ProductItem, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[update]
fn create_product(product: ProductItemCreateParams) -> u128 {
    PRODUCT_STORE.with(|product_store| {
        let mut products = product_store.borrow_mut();
        let new_id = products.len() as u128;
        let new_product = ProductItem {
            name: product.name,
            price: product.price,
            description: product.description,
            id: new_id,
        };
        products.insert(new_id.clone(), new_product);
        return new_id;
    })
}

#[query]
fn get_product(key: u128) -> Option<ProductItem> {
    PRODUCT_STORE.with(|product_store| product_store.borrow_mut().get(&key))
}

#[query]
fn search_products(search_text: String) -> Vec<ProductItem> {
    PRODUCT_STORE.with(|p| {
        let mut products = vec![];

        for (_, product) in p.borrow().iter() {
            if product.name.to_ascii_lowercase().contains(&search_text.to_ascii_lowercase()) {
                products.push(product.clone());
            }
        }
        products
    })
}