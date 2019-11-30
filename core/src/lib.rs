//! The `morgan` library implements the Morgan high-performance blockchain architecture.
//! It includes a full Rust implementation of the architecture (see
//! [Validator](server/struct.Validator.html)) as well as hooks to GPU implementations of its most
//! paralellizable components (i.e. [SigVerify](sigverify/index.html)).  It also includes
//! command-line tools to spin up fullnodes and a Rust library
//!

// pub mod bank_forks;
pub mod treasuryForks;
pub mod treasuryStage;
pub mod fetchSpotStage;
pub mod propagateStage;
#[cfg(feature = "chacha")]
pub mod chacha;
#[cfg(all(feature = "chacha", feature = "cuda"))]
pub mod chachaCUDA;
pub mod ClusterVoteMessageListener;
#[macro_use]
pub mod connectionInfo;
pub mod connectionInfoTable;
pub mod gossip;
pub mod gossipErrorType;
pub mod pullFromGossip;
pub mod pushToGossip;
pub mod propagationValue;
#[macro_use]
pub mod blockBufferPool;
pub mod blockStream;
pub mod blockStreamService;
pub mod blockBufferPoolProcessor;
pub mod cluster;
pub mod clusterMessage;
pub mod ClusterFixMessageListener;
pub mod clusterTests;
pub mod entryInfo;
pub mod expunge;
pub mod fetchStage;
pub mod createKeys;
pub mod genesisUtils;
pub mod gossipService;
pub mod leaderArrange;
pub mod leaderArrangeCache;
pub mod leaderArrangeUtils;
pub mod localCluster;
pub mod localVoteSignerService;
pub mod forkSelection;
pub mod packet;
pub mod waterClock;
pub mod waterClockRecorder;
pub mod waterClockService;
pub mod recvmmsg;
pub mod fixMissingSpotService;
pub mod repeatStage;
pub mod cloner;
pub mod result;
pub mod retransmitStage;
pub mod rpc;
pub mod rpcPubsub;
pub mod rpcPubSsubService;
pub mod rpcService;
pub mod rpcSubscriptions;
pub mod service;
pub mod signatureVerify;
pub mod signatureVerifyStage;
pub mod stakingUtils;
pub mod storageStage;
pub mod streamer;
pub mod testTx;
pub mod transactionProcessCentre;
pub mod transactionVerifyCentre;
pub mod verifier;
pub mod spotTransmitService;

#[macro_use]
extern crate morgan_budget_controller;

#[cfg(test)]
#[cfg(any(feature = "chacha", feature = "cuda"))]
#[macro_use]
extern crate hex_literal;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate morgan_metricbot;

#[cfg(test)]
#[macro_use]
extern crate matches;
