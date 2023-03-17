import React, { useEffect, useState } from 'react';

import CandidateCard from '../components/candidate_card';
import LikeIcon from '../img/like_icon.png';
import {
  checkVoterHasBeenAdded,
  checkVoterHasVoted,
  closeElection,
  ifElectionClosed,
  nftAddLikesToCandidate,
  nftReturnCandidateLikes,
  nftTokensForKind,
  nftTransfer,
  reopenElection,
  voterVoted,
} from '../js/near/utils';

// Home screen(user can vote here)
const Home = () => {
  // set valuable for candidate NFT info, num of likes for each candidate, state
  const [candidateInfoList, setCandidateInfoList] = useState();
  const [candidateLikesList] = useState([]);
  const [state, setState] = useState('fetching');

  // enum of state
  const State = {
    Fetching: 'fetching',
    Fetched: 'fetched',
    Open: 'open',
    Closed: 'closed',
  };

  // fetch candidate nft info
  useEffect(async () => {
    await nftTokensForKind('candidate').then((value) => {
      setCandidateInfoList(value);
      setState('fetched');
    });
  }, []);

  // vote function
  const vote = (tokenId) => {
    // check if user has already voted
    checkVoterHasVoted(window.accountId).then((value) => {
      if (value) {
        window.alert('You have already voted!');
        return;
      }

      // check if user has vote ticket
      checkVoterHasBeenAdded(window.accountId).then((value) => {
        const tokenIdOfVoter = parseFloat(value);
        if (tokenIdOfVoter === 0) {
          window.alert(
            "You don't have vote ticket! Please ask deployer to give it to you.",
          );
          return;
        }
        // confirm if user really vote to specified candidate(because even if they cancel transaction, contract judge user voted)
        const isSure = window.confirm(
          "Once you vote, you can't change selected candidate. Are you OK?",
        );
        if (!isSure) {
          return;
        }
        // transfer vote ticket from user to contract(get rid of vote ticket)
        nftTransfer(process.env.CONTRACT_NAME, tokenIdOfVoter);
        // add vote to specified candidate
        nftAddLikesToCandidate(tokenId);

        // add user ID to voted-list
        voterVoted(window.accountId);
      });
    });
  };

  // body(in case election is open)
  const cardsInCaseOpen = () => {
    const candidateCardList = [];
    for (let i = 0; i < candidateInfoList.length; i++) {
      // format data for rendering
      candidateCardList.push(
        <div className="items-center">
          <CandidateCard
            CID={candidateInfoList[i].metadata.media_CID}
            name={candidateInfoList[i].metadata.candidate_name}
            manifest={candidateInfoList[i].metadata.candidate_manifest}
          />
          <div className="center text-xl items-center">
            <img src={LikeIcon} className="object-cover h-5 w-5 mr-2" />
            <p className="mr-2">{candidateLikesList[i]}</p>
            <button
              value={candidateInfoList[i].metadata.tokenId}
              onClick={(event) => vote(parseInt(event.target.value))}
              className="vote_button hover:skew-1"
            >
              Vote!
            </button>
          </div>
        </div>,
      );
    }
    return candidateCardList;
  };

  // body(in case election is closed)
  const cardsInCaseClosed = () => {
    const candidateCardList = [];
    const mostVotedNum = candidateLikesList.reduce((a, b) => {
      return Math.max(a, b);
    });
    // format data for rendering
    for (let i = 0; i < candidateInfoList.length; i++) {
      if (candidateLikesList[i] === mostVotedNum) {
        // for winner candidate rendering
        candidateCardList.push(
          <div className="items-center">
            <div className="text-2xl shadow-rose-600 center font-semibold text-red-700">
              Won!
            </div>
            <CandidateCard
              CID={candidateInfoList[i].metadata.media_CID}
              name={candidateInfoList[i].metadata.candidate_name}
              manifest={candidateInfoList[i].metadata.candidate_manifest}
            />
            <div className="center text-xl items-center">
              <img src={LikeIcon} className="object-cover h-5 w-5 mr-2" />
              <p className="mr-2">{candidateLikesList[i]}</p>
            </div>
          </div>,
        );
      } else {
        // for other candidate rendering
        candidateCardList.push(
          <div className="items-center opacity-20">
            <div className="pt-7"></div>
            <CandidateCard
              CID={candidateInfoList[i].metadata.media_CID}
              name={candidateInfoList[i].metadata.candidate_name}
              manifest={candidateInfoList[i].metadata.candidate_manifest}
            />
            <div className="center text-xl items-center">
              <img src={LikeIcon} className="object-cover h-5 w-5 mr-2" />
              <p className="mr-2">{candidateLikesList[i]}</p>
            </div>
          </div>,
        );
      }
    }
    return candidateCardList;
  };

  // fetching like method
  const getCandidateLikes = async () => {
    // get num of likes for each candidate
    for (let i = 0; i < candidateInfoList.length; i++) {
      await nftReturnCandidateLikes(candidateInfoList[i].metadata.tokenId).then(
        (value) => {
          candidateLikesList.push(value);
        },
      );
    }

    // check if election is closed
    const isClosed = await ifElectionClosed();
    console.log(isClosed);
    if (isClosed) {
      setState('closed');
    } else {
      setState('open');
    }
  };

  // close button function(display to only contract deployer)
  const closeButton = () => {
    // check if user is contract deployer
    if (window.accountId !== process.env.CONTRACT_NAME) {
      return;
    }
    return (
      <button
        className="close_button hover:skew-1 h-10 bg-red-600 mb-3"
        onClick={() => {
          // confirm that user really close this election
          const isSureToClose = window.confirm(
            'Are you sure to close this election?',
          );
          if (isSureToClose) {
            // close this election
            closeElection();

            // change state to closed
            setState('closed');
          }
        }}
      >
        Close Election
      </button>
    );
  };

  // reopen button function(display to only contract deployer)
  const reopenButton = () => {
    // check if user is contract deployer
    if (window.accountId !== process.env.CONTRACT_NAME) {
      return;
    }
    return (
      <button
        className="close_button hover:skew-1 h-10 bg-red-600 mb-3"
        onClick={() => {
          const isSureToClose = window.confirm(
            'Are you sure to reopen this election?',
          );
          if (isSureToClose) {
            // reopen this election
            reopenElection();

            // change state to open
            setState('open');
          }
        }}
      >
        Reopen Election
      </button>
    );
  };

  // message to wait for fetching data
  const messageToWait = () => {
    return (
      <div className="grid h-screen place-items-center text-3xl">
        Fetching NFTs of candidates...
      </div>
    );
  };

  switch (state) {
    // in case fetching candidate NFTs info
    case State.Fetching:
      return <div>{messageToWait()}</div>;

    // in case fetching number of likes for each candidate
    case State.Fetched:
      getCandidateLikes();
      return <div>{messageToWait()}</div>;

    // in case all data is fetched(election is open)
    case State.Open:
      return (
        <div>
          <div className="center">{closeButton()}</div>
          <div className="grid grid-cols-3 gap-10">{cardsInCaseOpen()}</div>
        </div>
      );

    // in case all data is fetched(election is closed)
    case State.Closed:
      return (
        <div>
          <div className="center">{reopenButton()}</div>
          <div className="grid grid-cols-3 gap-10">{cardsInCaseClosed()}</div>
        </div>
      );
  }
};
export default Home;
