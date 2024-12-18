use macroquad::prelude::*;
use rapier2d::prelude::*;

//const MOVEMENT_SPEED: f32 = 10.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    //let mut x = screen_width() / 2.0;
    //let mut y = screen_height() / 2.0;

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // create the ground
    let collider = ColliderBuilder::cuboid(100.0, 20.0)
        .translation(vector![100.0, 500.0])
        .build();
    let ground_handle = collider_set.insert(collider);

    // create the bouncing ball
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![100.0, 100.0])
        .build();
    let collider = ColliderBuilder::ball(30.0).restitution(1.0).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    // create other structures necessary for the simulation
    let gravity = vector![0.0, 100.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();

    loop {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        let ball_body = &rigid_body_set[ball_body_handle];
        let ground = &collider_set[ground_handle];
        //println!(
        //    "Ball altitude: x:{} y:{}", 
        //    ball_body.translation().x,
        //    ball_body.translation().y
        //);

        clear_background(GRAY);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_down(KeyCode::Up) {
            draw_text("UP ^", 100.0, 50.0, 100.0, RED
            );
            //y -= MOVEMENT_SPEED;
        }

        if is_key_down(KeyCode::Down) {
            draw_text("DOWN ^", 100.0, 50.0, 100.0, RED);
            //y += MOVEMENT_SPEED;
        }

        if is_key_down(KeyCode::Right) {
            draw_text(
                "RIGHT ->", 
                screen_width() / 2.0,
                screen_height() / 2.0, 
                100.0, 
                RED
            );

            //x += MOVEMENT_SPEED;
        }

        if is_key_down(KeyCode::Left) {
            draw_text(
                "LEFT <-", 
                screen_width() / 2.0,
                screen_height() / 2.0, 
                100.0, 
                RED
            );
            //x -= MOVEMENT_SPEED;
        }

        draw_circle(
            ball_body.translation().x, 
            ball_body.translation().y, 
            30.0, 
            RED
        );

        draw_rectangle(
            ground.translation().x - 100.0, 
            ground.translation().y - 20.0, 100.0, 20.0 * 2.0, GREEN);

        next_frame().await;
    }
}
