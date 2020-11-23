#[macro_use]
extern crate spatialos_macro;

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(dead_code)]
#[doc = " A set of worker attributes. This represents a minimum set of requirements that a candidate worker"]
#[doc = " must fulfill in order to be eligible for some constraint, and corresponds to the concrete set of"]
#[doc = " capabilities defined in each worker's JSON configuration file."]
#[derive(SpatialType)]
pub struct WorkerAttributeSet {
    #[doc = " A particular capability is just an arbitrary string. A particular worker's attribute set must"]
    #[doc = " contain _all_ of these attributes in order to satisfy this WorkerAttributeSet."]
    #[field_id(1)]
    attribute: Vec<String>,
}

#[allow(dead_code)]
#[doc = " A WorkerRequirementSet is a way to describe a set of workers. We can use a WorkerRequirementSet"]
#[doc = " to, for example, describe the sorts of workers that are allowed to be authoritative on a"]
#[doc = " particular component."]
#[doc = " For example, we might want an entity to be readable on any worker that can handle \"visual\" or"]
#[doc = " \"physical\" things, and could describe that with a WorkerRequirementSet containing two"]
#[doc = " WorkerAttributeSets:"]
#[doc = "   {"]
#[doc = "     { \"visual\" },"]
#[doc = "     { \"physical\" }"]
#[doc = "   }"]
#[derive(SpatialType)]
pub struct WorkerRequirementSet {
    #[doc = " A worker satisfies this WorkerRequirementSet if it satisfies _any_ of these"]
    #[doc = " WorkerAttributeSets (i.e. if any one of these WorkerAttributeSets is a subset of the worker's"]
    #[doc = " attributes)."]
    #[field_id(1)]
    attribute_set: Vec<WorkerAttributeSet>,
}

#[allow(dead_code)]
#[doc = " A type representing a 3-dimensional position in space. This is primarily used as part of the"]
#[doc = " standard Position component, below, but can also be reused for other purposes."]
#[derive(SpatialType)]
pub struct Coordinates {
    #[field_id(1)]
    x: f64,
    #[field_id(2)]
    y: f64,
    #[field_id(3)]
    z: f64,
}

