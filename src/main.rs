use rapier3d::na::Isometry3;
use rapier3d::prelude::*;
use rapier_testbed3d::{PhysicsState, Testbed, TestbedApp, TestbedGraphics};
use rapier_testbed3d::harness::RunState;
use rapier_testbed3d::physics::PhysicsEvents;


fn init(testbed: &mut Testbed) {
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();
    
    let kinematic = {
        let rigid_body = RigidBodyBuilder::new_kinematic_position_based()
            .translation(vector![0.0, 0.0, 0.0])
            .build();
        let handle = bodies.insert(rigid_body);
        let collider = ColliderBuilder::ball(1.0).build();
        colliders.insert_with_parent(collider, handle, &mut bodies);
        
        handle
    };
    
    let dynamic = {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0.0, -5.0, 0.0])
            .build();
        let handle = bodies.insert(rigid_body);
        let collider = ColliderBuilder::ball(1.0).build();
        colliders.insert_with_parent(collider, handle, &mut bodies);
    
        handle
    };
    
    let joint = FixedJoint::new().local_anchor2(point!(0.0, 5.0, 0.0));
    impulse_joints.insert(kinematic, dynamic, joint);
    
    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.look_at(point![20.0, 4.0, 8.0], point![0.0, 0.0, 0.0]);
    
    testbed.add_callback(move |_: Option<&mut TestbedGraphics>, physics: &mut PhysicsState, _: &PhysicsEvents, run_state: &RunState| {
        if run_state.time > 3.0 {
            if let Some(kinematic_rb) = physics.bodies.get_mut(kinematic) {
                kinematic_rb.set_position(Isometry3::translation(0.0, 0.0, (run_state.time - 3.0) * 3.0), true);
            }
        }
    });
}

fn main() {
    let testbed = TestbedApp::from_builders(0, vec![("Issue", init)]);
    testbed.run()
}
