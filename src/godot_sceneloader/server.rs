

mod sceneloader;

#[main]
pub fn main() {
    let nodes = sceneloader::test_get_nodes();

    sceneloader::debug_nodes(&nodes);

    for (_key, node) in nodes {
        let node_pos : Option<Vec3> = node.pos;
        let node_rot : Option<Quat> = node.rot;
        let node_siz : Option<Vec3> = node.siz;
        match node.name.as_str() {
            "cube" | "cube2"=> {
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(translation(), node_pos.unwrap())
                    .with(rotation(), node_rot.unwrap())
                    .with(scale(), node_siz.unwrap())
                    .spawn();
            },
            "laserbox" | "laserbox2" => {
                let forward = node_rot.unwrap() * vec3(0., 1., 0.);
                messages::SpawnLaserbox{ position:node_pos.unwrap(), facing_rot: node_rot.unwrap(), facing_dir:forward }.send_local_broadcast(false);
            },
            _=>{}, // do nothing with other nodes
        }
    }
}

use ambient_api::{
    components::core::{
        primitives::cube,
        transform::{translation, rotation, scale},
    },
    concepts::make_transformable,
    prelude::*,
};







