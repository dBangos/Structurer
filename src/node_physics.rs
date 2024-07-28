use crate::egui::Vec2;
use crate::Structurer;
impl Structurer {
    pub fn node_physics(&mut self) {
        // Physics code based on https://editor.p5js.org/JeromePaddick/sketches/bjA_UOPip
        let edge_length: f32 = 1000.0;
        let divider: f32 = 7.0;
        let gravity_constant: f32 = 0.1;
        let force_constant: f32 = 1000.0;
        for title_1 in self.title_order.clone() {
            //Gravity
            self.titles.get_mut(&title_1).unwrap().node_force =
                self.titles[&title_1].node_position.to_vec2() * (-1.0) * gravity_constant;

            //println!("Gravity: {}", self.titles[&title_1].node_force);
            //Repulsive forces
            for title_2 in self.title_order.clone() {
                if self.titles[&title_1].id == self.titles[&title_2].id {
                    continue;
                } else {
                    let dir =
                        self.titles[&title_2].node_position - self.titles[&title_1].node_position;
                    let mut repulsive_force: Vec2 = Vec2::new(10.0, 10.0);
                    if dir.length() != 0.0 {
                        repulsive_force = dir / (dir.length() * dir.length()) * force_constant;
                    }
                    self.titles.get_mut(&title_1).unwrap().node_force -= repulsive_force;
                    self.titles.get_mut(&title_2).unwrap().node_force += repulsive_force;
                    //println!("Repulsive: {}", repulsive_force);
                }
            }
        }

        //Loop pulling in links
        for (title_1, title_2) in self.linked_pairs.clone() {
            let dir = self.titles[&title_1].node_position - self.titles[&title_2].node_position;

            self.titles.get_mut(&title_1).unwrap().node_force -= dir / divider;
            self.titles.get_mut(&title_2).unwrap().node_force += dir / divider;
        }

        for title_1 in self.title_order.clone() {
            let velocity = self.titles[&title_1].node_force / divider;
            //println!("Velocity: {}", velocity);
            self.titles.get_mut(&title_1).unwrap().node_position += velocity;
        }
    }
}
