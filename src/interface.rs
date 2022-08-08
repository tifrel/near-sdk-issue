use near_sdk::{json_types, AccountId};

#[near_sdk::ext_contract(ext_ft)]
pub trait ExtFtContract {
    fn ft_transfer(receiver_id: AccountId, amount: json_types::U128, memo: Option<String>);
}
