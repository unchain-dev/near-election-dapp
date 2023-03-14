import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';

import getConfig from './config';

const BN = require('bn.js');

const nearConfig = getConfig(process.env.NODE_ENV || 'development');

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(
    Object.assign(
      { deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } },
      nearConfig,
    ),
  );

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near);

  window.accountId = window.walletConnection.getAccountId();

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(
    window.walletConnection.account(),
    nearConfig.contractName,
    {
      viewMethods: [
        'nftMetadata',
        'nftTokensForKind',
        'nftReturnCandidateLikes',
        'checkVoterHasBeenAdded',
        'checkVoteHasVoted',
        'ifElectionClosed',
      ],

      changeMethods: [
        'newDefaultMeta',
        'nftMint',
        'nftTransfer',
        'nftAddLikesToCandidate',
        'voterVoted',
        'closeElection',
        'reopenElection',
      ],
    },
  );
}

export function logout() {
  window.walletConnection.signOut();
  // reload page
  window.location.replace(window.location.origin + window.location.pathname);
}

export function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName);
}

export async function newDefaultMeta() {
  await window.contract.newDefaultMeta({ owner_id: window.accountId });
}

export async function nftMint(
  title,
  description,
  media,
  mediaCID,
  candidateName,
  candidateManifest,
  tokenKind,
  receiverId,
) {
  await window.contract.nftMint(
    {
      metadata: {
        title,
        description,
        media,
        mediaCID,
        candidateName,
        candidateManifest,
        tokenKind,
      },
      receiverId,
    },
    300000000000000, // attached GAS (optional)
    new BN('1000000000000000000000000'),
  );
}

export async function nftTransfer(receiverId, tokenId) {
  await window.contract.nftTransfer(
    {
      receiverId,
      tokenId,
    },
    300000000000000, // attached GAS (optional)
    new BN('1'), // deposit yoctoNEAR
  );
}

export async function nftAddLikesToCandidate(tokenId) {
  await window.contract.nftAddLikesToCandidate({ tokenId });
}

export async function nftMetadata() {
  const contractMetadata = await window.contract.nftMetadata();
  return contractMetadata;
}

export async function nftTokensForKind(tokenKind) {
  const tokensList = await window.contract.nftTokensForKind({
    tokenKind,
  });
  return tokensList;
}

export async function nftReturnCandidateLikes(tokenId) {
  const numOfLikes = await window.contract.nftReturnCandidateLikes({
    tokenId,
  });

  return numOfLikes;
}

export async function checkVoterHasBeenAdded(voterId) {
  return await window.contract.checkVoterHasBeenAdded({
    voterId,
  });
}

export async function checkVoteHasVoted(voterId) {
  return await window.contract.checkVoteHasVoted({ voterId });
}

export async function voterVoted(voterId) {
  return await window.contract.voterVoted({ voterId });
}

export async function ifElectionClosed() {
  return await window.contract.ifElectionClosed();
}

export async function closeElection() {
  await window.contract.closeElection();
}
export async function reopenElection() {
  await window.contract.reopenElection();
}