#[allow(dead_code)]
#[doc = " A type representing the dimensions of a cuboid."]
#[derive(SpatialType)]
pub struct EdgeLength {
    #[field_id(1)]
    x: f64,
    #[field_id(2)]
    y: f64,
    #[field_id(3)]
    z: f64,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct Query {
    #[field_id(1)]
    constraint: QueryConstraint,
    #[doc = " Either full_snapshot_result or a list of result_component_id should be provided. Providing both is invalid."]
    #[field_id(2)]
    full_snapshot_result: Option<bool>,
    #[field_id(3)]
    result_component_id: Vec<u32>,
    #[doc = " Used for frequency-based rate limiting. Represents the maximum frequency of updates for this"]
    #[doc = " particular query. An empty option represents no rate-limiting (ie. updates are received"]
    #[doc = " as soon as possible). Frequency is measured in Hz."]
    #[doc = " If set, the time between consecutive updates will be at least 1/frequency. This is determined"]
    #[doc = " at the time that updates are sent from the Runtime and may not necessarily correspond to the"]
    #[doc = " time updates are received by the worker."]
    #[doc = " If after an update has been sent, multiple updates are applied to a component, they will be"]
    #[doc = " merged and sent as a single update after 1/frequency of the last sent update. When components"]
    #[doc = " with events are merged, the resultant component will contain a concatenation of all the"]
    #[doc = " events."]
    #[doc = " If multiple queries match the same Entity-Component then the highest of all frequencies is"]
    #[doc = " used."]
    #[field_id(4)]
    frequency: Option<f32>,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct QueryConstraint {
    #[doc = " Only one constraint should be provided. Providing more than one is invalid."]
    #[field_id(1)]
    sphere_constraint: Option<SphereConstraint>,
    #[field_id(2)]
    cylinder_constraint: Option<CylinderConstraint>,
    #[field_id(3)]
    box_constraint: Option<BoxConstraint>,
    #[field_id(4)]
    relative_sphere_constraint: Option<RelativeSphereConstraint>,
    #[field_id(5)]
    relative_cylinder_constraint: Option<RelativeCylinderConstraint>,
    #[field_id(6)]
    relative_box_constraint: Option<RelativeBoxConstraint>,
    #[field_id(7)]
    entity_id_constraint: Option<i64>,
    #[field_id(8)]
    component_constraint: Option<u32>,
    #[field_id(9)]
    and_constraint: Vec<QueryConstraint>,
    #[field_id(10)]
    or_constraint: Vec<QueryConstraint>,
    #[doc = " reserved = 11"]
    #[field_id(12)]
    self_constraint: Option<SelfConstraint>,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct SphereConstraint {
    #[field_id(1)]
    center: Coordinates,
    #[field_id(2)]
    radius: f64,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct CylinderConstraint {
    #[field_id(1)]
    center: Coordinates,
    #[field_id(2)]
    radius: f64,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct BoxConstraint {
    #[field_id(1)]
    center: Coordinates,
    #[field_id(2)]
    edge_length: EdgeLength,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct RelativeSphereConstraint {
    #[field_id(1)]
    radius: f64,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct RelativeCylinderConstraint {
    #[field_id(1)]
    radius: f64,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct RelativeBoxConstraint {
    #[field_id(1)]
    edge_length: EdgeLength,
}

#[allow(dead_code)]
#[doc = " The self constraint matches the entity the Interest query is attached to."]
#[derive(SpatialType)]
pub struct SelfConstraint {}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct ComponentInterest {
    #[field_id(1)]
    queries: Vec<Query>,
}

#[allow(dead_code)]
#[derive(SpatialType)]
pub struct ShardedMap {}

#[allow(dead_code)]
#[doc = " The EntityAcl component defines what sorts of workers can read and write each entity in the"]
#[doc = " simulation. This component is REQUIRED (every entity must be created with it)."]
#[derive(SpatialComponent)]
#[id(50)]
pub struct EntityAcl {
    #[doc = " The read ACL defined the kinds of workers that may check out the entity. Note that a worker"]
    #[doc = " is currently required to satisfy this constraint even if it is authoritative on some component"]
    #[doc = " on this entity; i.e. an entity will _never_ be checked out on any worker that does not match"]
    #[doc = " this WorkerRequirementSet."]
    #[field_id(1)]
    read_acl: WorkerRequirementSet,
    #[doc = " This map defines the kinds of worker that are allowed to be authoritative on each component,"]
    #[doc = " where components are keyed by their component ID (as defined in schema and generated code)."]
    #[doc = " A component does not have to have an ACL, in which case it can't be authoritative on any"]
    #[doc = " worker."]
    #[field_id(2)]
    component_write_acl: HashMap<u32, WorkerRequirementSet>,
}

#[allow(dead_code)]
#[doc = " The Metadata component is used to hold debug and convenience information about"]
#[doc = " the entity. This component is optional."]
#[derive(SpatialComponent)]
#[id(53)]
pub struct Metadata {
    #[doc = " The entity type is a string describing what kind of thing the entity represents"]
    #[doc = " in the simulation. It is used by the Inspector to colour or filter entities"]
    #[doc = " based on their entity type, for example \"car\" or \"player\"."]
    #[field_id(1)]
    entity_type: String,
}

#[allow(dead_code)]
#[doc = " The Position component defines the canonical position of an entity inside a SpatialOS simulation."]
#[doc = " This is used by SpatialOS for load-balancing, authority delegation, and spatial queries. Note"]
#[doc = " that although this component can be used to represent an entity's position on workers, it doesn't"]
#[doc = " _have_ to be: it's completely reasonable for a simulation to define a custom or optimized notion"]
#[doc = " of position, and simply update this component as necessary (perhaps less frequently) for"]
#[doc = " authority delegation."]
#[derive(SpatialComponent)]
#[id(54)]
pub struct Position {
    #[field_id(1)]
    coords: Coordinates,
}

#[allow(dead_code)]
#[doc = " The Peristence component is a marker component used to indicate that an entity should be"]
#[doc = " persisted in simulation snapshots. Any entity without this component will be dropped when a"]
#[doc = " snapshot is taken."]
#[derive(SpatialComponent)]
#[id(55)]
pub struct Persistence {}

#[allow(dead_code)]
#[doc = " An entity's interest is a map of Component IDs to a list of Entity queries, where the queries define other Entities"]
#[doc = " needed to simulate the component."]
#[doc = " If a Worker is authoritative over a Component ID present in the map, it will be provided with updates for Entities"]
#[doc = " which match the corresponding queries."]
#[derive(SpatialComponent)]
#[id(58)]
pub struct Interest {
    #[field_id(1)]
    component_interest: HashMap<u32, ComponentInterest>,
}

fn main() {}
