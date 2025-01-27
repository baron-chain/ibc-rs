//! [`ChannelQueryService`](ChannelQueryService) takes a generic `I` to store `ibc_context` that implements [`QueryContext`](QueryContext).
//! `I` must be a type where writes from one thread are readable from another.
//! This means using `Arc<Mutex<_>>` or `Arc<RwLock<_>>` in most cases.

use ibc::core::host::ValidationContext;
use ibc::core::primitives::prelude::*;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::query_server::Query as ChannelQuery;
use ibc_proto::ibc::core::channel::v1::{
    QueryChannelClientStateRequest, QueryChannelClientStateResponse,
    QueryChannelConsensusStateRequest, QueryChannelConsensusStateResponse, QueryChannelRequest,
    QueryChannelResponse, QueryChannelsRequest, QueryChannelsResponse,
    QueryConnectionChannelsRequest, QueryConnectionChannelsResponse,
    QueryNextSequenceReceiveRequest, QueryNextSequenceReceiveResponse,
    QueryNextSequenceSendRequest, QueryNextSequenceSendResponse, QueryPacketAcknowledgementRequest,
    QueryPacketAcknowledgementResponse, QueryPacketAcknowledgementsRequest,
    QueryPacketAcknowledgementsResponse, QueryPacketCommitmentRequest,
    QueryPacketCommitmentResponse, QueryPacketCommitmentsRequest, QueryPacketCommitmentsResponse,
    QueryPacketReceiptRequest, QueryPacketReceiptResponse, QueryUnreceivedAcksRequest,
    QueryUnreceivedAcksResponse, QueryUnreceivedPacketsRequest, QueryUnreceivedPacketsResponse,
};
use tonic::{Request, Response, Status};

use super::{
    query_channel, query_channel_client_state, query_channel_consensus_state, query_channels,
    query_connection_channels, query_next_sequence_receive, query_next_sequence_send,
    query_packet_acknowledgement, query_packet_acknowledgements, query_packet_commitment,
    query_packet_commitments, query_packet_receipt, query_unreceived_acks,
    query_unreceived_packets,
};
use crate::core::context::QueryContext;

// TODO(rano): currently the services don't support pagination, so we return all the results.

/// The generic `I` must be a type where writes from one thread are readable from another.
/// This means using `Arc<Mutex<_>>` or `Arc<RwLock<_>>` in most cases.
pub struct ChannelQueryService<I>
where
    I: QueryContext + Send + Sync + 'static,
    <I as ValidationContext>::AnyClientState: Into<Any>,
    <I as ValidationContext>::AnyConsensusState: Into<Any>,
{
    ibc_context: I,
}

impl<I> ChannelQueryService<I>
where
    I: QueryContext + Send + Sync + 'static,
    <I as ValidationContext>::AnyClientState: Into<Any>,
    <I as ValidationContext>::AnyConsensusState: Into<Any>,
{
    /// The parameter `ibc_context` must be a type where writes from one thread are readable from another.
    /// This means using `Arc<Mutex<_>>` or `Arc<RwLock<_>>` in most cases.
    pub fn new(ibc_context: I) -> Self {
        Self { ibc_context }
    }
}

#[tonic::async_trait]
impl<I> ChannelQuery for ChannelQueryService<I>
where
    I: QueryContext + Send + Sync + 'static,
    <I as ValidationContext>::AnyClientState: Into<Any>,
    <I as ValidationContext>::AnyConsensusState: Into<Any>,
{
    async fn channel(
        &self,
        request: Request<QueryChannelRequest>,
    ) -> Result<Response<QueryChannelResponse>, Status> {
        let response = query_channel(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn channels(
        &self,
        request: Request<QueryChannelsRequest>,
    ) -> Result<Response<QueryChannelsResponse>, Status> {
        let response = query_channels(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn connection_channels(
        &self,
        request: Request<QueryConnectionChannelsRequest>,
    ) -> Result<Response<QueryConnectionChannelsResponse>, Status> {
        let response = query_connection_channels(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn channel_client_state(
        &self,
        request: Request<QueryChannelClientStateRequest>,
    ) -> Result<Response<QueryChannelClientStateResponse>, Status> {
        let response = query_channel_client_state(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn channel_consensus_state(
        &self,
        request: Request<QueryChannelConsensusStateRequest>,
    ) -> Result<Response<QueryChannelConsensusStateResponse>, Status> {
        let response = query_channel_consensus_state(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn packet_commitment(
        &self,
        request: Request<QueryPacketCommitmentRequest>,
    ) -> Result<Response<QueryPacketCommitmentResponse>, Status> {
        let response = query_packet_commitment(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn packet_commitments(
        &self,
        request: Request<QueryPacketCommitmentsRequest>,
    ) -> Result<Response<QueryPacketCommitmentsResponse>, Status> {
        let response = query_packet_commitments(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn packet_receipt(
        &self,
        request: Request<QueryPacketReceiptRequest>,
    ) -> Result<Response<QueryPacketReceiptResponse>, Status> {
        let response = query_packet_receipt(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn packet_acknowledgement(
        &self,
        request: Request<QueryPacketAcknowledgementRequest>,
    ) -> Result<Response<QueryPacketAcknowledgementResponse>, Status> {
        let response = query_packet_acknowledgement(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    /// Returns all the acknowledgements if sequences is omitted.
    async fn packet_acknowledgements(
        &self,
        request: Request<QueryPacketAcknowledgementsRequest>,
    ) -> Result<Response<QueryPacketAcknowledgementsResponse>, Status> {
        let response = query_packet_acknowledgements(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    /// Returns all the unreceived packets if sequences is omitted.
    async fn unreceived_packets(
        &self,
        request: Request<QueryUnreceivedPacketsRequest>,
    ) -> Result<Response<QueryUnreceivedPacketsResponse>, Status> {
        let response = query_unreceived_packets(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    /// Returns all the unreceived acknowledgements if sequences is omitted.
    async fn unreceived_acks(
        &self,
        request: Request<QueryUnreceivedAcksRequest>,
    ) -> Result<Response<QueryUnreceivedAcksResponse>, Status> {
        let response = query_unreceived_acks(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn next_sequence_receive(
        &self,
        request: Request<QueryNextSequenceReceiveRequest>,
    ) -> Result<Response<QueryNextSequenceReceiveResponse>, Status> {
        let response = query_next_sequence_receive(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }

    async fn next_sequence_send(
        &self,
        request: Request<QueryNextSequenceSendRequest>,
    ) -> Result<Response<QueryNextSequenceSendResponse>, Status> {
        let response = query_next_sequence_send(&self.ibc_context, request.get_ref())?;

        Ok(Response::new(response))
    }
}
