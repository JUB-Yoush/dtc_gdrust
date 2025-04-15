use godot::classes::AnimatedSprite2D;
use godot::classes::Area2D;
use godot::classes::IArea2D;
use godot::classes::PhysicsBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    speed: f32,
    screen_size: Vector2,
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    pub fn hit();

    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {}
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world NEW2");

        Self {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();
        self.signals()
            .hit()
            .connect_self(Self::on_player_body_entered);
    }

    fn process(&mut self, delta: f64) {
        let mut animated_sprite: Gd<AnimatedSprite2D> = self.base().get_node_as("AnimatedSprite2D");
        let mut velocity = Vector2::new(0.0, 0.0);
        let input = Input::singleton();
        let dir = input.get_vector("left", "right", "up", "down");

        velocity = Vector2::new(
            dir.x * self.speed * (delta as f32),
            dir.y * self.speed * (delta as f32),
        );

        if velocity != Vector2::ZERO {
            animated_sprite.play();
        } else {
            animated_sprite.stop();
        }

        if velocity.x != 0.0 {
            animated_sprite.set_animation("walk");
            animated_sprite.set_flip_v(false);
            animated_sprite.set_flip_h(velocity.x < 0.0);
        } else if velocity.y != 0.0 {
            animated_sprite.set_animation("up");
            animated_sprite.set_flip_v(velocity.y > 0.0);
        }

        let position = self.base().get_global_position() + velocity;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}
