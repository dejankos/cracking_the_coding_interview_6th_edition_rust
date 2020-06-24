// Design a parking lot using object-oriented principles

use itertools::Itertools;

#[derive(Debug)]
struct Parking {
    levels: Vec<ParkingLevel>,
}

#[derive(Debug, Clone)]
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
            VehicleType::Bus => 3,
        }
    }
}

impl Parking {
    fn new(levels: Vec<ParkingLevel>) -> Self {
        Parking { levels }
    }

    fn park_at_level(&mut self, lvl: isize, vehicle: VehicleType) -> bool {
        if let Some(l) = self.find_lvl(lvl) {
            let spot = self.find_spot_in_row(l, vehicle.size());
            if spot.0 {
                self.park(lvl, spot.1, spot.2, spot.3);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn park(&mut self, lvl: isize, row_idx: usize, start: usize, end: usize) {
        let m_lvl = self.levels.iter_mut().find(|l| l.id == lvl).unwrap();

        for i in start..end {
            m_lvl.rows[row_idx][i] = SpotStatus::Taken;
        }
    }

    fn find_lvl(&self, lvl: isize) -> Option<ParkingLevel> {
        self.levels.iter().find(|l| l.id == lvl).map(|l| l).cloned()
    }

    fn find_spot_in_row(&self, lvl: ParkingLevel, v_size: usize) -> (bool, usize, usize, usize) {
        for (row_idx, row) in lvl.rows.iter().enumerate() {
            for (i, v) in row.iter().enumerate() {
                let v_end = i + v_size;
                if *v == SpotStatus::Free && row.len() >= v_end {
                    if self.is_spot_free(&row[i..v_end]) {
                        return (true, row_idx, i, v_end);
                    }
                }
            }
        }
        (false, 0, 0, 0)
    }

    fn is_spot_free(&self, spots: &[SpotStatus]) -> bool {
        spots.iter().map(|s| *s == SpotStatus::Free).all_equal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_park_vehicles() {
        let lvl1 = ParkingLevel {
            rows: vec![vec![SpotStatus::Free; 5], vec![SpotStatus::Free; 3]],
            id: -1,
        };

        let lvl0 = ParkingLevel {
            rows: vec![vec![SpotStatus::Free; 5], vec![SpotStatus::Free; 2]],
            id: 0,
        };

        let mut parking = Parking {
            levels: vec![lvl0, lvl1],
        };

        let found = parking.park_at_level(-1, VehicleType::Bus);
        assert!(found);
        let found = parking.park_at_level(-1, VehicleType::Van);
        assert!(found);
        let found = parking.park_at_level(-1, VehicleType::Car);
        assert!(found);
        assert!(found);
        let found = parking.park_at_level(-1, VehicleType::Van);
        assert!(found);

        assert_eq!(
            vec![vec![SpotStatus::Taken; 5], vec![SpotStatus::Taken; 3], ],
            parking.find_lvl(-1).unwrap().rows
        );

        assert_eq!(
            vec![vec![SpotStatus::Free; 5], vec![SpotStatus::Free; 2], ],
            parking.find_lvl(0).unwrap().rows
        );
    }
}
