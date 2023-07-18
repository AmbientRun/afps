use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        physics::cube_collider,
        primitives::{cube,quad},
        rendering::color,
        transform::{lookat_target, translation, rotation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use concepts::{make_laser_box, make_laser_beam, };
use components::{
    laserbox,
    laserbox_laser_rot,
    laserbox_dir,
    laserbox_range,
    laserbox_enabled,
    
    laserbox_laser_ent,
    
    laserbeam_origin,
    laserbeam_dir,
    laserbeam_range,
};

#[main]
pub fn main() {
    messages::SpawnLaserbox::subscribe(|_src,msg|{
        let laserbox = Entity::new()
            .with_merge(make_transformable())
            .with_merge(make_laser_box())
            .with(cube(), ())
            .with(cube_collider(), Vec3::splat(1.))
            .with(translation(), msg.position)
            .with(rotation(), msg.facing_rot)
            .with(laserbox_laser_rot(), msg.facing_rot)
            .with(laserbox_dir(), msg.facing_dir)
            .spawn();
        entity::set_component(laserbox, laserbox_enabled(), true);
        add_laser_ent_to(laserbox);
        
    });

    messages::Shoot::subscribe(move |_source, msg| {
        println!("Shot!");
        let result = physics::raycast_first(msg.ray_origin, msg.ray_dir);

        if let Some(hit) = result {
            println!("Hit somethin'!");
            if let Some(was_enabled) = entity::get_component(hit.entity, laserbox_enabled()) {
                let laserbox = hit.entity;
                let now_enabled = !was_enabled;
                entity::set_component(laserbox, laserbox_enabled(), now_enabled);
                println!("Toggle laser on or off: {}!", now_enabled);
                if now_enabled {
                    add_laser_ent_to(laserbox);
                } else {
                    try_remove_laser_ent_from(laserbox);
                }
            }
        }
    });

    change_query((laserbox_enabled(), laserbox_laser_ent())).track_change(laserbox_enabled()).bind(|laserboxes|{
        for (laserbox, (enabled, _ent)) in laserboxes {
            println!("Toggle laser on or off: {enabled}!");
            if enabled {
                add_laser_ent_to(laserbox);
            } else {
                try_remove_laser_ent_from(laserbox);
            }
        }
    });

    // query((laserbeam_origin(), laserbeam_dir(), laserbeam_range())).each_frame(|lasers|{
    //     for (laser,(origin,dir,_range)) in lasers {
    //         messages::Shoot{
    //             ray_origin: origin + dir.normalize()*1.5,
    //             ray_dir: dir,
    //             source: laser,
    //         }.send_local_broadcast(false);
    //     }
    // });

}

fn add_laser_ent_to(laserbox : EntityId) -> EntityId {
    try_remove_laser_ent_from(laserbox); // remove old
    
    let pos = entity::get_component(laserbox, translation()).unwrap();
    let dir = entity::get_component(laserbox, laserbox_dir()).unwrap();
    let rot = entity::get_component(laserbox, laserbox_laser_rot()).unwrap();
    let range = entity::get_component(laserbox, laserbox_range()).unwrap();
    let laser_ent = Entity::new()
        .with_merge(make_transformable())
        .with(cube(), ())
        .with(translation(), pos + dir * range * 0.5)
        .with(rotation(), rot)
        .with(scale(), vec3(0.1, range, 0.1))
        .with(color(), vec4(1.,0.,1.,1.))
        .with_merge(make_laser_beam())
        .with(laserbeam_origin(), pos)
        .with(laserbeam_dir(), dir)
        .with(laserbeam_range(), range)
        .spawn();
    entity::add_component(laserbox, laserbox_laser_ent(), laser_ent);
    laser_ent
}
fn try_remove_laser_ent_from(laserbox : EntityId) -> bool {
    if let Some(laser_ent) = entity::get_component(laserbox, laserbox_laser_ent()) {
        entity::despawn(laser_ent);
        entity::remove_component(laserbox, laserbox_laser_ent());
        true
    } else { false }
}