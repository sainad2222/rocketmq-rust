/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::protocol::command_custom_header::{CommandCustomHeader, FromMap};

/// Represents the header for a broker registration request.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterBrokerRequestHeader {
    /// The name of the broker.
    #[serde(rename = "brokerName")]
    pub broker_name: String,

    /// The address of the broker.
    #[serde(rename = "brokerAddr")]
    pub broker_addr: String,

    /// The name of the cluster to which the broker belongs.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,

    /// The address of the highly available (HA) server associated with the broker.
    #[serde(rename = "haServerAddr")]
    pub ha_server_addr: String,

    /// The unique identifier for the broker.
    #[serde(rename = "brokerId")]
    pub broker_id: i64,

    /// The optional heartbeat timeout in milliseconds.
    #[serde(rename = "heartbeatTimeoutMillis")]
    pub heartbeat_timeout_millis: Option<i64>,

    /// The optional flag indicating whether acting as the master is enabled.
    #[serde(rename = "enableActingMaster")]
    pub enable_acting_master: Option<bool>,

    /// Indicates whether the data is compressed.
    pub compressed: bool,

    /// The CRC32 checksum for the message body.
    #[serde(rename = "bodyCrc32")]
    pub body_crc32: u32,
}

impl RegisterBrokerRequestHeader {
    const BROKER_NAME: &'static str = "brokerName";
    const BROKER_ADDR: &'static str = "brokerAddr";
    const CLUSTER_NAME: &'static str = "clusterName";
    const HA_SERVER_ADDR: &'static str = "haServerAddr";
    const BROKER_ID: &'static str = "brokerId";
    const HEARTBEAT_TIMEOUT_MILLIS: &'static str = "heartbeatTimeoutMillis";
    const ENABLE_ACTING_MASTER: &'static str = "enableActingMaster";
    const COMPRESSED: &'static str = "compressed";
    const BODY_CRC32: &'static str = "bodyCrc32";

    /// Creates a new instance of `RegisterBrokerRequestHeader`.
    ///
    /// # Arguments
    ///
    /// * `broker_name` - The name of the broker.
    /// * `broker_addr` - The address of the broker.
    /// * `cluster_name` - The name of the cluster.
    /// * `ha_server_addr` - The address of the HA server.
    /// * `broker_id` - The unique identifier for the broker.
    /// * `heartbeat_timeout_millis` - The optional heartbeat timeout in milliseconds.
    /// * `enable_acting_master` - The optional flag indicating whether acting as the master is
    ///   enabled.
    /// * `compressed` - Indicates whether the data is compressed.
    /// * `body_crc32` - The CRC32 checksum for the message body.
    ///
    /// # Returns
    ///
    /// A new `RegisterBrokerRequestHeader` instance.
    pub fn new(
        broker_name: String,
        broker_addr: String,
        cluster_name: String,
        ha_server_addr: String,
        broker_id: i64,
        heartbeat_timeout_millis: Option<i64>,
        enable_acting_master: Option<bool>,
        compressed: bool,
        body_crc32: u32,
    ) -> Self {
        RegisterBrokerRequestHeader {
            broker_name,
            broker_addr,
            cluster_name,
            ha_server_addr,
            broker_id,
            heartbeat_timeout_millis,
            enable_acting_master,
            compressed,
            body_crc32,
        }
    }
}

impl FromMap for RegisterBrokerRequestHeader {
    type Target = Self;

    fn from(map: &HashMap<String, String>) -> Option<Self::Target> {
        Some(RegisterBrokerRequestHeader {
            broker_name: map
                .get(RegisterBrokerRequestHeader::BROKER_NAME)
                .map(|s| s.to_string())
                .unwrap_or_default(),
            broker_addr: map
                .get(RegisterBrokerRequestHeader::BROKER_ADDR)
                .map(|s| s.to_string())
                .unwrap_or_default(),
            cluster_name: map
                .get(RegisterBrokerRequestHeader::CLUSTER_NAME)
                .map(|s| s.to_string())
                .unwrap_or_default(),
            ha_server_addr: map
                .get(RegisterBrokerRequestHeader::HA_SERVER_ADDR)
                .map(|s| s.to_string())
                .unwrap_or_default(),
            broker_id: map
                .get(RegisterBrokerRequestHeader::BROKER_ID)
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0),
            heartbeat_timeout_millis: map
                .get(RegisterBrokerRequestHeader::HEARTBEAT_TIMEOUT_MILLIS)
                .and_then(|s| s.parse::<i64>().ok()),
            enable_acting_master: map
                .get(RegisterBrokerRequestHeader::ENABLE_ACTING_MASTER)
                .and_then(|s| s.parse::<bool>().ok()),
            compressed: map
                .get(RegisterBrokerRequestHeader::COMPRESSED)
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(false),
            body_crc32: map
                .get(RegisterBrokerRequestHeader::BODY_CRC32)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0),
        })
    }
}

impl CommandCustomHeader for RegisterBrokerRequestHeader {
    fn check_fields(&self) -> anyhow::Result<(), Error> {
        Ok(())
    }

    fn to_map(&self) -> Option<HashMap<String, String>> {
        let mut map = HashMap::new();

        map.insert(
            RegisterBrokerRequestHeader::BROKER_NAME.to_string(),
            self.broker_name.clone(),
        );
        map.insert(
            RegisterBrokerRequestHeader::BROKER_ADDR.to_string(),
            self.broker_addr.clone(),
        );
        map.insert(
            RegisterBrokerRequestHeader::CLUSTER_NAME.to_string(),
            self.cluster_name.clone(),
        );
        map.insert(
            RegisterBrokerRequestHeader::HA_SERVER_ADDR.to_string(),
            self.ha_server_addr.clone(),
        );
        map.insert(
            RegisterBrokerRequestHeader::BROKER_ID.to_string(),
            self.broker_id.to_string(),
        );

        if let Some(heartbeat_timeout) = self.heartbeat_timeout_millis {
            map.insert(
                RegisterBrokerRequestHeader::HEARTBEAT_TIMEOUT_MILLIS.to_string(),
                heartbeat_timeout.to_string(),
            );
        }

        if let Some(enable_acting_master) = self.enable_acting_master {
            map.insert(
                RegisterBrokerRequestHeader::ENABLE_ACTING_MASTER.to_string(),
                enable_acting_master.to_string(),
            );
        }

        map.insert(
            RegisterBrokerRequestHeader::COMPRESSED.to_string(),
            self.compressed.to_string(),
        );
        map.insert(
            RegisterBrokerRequestHeader::BODY_CRC32.to_string(),
            self.body_crc32.to_string(),
        );

        Some(map)
    }
}
