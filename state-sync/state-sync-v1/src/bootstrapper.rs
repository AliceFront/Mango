// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0
use crate::{
    client::{CoordinatorMessage, StateSyncClient},
    coordinator::StateSyncCoordinator,
    executor_proxy::{ExecutorProxy, ExecutorProxyTrait},
    network::{StateSyncEvents, StateSyncSender},
};
use consensus_notifications::ConsensusNotificationListener;
use executor_types::ChunkExecutor;
use futures::channel::mpsc;
use mango_config::{config::NodeConfig, network_id::NodeNetworkId};
use mango_types::waypoint::Waypoint;
use mempool_notifications::MempoolNotificationSender;
use std::{boxed::Box, collections::HashMap, sync::Arc};
use storage_interface::DbReader;
use subscription_service::ReconfigSubscription;
use tokio::runtime::{Builder, Runtime};

/// Creates and bootstraps new state syncs and creates clients for
/// communicating with those state syncs.
pub struct StateSyncBootstrapper {
    _runtime: Runtime,
    coordinator_sender: mpsc::UnboundedSender<CoordinatorMessage>,
}

impl StateSyncBootstrapper {
    pub fn bootstrap<M: MempoolNotificationSender + 'static>(
        network: Vec<(NodeNetworkId, StateSyncSender, StateSyncEvents)>,
        mempool_notifier: M,
        consensus_listener: ConsensusNotificationListener,
        storage: Arc<dyn DbReader>,
        executor: Box<dyn ChunkExecutor>,
        node_config: &NodeConfig,
        waypoint: Waypoint,
        reconfig_event_subscriptions: Vec<ReconfigSubscription>,
    ) -> Self {
        let runtime = Builder::new_multi_thread()
            .thread_name("state-sync")
            .enable_all()
            .build()
            .expect("[State Sync] Failed to create runtime!");

        let executor_proxy = ExecutorProxy::new(storage, executor, reconfig_event_subscriptions);
        Self::bootstrap_with_executor_proxy(
            runtime,
            network,
            mempool_notifier,
            consensus_listener,
            node_config,
            waypoint,
            executor_proxy,
        )
    }

    pub fn bootstrap_with_executor_proxy<
        E: ExecutorProxyTrait + 'static,
        M: MempoolNotificationSender + 'static,
    >(
        runtime: Runtime,
        network: Vec<(NodeNetworkId, StateSyncSender, StateSyncEvents)>,
        mempool_notifier: M,
        consensus_listener: ConsensusNotificationListener,
        node_config: &NodeConfig,
        waypoint: Waypoint,
        executor_proxy: E,
    ) -> Self {
        let (coordinator_sender, coordinator_receiver) = mpsc::unbounded();
        let initial_state = executor_proxy
            .get_local_storage_state()
            .expect("[State Sync] Starting failure: cannot sync with storage!");
        let network_senders: HashMap<_, _> = network
            .iter()
            .map(|(network_id, sender, _events)| (network_id.clone(), sender.clone()))
            .collect();

        let coordinator = StateSyncCoordinator::new(
            coordinator_receiver,
            mempool_notifier,
            consensus_listener,
            network_senders,
            node_config,
            waypoint,
            executor_proxy,
            initial_state,
        )
        .expect("[State Sync] Unable to create state sync coordinator!");
        runtime.spawn(coordinator.start(network));

        Self {
            _runtime: runtime,
            coordinator_sender,
        }
    }

    pub fn create_client(&self) -> StateSyncClient {
        StateSyncClient::new(self.coordinator_sender.clone())
    }
}
