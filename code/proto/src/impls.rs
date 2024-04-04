use malachite_common::{Context, Round, SignedProposal, SignedVote, SigningScheme, VoteType};

use crate::{self as proto, Error, Protobuf};

impl Protobuf for Round {
    type Proto = proto::Round;

    fn from_proto(proto: Self::Proto) -> Result<Self, Error> {
        Ok(Round::new(proto.round))
    }

    fn to_proto(&self) -> Result<Self::Proto, Error> {
        Ok(proto::Round {
            round: self.as_i64(),
        })
    }
}

impl<Ctx: Context> Protobuf for SignedVote<Ctx>
where
    Ctx::Vote: Protobuf<Proto = proto::Vote>,
{
    type Proto = proto::SignedVote;

    fn from_proto(proto: Self::Proto) -> Result<Self, Error> {
        let vote = proto
            .vote
            .ok_or_else(|| Error::missing_field::<proto::SignedVote>("vote"))?;

        Ok(Self {
            vote: Ctx::Vote::from_proto(vote)?,
            signature: Ctx::SigningScheme::decode_signature(&proto.signature)
                .map_err(|e| Error::Other(format!("Failed to decode signature: {e}")))?,
        })
    }

    fn to_proto(&self) -> Result<Self::Proto, Error> {
        Ok(proto::SignedVote {
            vote: Some(self.vote.to_proto()?),
            signature: Ctx::SigningScheme::encode_signature(&self.signature),
        })
    }
}

impl From<proto::VoteType> for VoteType {
    fn from(vote_type: proto::VoteType) -> Self {
        match vote_type {
            proto::VoteType::Prevote => VoteType::Prevote,
            proto::VoteType::Precommit => VoteType::Precommit,
        }
    }
}

impl From<VoteType> for proto::VoteType {
    fn from(vote_type: VoteType) -> proto::VoteType {
        match vote_type {
            VoteType::Prevote => proto::VoteType::Prevote,
            VoteType::Precommit => proto::VoteType::Precommit,
        }
    }
}

impl<Ctx: Context> Protobuf for SignedProposal<Ctx>
where
    Ctx::Proposal: Protobuf<Proto = proto::Proposal>,
{
    type Proto = proto::SignedProposal;

    fn from_proto(proto: Self::Proto) -> Result<Self, Error> {
        let proposal = proto
            .proposal
            .ok_or_else(|| Error::Other("Missing field `proposal`".to_string()))?;

        Ok(Self {
            proposal: Ctx::Proposal::from_proto(proposal)?,
            signature: Ctx::SigningScheme::decode_signature(&proto.signature)
                .map_err(|e| Error::Other(format!("Failed to decode signature: {e}")))?,
        })
    }

    fn to_proto(&self) -> Result<Self::Proto, Error> {
        Ok(proto::SignedProposal {
            proposal: Some(self.proposal.to_proto()?),
            signature: Ctx::SigningScheme::encode_signature(&self.signature),
        })
    }
}