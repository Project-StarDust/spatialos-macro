#[macro_use]
extern crate spatialos_macro;

mod generated {
    mod improbable {
        mod restricted {
            #[allow(unused_imports)]
            use std::collections::HashMap;
            #[derive(SpatialEnum, Clone, Debug)]
            pub enum ConnectionStatus {
                #[value(0u32)]
                Unknown,
                #[doc = " The worker requested a bridge from the receptionist, but the bridge has not yet had the worker connect to it."]
                #[value(1u32)]
                AwaitingWorkerConnection,
                #[doc = " The worker is connected to the bridge as normal."]
                #[value(2u32)]
                Connected,
                #[doc = " A worker was connected at one point, but is no longer connected. Currently, reconnecting is unsupported."]
                #[value(3u32)]
                Disconnected,
            }
            #[allow(dead_code)]
            #[doc = " Represents data relevant to the connection between the Runtime and the worker."]
            #[derive(SpatialType)]
            pub struct Connection {
                #[field_id(1u32)]
                #[spatial_type("enum")]
                status: crate::generated::improbable::restricted::ConnectionStatus,
                #[doc = " The latency measuring the round trip time for:"]
                #[doc = " 1. The runtime sending an op to a worker"]
                #[doc = " 2. The worker responding to that op"]
                #[doc = " 3. The runtime to process the response from the worker"]
                #[doc = " This is not network latency: it is an upper bound on network latency that also captures how backed up with ops a connection is."]
                #[doc = " 0 if the worker has not yet connected."]
                #[field_id(2u32)]
                #[spatial_type("uint32")]
                data_latency_ms: u32,
                #[doc = " The UNIX epoch time at which the worker connection was started. 0 if the worker has not yet connected."]
                #[field_id(3u32)]
                #[spatial_type("uint64")]
                connected_since_utc: u64,
            }
            #[allow(dead_code)]
            #[doc = " A request-response pair to disconnect a worker from a running deployment."]
            #[derive(SpatialType)]
            pub struct DisconnectRequest {}
            #[allow(dead_code)]
            #[derive(SpatialType)]
            pub struct DisconnectResponse {}
            #[allow(dead_code)]
            #[doc = " A bundle of data that can be used to uniquely identify a player."]
            #[derive(SpatialType)]
            pub struct PlayerIdentity {
                #[doc = " A player identifier is unique within the context of a single provider."]
                #[field_id(1u32)]
                #[spatial_type("string")]
                player_identifier: String,
                #[doc = " The provider is the system that was used to authenticate the user."]
                #[field_id(2u32)]
                #[spatial_type("string")]
                provider: String,
                #[doc = " Arbitrary metadata that can be associated with a player identity by a login service when"]
                #[doc = " the player connects."]
                #[doc = " This is completely opaque to SpatialOS and its meaning is defined by users."]
                #[field_id(3u32)]
                #[spatial_type("bytes")]
                metadata: Vec<u8>,
            }
            #[allow(dead_code)]
            #[doc = " This file contains system components, part of the restricted components package."]
            #[doc = " These components contain data that correspond to SpatialOS Runtime systems."]
            #[doc = " Workers will never gain authority over these components."]
            #[doc = " Workers may not create or delete entities that have these components on them."]
            #[doc = " Workers may issue commands against these components, but require the \\\"system_entity_command\\\" permission."]
            #[doc = " These command requests are handled by the Runtime rather than routed to an authoritative worker, as"]
            #[doc = " workers may never be authoritative over these components."]
            #[doc = " The System component is a marker component used to indicate that an entity corresponds to a"]
            #[doc = " SpatialOS runtime system entity."]
            #[doc = " It is present on all entities with any of the components below."]
            #[derive(SpatialComponent)]
            #[id(59u32)]
            pub struct System {}
            #[allow(dead_code)]
            #[doc = " The Worker component indicates that the system entity it is on represents a worker."]
            #[doc = " It carries metadata identifying that worker."]
            #[derive(SpatialComponent)]
            #[id(60u32)]
            pub struct Worker {
                #[field_id(1u32)]
                #[spatial_type("string")]
                worker_id: String,
                #[field_id(2u32)]
                #[spatial_type("string")]
                worker_type: String,
                #[field_id(3u32)]
                #[spatial_type("type")]
                connection: crate::generated::improbable::restricted::Connection,
            }
            #[allow(dead_code)]
            #[doc = " The PlayerClient component is present on worker entities that correspond to player client workers."]
            #[doc = " These are identified by the Runtime as workers that have connected with a player identity token."]
            #[doc = " The contents of this token are exposed in this component."]
            #[derive(SpatialComponent)]
            #[id(61u32)]
            pub struct PlayerClient {
                #[field_id(1u32)]
                #[spatial_type("type")]
                player_identity: crate::generated::improbable::restricted::PlayerIdentity,
            }
        }
    }
}

fn main() {}
