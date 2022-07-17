use crate::*;
use chrono::Utc;

#[near_bindgen]
impl Contract {
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }

    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.token_metadata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let tokens_for_kind_set = self.tokens_per_owner.get(&account_id);
        if let Some(tokens_for_kind_set) = tokens_for_kind_set {
            U128(tokens_for_kind_set.len() as u128)
        } else {
            U128(0)
        }
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let tokens_for_kind_set = self.tokens_per_owner.get(&account_id);
        let tokens = if let Some(tokens_for_kind_set) = tokens_for_kind_set {
            tokens_for_kind_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));
        tokens
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    pub fn nft_tokens_for_kind(
        &self,
        token_kind: TokenKind,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let tokens_for_kind_set = self.tokens_per_kind.get(&token_kind);
        let tokens = if let Some(tokens_for_kind_set) = tokens_for_kind_set {
            tokens_for_kind_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));
        tokens
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    pub fn get_time() {
        let dt = Utc::now();
        println!("{}", dt);
        println!("{}", dt.format("%Y年%m月%d日 %H:%M:%S"));
    }
}
