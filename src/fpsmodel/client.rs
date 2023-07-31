use ambient_api::{
    animation::{get_bone_by_bind_id, BindId},
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children, parent},
        model::model_loaded,
        player::user_id,
        prefab::prefab_from_url,
        transform::reset_scale,
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use ambient_api::components::core::{
    player::player,
    // primitives::quad,
    rendering::color, //pbr_material_from_url
    transform::{local_to_parent, rotation, scale, translation},
};

#[main]
pub fn main() {
    spawn_query((player(), components::player_model_ref(), user_id())).bind(move |results| {
        for (id, (_, model, uid)) in results {
            run_async(async move {
                entity::wait_for_component(model, model_loaded()).await;
                println!("___model loaded___waiting for binding__");
                let hand = get_bone_by_bind_id(model, &BindId::RightHand);
                if hand.is_none() {
                    return;
                }
                let hand = hand.unwrap();

                let head = get_bone_by_bind_id(model, &BindId::Head);
                if head.is_none() {
                    return;
                }
                let head = head.unwrap();

                let gun = Entity::new()
                    .with_merge(make_transformable())
                    .with(
                        prefab_from_url(),
                        asset::url("assets/gun/m4a1_carbine.glb").unwrap(),
                    )
                    // y => far from body; need more tuning
                    .with(translation(), vec3(0.0, 0.2, 0.0))
                    .with(
                        rotation(),
                        Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    )
                    .with(scale(), Vec3::ONE * 0.01)
                    .with_default(local_to_parent())
                    .with_default(reset_scale())
                    .spawn();
                let cam = Entity::new()
                    .with_merge(make_perspective_infinite_reverse_camera())
                    .with(aspect_ratio_from_window(), EntityId::resources())
                    .with_default(main_scene())
                    .with(user_id(), uid)
                    // .with(parent(), id)
                    // this is FPS
                    // .with(translation(), vec3(0.0, 0.2, 2.0))
                    // third person
                    // .with(translation(), vec3(-100.0, 0.0, 0.0))
                    // .with_default(local_to_parent())
                    // .with_default(local_to_world())
                    .with(rotation(), Quat::from_rotation_x(std::f32::consts::PI))
                    .with_default(local_to_parent())
                    .spawn();

                // entity::mutate_component(id, children(), |v| {
                //     v.push(cam);
                // });

                entity::add_component(id, components::player_cam_ref(), cam);

                entity::add_child(hand, gun);
                entity::add_child(head, cam);
            });
        }
    });
}
