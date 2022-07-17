use crate::*;
pub type TokenId = u128;
pub type CandidateName = String;
pub type TokenKind = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]

// metadata of contract
pub struct NFTContractMetadata {
    pub spec: String,
    pub name: String,
    pub icon_uri: String,
    pub reference: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

//metadata of Token
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: String,
    pub candidate_name: Option<String>,
    pub candidate_manifest: Option<String>,
    pub token_kind: String,
    pub token_id: Option<u128>,
    pub num_of_likes: Option<u128>,
}

// metadata of Token Owner
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenOwner {
    pub owner_id: AccountId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

// metadata of type of Json
pub struct JsonToken {
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}

pub trait NFTTokenMetadata {
    fn nft_metadata(&self) -> NFTContractMetadata;
}

// view function for contract info
#[near_bindgen]
impl NFTTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
