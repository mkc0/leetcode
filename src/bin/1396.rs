// 40 ms Rust
// 100+ ms C++
// 90ms Java

use std::collections::HashMap;

struct UndergroundSystem {
    average_map: HashMap<(String, String), (i32, i32)>,
    checked_in_map: HashMap<i32, (String, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            average_map: HashMap::new(),
            checked_in_map: HashMap::new(),
        }
    }
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checked_in_map.insert(id, (station_name, t));
    }
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let check_in = self.checked_in_map.get(&id).unwrap();
        let used_t = t - check_in.1;
        let station_pair = (check_in.0.clone(), station_name);
        let new_pair;
        if let Some(pair) = self.average_map.get(&station_pair) {
            new_pair = (pair.0 + used_t, pair.1 + 1);
        } else {
            new_pair = (used_t, 1);
        }
        self.average_map.insert(station_pair, new_pair);
        self.checked_in_map.remove(&id);
    }
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let a = self.average_map.get(&(start_station, end_station)).unwrap();
        a.0 as f64 / a.1 as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

fn main() {
    let mut obj = UndergroundSystem::new();
    obj.check_in(10, "A".to_string(), 2);
    obj.check_out(10, "B".to_string(), 7);
    let ret_3: f64 = obj.get_average_time("A".to_string(), "B".to_string());
    println!("{}", ret_3);
}
