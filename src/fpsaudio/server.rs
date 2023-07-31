use ambient_api::prelude::*;

#[main]
pub fn main() {
    messages::Shoot::subscribe(move |_, msg| {
        messages::FireSound::new(msg.source).send_client_broadcast_unreliable();
    });

    messages::FootOnGround::subscribe(move |_, msg| {
        messages::WalkSound::new(msg.source).send_client_broadcast_unreliable();
    });

    messages::Explosion::subscribe(move |_, msg| {
        println!("explosion msg got from server");
        messages::Explosion::new(msg.pos).send_client_broadcast_unreliable();
    });
}
