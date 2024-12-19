use macroquad::prelude::*;
use nalgebra::Vector2;
use rapier2d::prelude::*;

const BALL_SIZE: f32 = 0.2;
const BALL_RESTITUTION: f32 = 1.7;
const GROUND_SIZE: Vec2 = vec2(0.5, 0.1);

struct Game {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
    camera: Camera2D,
}

struct Ball {
    size: f32,
    collider: Collider,
    rigid_body: RigidBody,
    rigid_body_handle: Option<RigidBodyHandle>,
}

struct Ground {
    size: Vec2,
    collider: Collider,
    collider_handle: Option<ColliderHandle>,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game {
        rigid_body_set: RigidBodySet::new(),
        collider_set: ColliderSet::new(),
        gravity: vector![0.0, 9.81],
        integration_parameters: IntegrationParameters::default(),
        physics_pipeline: PhysicsPipeline::new(),
        island_manager: IslandManager::new(),
        broad_phase: DefaultBroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        impulse_joint_set: ImpulseJointSet::new(),
        multibody_joint_set: MultibodyJointSet::new(),
        ccd_solver: CCDSolver::new(),
        query_pipeline: QueryPipeline::new(),
        physics_hooks: (),
        event_handler: (),
        camera: Camera2D {
            zoom: vec2(0.5, 0.5 * screen_width() / screen_height()),
            target: vec2(1.0, 1.0),
            ..Default::default()
        }
    };

    let mut ball = Ball {
        size: BALL_SIZE,
        collider: ColliderBuilder::ball(BALL_SIZE).restitution(BALL_RESTITUTION).build(),
        rigid_body: RigidBodyBuilder::dynamic()
            .translation(vector![1.0, 0.1])
            .build(),
        rigid_body_handle: None,
    };

    let mut ground = Ground {
        size: GROUND_SIZE,
        collider: ColliderBuilder::cuboid(GROUND_SIZE.x, GROUND_SIZE.y)
            .translation(vector![1.0, 2.0])
            .build(),
        collider_handle: None,
    };

    // create the ground
    ground.collider_handle = Some(game.collider_set.insert(ground.collider));

    // create another ground
    let collider = ColliderBuilder::cuboid(100.0, 20.0)
        .translation(vector![400.0, 400.0])
        .build();
    let ground_handle_2 = game.collider_set.insert(collider);

    ball.rigid_body_handle = Some(game.rigid_body_set.insert(ball.rigid_body));
    game.collider_set.insert_with_parent(
        ball.collider, 
        ball.rigid_body_handle.expect("Empty RigidBodyHandle"), 
        &mut game.rigid_body_set
    );

    // create another bouncing ball
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![420.0, 100.0])
        .build();
    let collider = ColliderBuilder::ball(50.0).restitution(1.0).build();
    let ball_body_handle_2 = game.rigid_body_set.insert(rigid_body);
    game.collider_set.insert_with_parent(collider, ball_body_handle_2, &mut game.rigid_body_set);

    loop {
        game.physics_pipeline.step(
            &game.gravity,
            &game.integration_parameters,
            &mut game.island_manager,
            &mut game.broad_phase,
            &mut game.narrow_phase,
            &mut game.rigid_body_set,
            &mut game.collider_set,
            &mut game.impulse_joint_set,
            &mut game.multibody_joint_set,
            &mut game.ccd_solver,
            Some(&mut game.query_pipeline),
            &game.physics_hooks,
            &game.event_handler,
        );

        ball.rigid_body = game.rigid_body_set[ball.rigid_body_handle.expect("Empty RigidBodyHandle")].clone();
        let ball_body_2 = &game.rigid_body_set[ball_body_handle_2];
        ground.collider = game.collider_set[ground.collider_handle.expect("Empty ColliderBodyHandle")].clone();
        let ground_2 = &game.collider_set[ground_handle_2];

        clear_background(GRAY);

        // Render some primitives in camera space
        set_camera(&game.camera);

        if is_key_down(KeyCode::Up) {
            game.camera.target.y -= 0.05;  
        }

        if is_key_down(KeyCode::Down) {
            draw_text("DOWN ^", 100.0, 50.0, 100.0, RED);
            game.camera.target.y += 0.05;  
        }

        if is_key_down(KeyCode::Right) {
            game.camera.target.x += 0.05;  
        }

        if is_key_down(KeyCode::Left) {
            game.camera.target.x -= 0.05;  
        }

        draw_circle(
            ball.rigid_body.translation().x, 
            ball.rigid_body.translation().y, 
            ball.size, 
            RED
        );

        draw_circle(
            ball_body_2.translation().x, 
            ball_body_2.translation().y, 
            50.0, 
            ORANGE
        );

        draw_rectangle(
            ground.collider.translation().x - ground.size.x, 
            ground.collider.translation().y - ground.size.y, 
            ground.size.x * 2.0, 
            ground.size.y * 2.0, 
            GREEN
        );

        draw_rectangle(
            ground_2.translation().x - 100.0, 
            ground_2.translation().y - 20.0, 
            100.0 * 2.0, 
            20.0 * 2.0, 
            BLUE
        );

        set_default_camera();
        draw_text("HELLO", 30.0, 200.0, 30.0, BLACK);

        next_frame().await;
    }
}
