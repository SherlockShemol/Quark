// Copyright (c) 2021 Quark Container Authors / 2018 The gVisor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub const EVENT_TYPE_SET: &str = "set";
pub const EVENT_TYPE_DELETE: &str = "delete";

pub const GRPC_SERVER_ADDRESS: &str = "http://[::1]:51051";

pub const RDMA_SVC_CLIENT_ROLE_NORMAL:i32 = 1;
pub const RDMA_SVC_CLIENT_ROLE_EGRESS:i32 = 2;
pub const RDMA_SVC_CLIENT_ROLE_INGRESS:i32 = 3;
