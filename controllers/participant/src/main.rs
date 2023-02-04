use std::time::Duration;

use webots::prelude::*;

struct Participant {
    arm_up: bool,
    arm_motor: Motor,
}

impl Robot for Participant {
    fn time_step(&self) -> std::time::Duration {
        // Most robots will want this to something more like 16 milliseconds.
        Duration::from_secs(1)
    }

    fn init() -> Self {
        let arm_motor = Motor::new("RShoulderPitch");
        arm_motor.set_velocity(arm_motor.max_velocity());
        Participant {
            arm_motor,
            arm_up: false,
        }
    }

    fn step(&mut self) {
        if !self.arm_up {
            self.arm_motor.set_position(self.arm_motor.max_position());
        } else {
            self.arm_motor.set_position(self.arm_motor.min_position());
        }
        self.arm_up = !self.arm_up;
    }
}

fn main() {
    webots::run::<Participant>();
}
