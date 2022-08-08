use near_sdk::{AccountId, Balance, Gas, Promise};

mod interface;

pub fn ft_transfer(ft_contract_id: AccountId, receiver_id: AccountId, amount: Balance) -> Promise {
    crate::interface::ext_ft::ft_transfer(
        receiver_id,
        amount.into(),
        None,
        ft_contract_id,
        1,
        // could possibly tune this down, but these costs will
        // depend on the impl of the FT contract
        Gas(15_000_000_000_000),
    )
}
