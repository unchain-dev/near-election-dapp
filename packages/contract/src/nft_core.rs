use crate::*;
use near_sdk::ext_contract;
pub trait NonFungibleTokenCore {
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId);
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
    fn nft_add_likes_to_candidate(&mut self, token_id: TokenId);
}
#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[payable]
    // transfer token
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId) {
        assert!(
            !(&self.is_election_closed),
            "You can no longer vote because it's been closed!"
        );
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();

        self.internal_transfer(&sender_id, &receiver_id, &token_id);
    }

    // get specified token info
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            Some(JsonToken {
                owner_id: token.owner_id,
                metadata,
            })
        } else {
            None
        }
    }

    fn nft_add_likes_to_candidate(&mut self, token_id: TokenId) {
        if self.likes_per_candidate.get(&token_id).is_some() {
            let mut likes = self.likes_per_candidate.get(&token_id);
            likes.replace(likes.unwrap() + 1 as Likes);
            self.likes_per_candidate.insert(&token_id, &likes.unwrap());
        }
    }
}
