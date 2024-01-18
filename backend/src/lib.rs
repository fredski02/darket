use serde::Serialize;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::caller as caller_api;
use ic_cdk::export::{candid, Principal};
use ic_cdk::println;
use ic_cdk_macros::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100000;

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
struct ProductItem {
    pub name: String,
    pub price: f32,
    pub id: u128,
    pub description: String,
    pub owner: String,
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

#[derive(Serialize, Deserialize)]
struct TempStore {
    carts: BTreeMap<Principal, Vec<ProductItem>>,
}

impl Default for TempStore {
    fn default() -> Self {
        Self {
            carts: BTreeMap::new(),
        }
    }
}
/// Unlike Motoko, the caller identity is not built into Rust.
/// Thus, we use the ic_cdk::api::caller() method inside this wrapper function.
/// The wrapper prevents the use of the anonymous identity. Forbidding anonymous
/// interactions is the recommended default behavior for IC canisters.
fn caller() -> Principal {
    let caller = caller_api();
    // The anonymous principal is not allowed to interact with the
    // encrypted notes canister.
    if caller == Principal::anonymous() {
        panic!("Anonymous principal not allowed to make calls.")
    }
    caller
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRODUCT_STORE: RefCell<StableBTreeMap<u128, ProductItem, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static TEMP_STORE: RefCell<TempStore> = RefCell::new(TempStore::default())
}

#[update]
fn create_product(product: ProductItemCreateParams) -> u128 {
    PRODUCT_STORE.with(|product_store| {
        let user = caller();
        println!("{:?}", user.to_string());
        let mut products = product_store.borrow_mut();
        let new_id = products.len() as u128;
        let new_product = ProductItem {
            name: product.name,
            price: product.price,
            description: product.description,
            id: new_id,
            owner: user.to_string(),
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
            if product
                .name
                .to_ascii_lowercase()
                .contains(&search_text.to_ascii_lowercase())
            {
                products.push(product.clone());
            }
        }
        products
    })
}

#[query]
fn get_user_products() -> Vec<ProductItem> {
    PRODUCT_STORE.with(|p| {
        let mut products = vec![];
        let user = caller();

        for (_, product) in p.borrow().iter() {
            if product.owner == user.to_string() {
                products.push(product.clone())
            }
        }
        products
    })
}

#[update]
fn delete_product(key: u128) -> bool {
    PRODUCT_STORE.with(|p| {
        let user = caller();

        let product = p.borrow_mut().get(&key);
        match product {
            Some(prod) => {
                if prod.owner != user.to_string() {
                    return false;
                }
                let res = p.borrow_mut().remove(&key);
                match res {
                    Some(_) => return true,
                    None => return false,
                }
            }
            None => return false,
        }
    })
}

#[update]
fn add_to_cart(product_id: u128) -> Result<(), String> {
    PRODUCT_STORE.with(|prod_store| {
        TEMP_STORE.with(|temp_store| {
            let user = caller();

            let mut cart: Vec<ProductItem> = vec![];
            let p_store = prod_store.borrow();
            let mut t_store = temp_store.borrow_mut();
            match p_store.get(&product_id) {
                Some(prod) => cart.push(prod),
                None => {
                    return Err(format!(
                        "Product with id : {} was not found. Failed to add to cart",
                        &product_id
                    ))
                }
            }

            match t_store.carts.get_mut(&user) {
                Some(user_cart) => {
                    user_cart.append(&mut cart);
                    Ok(())
                }
                None => {
                    t_store.carts.insert(user, cart);
                    Ok(())
                }
            }
        })
    })
}

#[update]
fn remove_from_cart(product_id: u128) -> Result<(), String> {
    TEMP_STORE.with(|temp_store| {
        let user = caller();
        match temp_store.borrow_mut().carts.get_mut(&user) {
            Some(cart) => {
                let index = cart.iter().position(|prod| prod.id == product_id).unwrap();
                cart.remove(index);
                // cart.retain(|cart_item| &cart_item.id != &product_id);
                Ok(())
            }
            None => Err("User has not cart".to_string()),
        }
    })
}

#[query]
fn get_cart() -> Vec<ProductItem> {
    TEMP_STORE.with(|temp_store| {
        let user = caller();

        let return_cart: Vec<ProductItem>;
        let mut store = temp_store.borrow_mut();
        match store.carts.get(&user) {
            Some(cart) => {
                return_cart = cart.clone();
            }
            None => {
                store.carts.insert(user, vec![]);
                let temp_vec: Vec<ProductItem> = vec![];
                return_cart = temp_vec.clone();
            }
        }
        return_cart
    })
}
