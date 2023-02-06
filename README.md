# Humanoid Robot Wrestling Controller Example

[![webots.cloud - Competition](https://img.shields.io/badge/webots.cloud-Competition-007ACC)][1]

## Ferris Rust controller

Minimalist Rust controller example for the [Humanoid Robot Wrestling Competition](https://github.com/cyberbotics/wrestling).

Demonstrates how to move the arm motor.

Uses the work-in-progress [webots-rust] bindings. Full access to the Webots `libcontroller` C API is
provided through the `webots::sys` module, but ergonomic bindings are still work-in-progress and will be added as needed,
with priority on the sensors and functionality offered by the NAO model used in the tournament.

[webots-rust]: https://github.com/katharostech/webots-rust

```rust
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
```

Note that the controllers [Dockerfile](controllers/Dockerfile) was also modified to include the installation of Rust.

[1]: https://webots.cloud/run?version=R2022b&url=https%3A%2F%2Fgithub.com%2Fcyberbotics%2Fwrestling%2Fblob%2Fcompetition%2Fworlds%2Fwrestling.wbt&type=competition "Leaderboard"
