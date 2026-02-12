use crate::*;

#[near_bindgen]
impl Contract {
    // Additional admin-only method to change treasury
    pub fn set_treasury(&mut self, new_treasury: AccountId) {
        self.assert_admin();
        self.treasury = new_treasury;
    }

    // Emergency withdraw for stuck tokens (admin only)
    pub fn emergency_withdraw(&mut self, token: Option<AccountId>, amount: Balance, receiver: AccountId) -> Promise {
        self.assert_admin();
        require!(self.paused, "Contract must be paused for emergency withdraw");

        if let Some(token_account) = token {
            ext_ft::ext(token_account)
                .with_attached_deposit(1)
                .with_static_gas(Gas(5 * TGAS))
                .ft_transfer(receiver, amount, None)
        } else {
            Promise::new(receiver).transfer(amount)
        }
    }
}
