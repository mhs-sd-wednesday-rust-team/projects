use specs::{Component, DenseVecStorage, DispatcherBuilder, Join, World, WorldExt};

pub mod view;

#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub struct Experience {
    pub level: usize,
    pub exp_count: usize,
}

impl Experience {
    pub fn max_exp(&self) -> usize {
        100.0_f64.powf(1.0 + self.level as f64 * 0.01) as usize
    }

    pub fn exp_ratio(&self) -> f64 {
        self.exp_count as f64 / self.max_exp() as f64
    }

    pub fn up(&mut self, gain: GainExperience) {
        let mut exp_count = gain.exp_count;
        while self.exp_count + exp_count > self.max_exp() {
            exp_count -= self.max_exp();
            self.level += 1;
        }
        self.exp_count += exp_count;
    }
}

#[derive(Component, Clone, Debug)]
#[allow(dead_code)]
pub struct KillExperience {
    pub exp_count: usize,
}

impl KillExperience {
    #[allow(dead_code)]
    pub fn new(exp_count: usize) -> Self {
        Self { exp_count }
    }
}

#[derive(Component, Clone, Debug)]
pub struct GainExperience {
    pub exp_count: usize,
}

impl GainExperience {
    #[allow(dead_code)]
    pub fn new(exp_count: usize) -> Self {
        Self { exp_count }
    }
}

impl From<KillExperience> for GainExperience {
    fn from(value: KillExperience) -> Self {
        Self {
            exp_count: value.exp_count,
        }
    }
}

struct GainExperienceSystem;

impl<'a> specs::System<'a> for GainExperienceSystem {
    type SystemData = (
        specs::WriteStorage<'a, Experience>,
        specs::WriteStorage<'a, GainExperience>,
    );

    fn run(&mut self, (mut experience, mut gain_experience): Self::SystemData) {
        for (exp, gain_exp) in (&mut experience, gain_experience.drain()).join() {
            exp.up(gain_exp);
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Experience>();
    world.register::<GainExperience>();
    world.register::<KillExperience>();

    dispatcher.add(GainExperienceSystem, "gain_experience_system", &[]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use specs::Builder;

    use super::*;

    #[test]
    fn test_level_up() {
        struct TestCase {
            before: Experience,
            gain: GainExperience,
            after: Experience,
        }

        let test_cases = vec![
            TestCase {
                before: Experience {
                    level: 10,
                    exp_count: 0,
                },
                gain: GainExperience::new(0),
                after: Experience {
                    level: 10,
                    exp_count: 0,
                },
            },
            TestCase {
                before: Experience {
                    level: 0,
                    exp_count: 0,
                },
                gain: GainExperience::new(50),
                after: Experience {
                    level: 0,
                    exp_count: 50,
                },
            },
            TestCase {
                before: Experience {
                    level: 0,
                    exp_count: 0,
                },
                gain: GainExperience::new(120),
                after: Experience {
                    level: 1,
                    exp_count: 20,
                },
            },
            TestCase {
                before: Experience {
                    level: 1000,
                    exp_count: 1,
                },
                gain: GainExperience::new(1),
                after: Experience {
                    level: 1000,
                    exp_count: 2,
                },
            },
            TestCase {
                before: Experience {
                    level: 8,
                    exp_count: 10,
                },
                gain: GainExperience::new(400),
                after: Experience {
                    level: 10,
                    exp_count: 115,
                },
            },
        ];

        for test_case in test_cases {
            let mut exp = test_case.before.clone();
            exp.up(test_case.gain.clone());
            assert_eq!(
                exp, test_case.after,
                "before={:?}, gain={:?}, after={:?}",
                test_case.before, test_case.gain, test_case.after
            );
        }
    }

    #[test]
    fn test_system() {
        let mut world = World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        register(&mut dispatcher_builder, &mut world).unwrap();

        let mut dispatcher = dispatcher_builder.build();

        let e = world
            .create_entity()
            .with(Experience::default())
            .with(GainExperience::new(200))
            .build();

        let gains_count = world.read_component::<GainExperience>().count();
        assert_eq!(gains_count, 1);

        dispatcher.dispatch(&world);
        world.maintain();

        let gains_count = world.read_component::<GainExperience>().count();
        assert_eq!(gains_count, 0);

        let exps = world.read_component::<Experience>();
        let exp = exps
            .get(e)
            .expect("world contains previously created entity");

        assert_eq!(
            exp,
            &Experience {
                level: 1,
                exp_count: 100
            }
        );
    }
}
