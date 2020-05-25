// Design a parking lot using object-oriented principles

use itertools::Itertools;

#[derive(Debug)]
struct Parking {
    levels: Vec<ParkingLevel>
}

#[derive(Debug)]
struct ParkingLevel {
    rows: Vec<Vec<SpotStatus>>,
    id: isize,
}

enum VehicleType {
    Car,
    Bus,
    Van,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum SpotStatus {
    Free,
    Taken,
}

impl VehicleType {
    fn size(&self) -> usize {
        match *self {
            VehicleType::Car => 1,
            VehicleType::Van => 2,
            VehicleType::Bus => 3
        }
    }
}

impl Parking {
    fn new(levels: Vec<ParkingLevel>) -> Self {
        Parking { levels }
    }

    fn park_at_level(&mut self, lvl: isize, vehicle: VehicleType) -> bool {
        let level = self.levels.iter_mut().find(|l| l.id == lvl);
        let mut found = false;
        if let Some(l) = level {
            l.rows.iter_mut()
                .for_each(|r| {
                    let len = r.len();
                    let (mut start, mut end) = (0, 0);

                    if !found {
                        for (i, v) in r.iter().enumerate() {
                            let v_end = i + vehicle.size() ;
                            if *v == SpotStatus::Free && len >= v_end {
                                let slice = &r[i..v_end];
                                let free_spot = slice.iter().map(|s| if *s == SpotStatus::Free { true } else { false }).all_equal();

                                if free_spot {
                                    start = i;
                                    end = v_end;
                                    found = true;
                                }
                            }
                        }

                        for spot in start..end {
                            r[spot] = SpotStatus::Taken;
                        }
                    }

                }
                );
        } else {
            found = false
        }

        found
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_park_vehicles() {
        let lvl1 = ParkingLevel {
            rows: vec![
                vec![SpotStatus::Free; 5],
                vec![SpotStatus::Free; 3],
            ],
            id: -1,
        };

        let lvl0 = ParkingLevel {
            rows: vec![
                vec![SpotStatus::Free; 5],
                vec![SpotStatus::Free; 2],
            ],
            id: 0,
        };

        let mut parking = Parking {
            levels: vec![lvl0, lvl1]
        };

        println!("{:?}", parking);
        let found = parking.park_at_level(-1, VehicleType::Car);
        let found = parking.park_at_level(-1, VehicleType::Car);
        let found = parking.park_at_level(-1, VehicleType::Car);
        let found = parking.park_at_level(-1, VehicleType::Car);
        let found = parking.park_at_level(-1, VehicleType::Van);
        let found = parking.park_at_level(-1, VehicleType::Car);
        let found = parking.park_at_level(-1, VehicleType::Car);
        println!("found = {}", found);
        println!("{:?}", parking);

    }
}