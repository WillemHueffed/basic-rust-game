use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        // Should use rust irrefutable patterns here
        for (pos, val) in (&mut data.0, &data.1).join() {
            match val.direction {
                Left => {
                    pos.0 = pos.0.offset(-val.speed, 0);
                }
                Right => {
                    pos.0 = pos.0.offset(val.speed, 0);
                }
                Up => {
                    pos.0 = pos.0.offset(0, -val.speed);
                }
                Down => {
                    pos.0 = pos.0.offset(0, val.speed);
                }
            }
        }
    }
}
