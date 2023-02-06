use std::time::Duration;
use webots::prelude::*;

const MOVE_DELAY: Duration = Duration::from_millis(800);

struct Participant {
    move_timer: Duration,
    arm_up: bool,
    arm_motor: Motor,
}

impl Robot for Participant {
    fn init() -> Self {
        let arm_motor = Motor::new("RShoulderPitch");
        arm_motor.set_velocity(arm_motor.max_velocity());
        Participant {
            move_timer: Duration::ZERO,
            arm_motor,
            arm_up: false,
        }
    }

    fn step(&mut self, time: StepTime) {
        self.move_timer += time.delta;
        if self.move_timer > MOVE_DELAY {
            self.move_timer = Duration::ZERO;

            if !self.arm_up {
                self.arm_motor.set_position(self.arm_motor.max_position());
            } else {
                self.arm_motor.set_position(self.arm_motor.min_position());
            }
            self.arm_up = !self.arm_up;
        }
    }
}

fn main() {
    webots::run_robot::<Participant>();
}
