use crate::*;
use near_sdk::ext_contract;

pub trait NonFungibleTokenCore {
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>);

    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
    fn nft_return_candidate_likes(&self, token_id: TokenId) -> Likes;
    fn nft_add_likes_to_candidate(&mut self, token_id: TokenId);
    fn check_voter_has_been_added(&self, voter_id: AccountId) -> TokenId;
    fn check_voter_has_voted(&self, voter_id: AccountId) -> bool;
    fn voter_voted(&mut self, voter_id: AccountId);
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
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>) {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();

        self.internal_transfer(&sender_id, &receiver_id, &token_id, memo);
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

    // get number of likes of specified candidate
    fn nft_return_candidate_likes(&self, token_id: TokenId) -> Likes {
        if self.tokens_by_id.get(&token_id).is_some() {
            self.likes_per_candidate.get(&token_id).unwrap()
        } else {
            0 as Likes
        }
    }

    // add info(key: receiver id, value: number ) to map(-> this list is for check voter has already voted)
    fn voter_voted(&mut self, voter_id: AccountId) {
        self.voted_voter_list.insert(&voter_id, &(0 as u128));
    }

    // check if voter id is in added-list
    fn check_voter_has_been_added(&self, voter_id: AccountId) -> TokenId {
        if self.added_voter_list.get(&voter_id).is_some() {
            return self.added_voter_list.get(&voter_id).unwrap();
        } else {
            0
        }
    }

    // check if voter id is in voted-list
    fn check_voter_has_voted(&self, voter_id: AccountId) -> bool {
        if self.voted_voter_list.get(&voter_id).is_some() {
            return true;
        } else {
            false
        }
    }
}
