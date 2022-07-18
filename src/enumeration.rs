use crate::*;

#[near_bindgen]
impl Contract {
    // get number of tokens
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }

    // get tokens(caller can select how many tokens to get)
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.token_metadata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    // get number of tokens for specified owner
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let tokens_for_kind_set = self.tokens_per_owner.get(&account_id);
        if let Some(tokens_for_kind_set) = tokens_for_kind_set {
            U128(tokens_for_kind_set.len() as u128)
        } else {
            U128(0)
        }
    }

    // get token info for specified owner
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

    // get tokens for specified kind(vote ticket or candidate info)
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
}
