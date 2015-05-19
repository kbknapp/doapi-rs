use super::request::Account;

pub struct DoManager<'t> {
    pub account: Account<'t>
}

impl<'t> DoManager<'t> {
    pub fn with_token(t: &'t str) -> DoManager {
        DoManager {
            account: Account::with_token(t)
        }
    }

    pub fn account(&self) -> &Account {
        &self.account
    }
}
