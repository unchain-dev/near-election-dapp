use crate::*;
use near_sdk::CryptoHash;

// hash account id
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();

    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// confirm caller attached one yoctoNEAR
pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR",
    )
}

// refund if caller deposit too much NEAR
pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNear to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;

    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {
    // remove token from map(token owner id->token id)
    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("Token should be owned by the sender");

        tokens_set.remove(token_id);

        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    // add token to map(token owner id->token id)
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokensPerOwnerInner {
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        tokens_set.insert(token_id);
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    // add token to map(token kind->token id)
    pub(crate) fn internal_add_token_to_kind_map(
        &mut self,
        token_id: &TokenId,
        token_kind: TokenKind,
    ) {
        let token_kind_clone = token_kind.clone();
        let mut tokens_set = self
            .tokens_per_kind
            .get(&token_kind_clone)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::TokensPerKindInner {
                        token_kind: token_kind,
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        tokens_set.insert(&token_id);
        self.tokens_per_kind.insert(&token_kind_clone, &tokens_set);
    }

    // transfer token
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        memo: Option<String>,
    ) -> TokenOwner {
        let token = self.tokens_by_id.get(token_id).expect("No token");

        if sender_id != &token.owner_id {
            env::panic_str("Unauthorized");
        }

        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        self.internal_remove_token_from_owner(&token.owner_id, token_id);

        self.internal_add_token_to_owner(receiver_id, token_id);

        let new_token = TokenOwner {
            owner_id: receiver_id.clone(),
        };

        self.tokens_by_id.insert(token_id, &new_token);

        if let Some(memo) = memo {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        token
    }
}
