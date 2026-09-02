use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
struct SaveId(String);

#[derive(Debug, Deserialize, Serialize)]
struct SaveV3 {
    world_seed: u64,
    player: SaveId,
    actors: Vec<ActorV3>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ActorV3 {
    id: SaveId,
    archetype: String,
    position: [f32; 3],
}

fn main() {}
