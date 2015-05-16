use super::request::Account;

pub struct DoManager<'a> {
    pub account: Account<'a>
}

impl<'a> DoManager<'a> {
    pub fn with_token(t: &'a str) -> DoManager {
        DoManager {
            account: Account::with_token(t)
        }
    }
}
