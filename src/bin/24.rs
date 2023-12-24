use z3::{ast::*, SatResult};

advent_of_code::solution!(24);

type Point = (i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    point: Point,
    velocity: Point,
}

impl Hailstone {
    fn from(s: &str) -> Self {
        let mut seg = s.split(" @ ");
        let point = seg
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let point = (point[0], point[1], point[2]);
        let velocity = seg
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let velocity = (velocity[0], velocity[1], velocity[2]);
        Self { point, velocity }
    }

    fn xy_pointslope(&self) -> (f64, f64) {
        let slope = self.velocity.1 as f64 / self.velocity.0 as f64;
        let intercept = slope * -1_f64 * self.point.0 as f64 + self.point.1 as f64;
        (slope, intercept)
    }

    fn xy_intercept(&self, other: &Self) -> (f64, f64) {
        let self_psf = self.xy_pointslope();
        let other_psf = other.xy_pointslope();
        let x = (self_psf.1 - other_psf.1) / (other_psf.0 - self_psf.0);
        let y = (self_psf.1 * other_psf.0 - other_psf.1 * self_psf.0) / (other_psf.0 - self_psf.0);
        (x, y)
    }

    fn xy_intercept_is_valid(&self, intercept: (f64, f64), min: f64, max: f64) -> bool {
        if intercept.0 < min || intercept.0 > max {
            return false;
        }
        if intercept.1 < min || intercept.1 > max {
            return false;
        }

        let x = self.point.0 as f64;
        if self.velocity.0 < 0 {
            if x < intercept.0 {
                return false;
            }
        } else if x > intercept.0 {
            return false;
        }

        let y = self.point.1 as f64;
        if self.velocity.1 < 0 {
            if y < intercept.1 {
                return false;
            }
        } else if y > intercept.1 {
            return false;
        }

        true
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input.lines().map(Hailstone::from).collect::<Vec<_>>();
    let (min, max) = if stones.len() == 5 {
        (7.0, 27.0)
    } else {
        (200000000000000.0, 400000000000000.0)
    };
    let intersections = stones.iter().enumerate().fold(0, |mut acc, (i, hs)| {
        (i + 1..stones.len()).for_each(|i2| {
            let hs2 = stones[i2];
            let intercept = hs.xy_intercept(&hs2);
            if hs.xy_intercept_is_valid(intercept, min, max)
                && hs2.xy_intercept_is_valid(intercept, min, max)
            {
                acc += 1;
            }
        });
        acc
    });
    Some(intersections)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input.lines().map(Hailstone::from).collect::<Vec<_>>();

    let z3_conf = z3::Config::new();
    let ctx = z3::Context::new(&z3_conf);
    let solver = z3::Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    stones.iter().enumerate().for_each(|(i, hs)| {
        let t_intercept = Int::new_const(&ctx, format!("t_{}", i));

        solver.assert(&(&x + &vx * &t_intercept)._eq(&(hs.point.0 + hs.velocity.0 * &t_intercept)));
        solver.assert(&(&y + &vy * &t_intercept)._eq(&(hs.point.1 + hs.velocity.1 * &t_intercept)));
        solver.assert(&(&z + &vz * &t_intercept)._eq(&(hs.point.2 + hs.velocity.2 * &t_intercept)));
    });

    let res = solver.check();
    assert_eq!(res, SatResult::Sat);
    let res = solver
        .get_model()
        .expect("Expected model to exist")
        .eval(&(&x + &y + &z), true)
        .expect("Expected eval to work");

    res.as_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
