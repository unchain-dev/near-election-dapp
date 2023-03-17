import React from 'react';
import { IpfsImage } from 'react-ipfs-image';

// template  candidate card template
const CandidateCard = (props) => {
  return (
    <div className="max-w-sm rounded overflow-hidden shadow-lg w-64 h-96">
      <IpfsImage
        className="w-full h-3/5"
        hash={props.CID}
        gatewayUrl="https://gateway.pinata.cloud/ipfs/"
        alt="Sunset in the mountains"
      />
      <div className="px-6 py-4">
        <div className="font-bold text-xl mb-2">{props.name}</div>
        <p className="text-gray-700 text-base">{props.manifest}</p>
      </div>
    </div>
  );
};

export default CandidateCard;
