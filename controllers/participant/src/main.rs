use webots::prelude::*;

const MOVE_DELAY: u32 = 100;

struct Participant {
    loop_counter: u32,
    arm_up: bool,
    arm_motor: Motor,
}

impl Robot for Participant {
    fn init() -> Self {
        let arm_motor = Motor::new("RShoulderPitch");
        arm_motor.set_velocity(arm_motor.max_velocity());
        Participant {
            arm_motor,
            loop_counter: 0,
            arm_up: false,
        }
    }

    fn step(&mut self) {
        self.loop_counter += 1;

        if self.loop_counter > MOVE_DELAY {
            self.loop_counter = 0;

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
    webots::run::<Participant>();
}
