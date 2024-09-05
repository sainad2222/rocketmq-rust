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

use serde::Deserialize;
use serde::Serialize;

use crate::protocol::command_custom_header::CommandCustomHeader;
use crate::protocol::command_custom_header::FromMap;
use crate::protocol::header::message_operation_header::TopicRequestHeaderTrait;
use crate::rpc::topic_request_header::TopicRequestHeader;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxOffsetRequestHeader {
    pub topic: String,

    pub queue_id: i32,

    pub committed: bool,

    #[serde(flatten)]
    pub topic_request_header: Option<TopicRequestHeader>,
}

impl Default for GetMaxOffsetRequestHeader {
    fn default() -> Self {
        GetMaxOffsetRequestHeader {
            topic: Default::default(),
            queue_id: Default::default(),
            committed: true,
            topic_request_header: Default::default(),
        }
    }
}

impl GetMaxOffsetRequestHeader {
    pub const TOPIC: &'static str = "topic";
    pub const QUEUE_ID: &'static str = "queueId";
    pub const COMMITTED: &'static str = "committed";
}

impl CommandCustomHeader for GetMaxOffsetRequestHeader {
    fn to_map(&self) -> Option<HashMap<String, String>> {
        let mut map = HashMap::new();
        map.insert(Self::TOPIC.to_string(), self.topic.clone());
        map.insert(Self::QUEUE_ID.to_string(), self.queue_id.to_string());
        map.insert(Self::COMMITTED.to_string(), self.committed.to_string());
        if let Some(topic_request_header) = &self.topic_request_header {
            if let Some(topic_request_header_map) = topic_request_header.to_map() {
                map.extend(topic_request_header_map);
            }
        }
        Some(map)
    }
}

impl FromMap for GetMaxOffsetRequestHeader {
    type Target = Self;

    fn from(map: &HashMap<String, String>) -> Option<Self::Target> {
        Some(GetMaxOffsetRequestHeader {
            topic: map
                .get(GetMaxOffsetRequestHeader::TOPIC)
                .map(|s| s.to_string())
                .unwrap_or_default(),
            queue_id: map
                .get(GetMaxOffsetRequestHeader::QUEUE_ID)
                .map(|s| s.parse().unwrap())
                .unwrap_or_default(),
            committed: map
                .get(GetMaxOffsetRequestHeader::COMMITTED)
                .map(|s| s.parse().unwrap())
                .unwrap_or(true),
            topic_request_header: <TopicRequestHeader as FromMap>::from(map),
        })
    }
}

impl TopicRequestHeaderTrait for GetMaxOffsetRequestHeader {
    fn set_lo(&mut self, lo: Option<bool>) {
        self.topic_request_header.as_mut().unwrap().lo = lo;
    }

    fn lo(&self) -> Option<bool> {
        self.topic_request_header.as_ref().unwrap().lo
    }

    fn set_topic(&mut self, topic: String) {
        self.topic = topic;
    }

    fn topic(&self) -> &str {
        self.topic.as_str()
    }

    fn broker_name(&self) -> Option<&str> {
        self.topic_request_header
            .as_ref()
            .unwrap()
            .rpc_request_header
            .as_ref()
            .unwrap()
            .broker_name
            .as_deref()
    }

    fn set_broker_name(&mut self, broker_name: String) {
        self.topic_request_header
            .as_mut()
            .unwrap()
            .rpc_request_header
            .as_mut()
            .unwrap()
            .broker_name = Some(broker_name);
    }

    fn namespace(&self) -> Option<&str> {
        self.topic_request_header
            .as_ref()
            .unwrap()
            .rpc_request_header
            .as_ref()
            .unwrap()
            .namespace
            .as_deref()
    }

    fn set_namespace(&mut self, namespace: String) {
        self.topic_request_header
            .as_mut()
            .unwrap()
            .rpc_request_header
            .as_mut()
            .unwrap()
            .namespace = Some(namespace);
    }

    fn namespaced(&self) -> Option<bool> {
        self.topic_request_header
            .as_ref()
            .unwrap()
            .rpc_request_header
            .as_ref()
            .unwrap()
            .namespaced
    }

    fn set_namespaced(&mut self, namespaced: bool) {
        self.topic_request_header
            .as_mut()
            .unwrap()
            .rpc_request_header
            .as_mut()
            .unwrap()
            .namespaced = Some(namespaced);
    }

    fn oneway(&self) -> Option<bool> {
        self.topic_request_header
            .as_ref()
            .unwrap()
            .rpc_request_header
            .as_ref()
            .unwrap()
            .oneway
    }

    fn set_oneway(&mut self, oneway: bool) {
        self.topic_request_header
            .as_mut()
            .unwrap()
            .rpc_request_header
            .as_mut()
            .unwrap()
            .oneway = Some(oneway);
    }

    fn queue_id(&self) -> Option<i32> {
        Some(self.queue_id)
    }

    fn set_queue_id(&mut self, queue_id: Option<i32>) {
        self.queue_id = queue_id.unwrap_or_default();
    }
}