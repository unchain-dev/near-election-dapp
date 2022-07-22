use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]

    //mint token
    pub fn nft_mint(&mut self, mut metadata: TokenMetadata, receiver_id: AccountId) {
        // set token id
        assert!(
            !(&self.is_election_closed),
            "You can add candidate or voter because this election has been closed!"
        );
        metadata.token_id = Some(self.token_id_counter);
        let initial_storage_usage = env::storage_usage();
        let receiver_id_clone = receiver_id.clone();
        let token = TokenOwner {
            owner_id: receiver_id,
        };
        let token_id = self.token_id_counter;
        let token_kind = metadata.token_kind.clone();

        assert!(
            self.tokens_by_id
                .insert(&self.token_id_counter, &token)
                .is_none(),
            "Token already exists"
        );

        // add info(key: receiver_id, value: token metadata ) to map
        self.token_metadata_by_id
            .insert(&self.token_id_counter, &metadata);

        // add info(key: receiver id, value: token id ) to map
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // add info(key: token id, value: token kind ) to map
        self.internal_add_token_to_kind_map(&token_id, token_kind);

        // add data(key: token id, value: number of likes)
        self.likes_per_candidate
            .insert(&self.token_id_counter, &(0 as Likes));

        // add info(key: receiver id, value: token id ) to map(-> this list is for check voter get vote ticket)
        self.added_voter_list
            .insert(&receiver_id_clone, &self.token_id_counter);

        // increment token id counter
        self.token_id_count();

        // calculate storage user used
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // refund unused payment deposit
        refund_deposit(required_storage_in_bytes);
    }

    // count token id
    pub fn token_id_count(&mut self) {
        self.token_id_counter = self.token_id_counter + 1;
    }

    // get next token id
    pub fn show_token_id_counter(&self) -> u128 {
        self.token_id_counter
    }
}
