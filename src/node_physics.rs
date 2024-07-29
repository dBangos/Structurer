use crate::egui::Vec2;
use crate::Structurer;
use rand::Rng;
impl Structurer {
    pub fn node_physics(&mut self) {
        // Physics code based on https://editor.p5js.org/JeromePaddick/sketches/bjA_UOPip
        let divider: f32 = 7.0;
        let gravity_constant: f32 = 0.1;
        let force_constant: f32 = 3000.0;
        for (index, title_1) in self.title_order.clone().into_iter().enumerate() {
            //Repulsive forces
            for inner_index in index..self.title_order.len() {
                if self.titles[&title_1].id == self.titles[&self.title_order[inner_index]].id {
                    continue;
                } else {
                    let dir = self.titles[&self.title_order[inner_index]].node_force
                        - self.titles[&title_1].node_force;
                    let repulsive_force: Vec2;
                    if dir.length() != 0.0 {
                        repulsive_force = dir / (dir.length() * dir.length()) * force_constant;
                    } else {
                        let random_val_1 = rand::thread_rng().gen_range(-10.0..10.0);
                        let random_val_2 = rand::thread_rng().gen_range(-10.0..10.0);
                        repulsive_force = Vec2::new(random_val_1, random_val_2);
                    }
                    self.titles.get_mut(&title_1).unwrap().node_force -= repulsive_force / divider;
                    self.titles
                        .get_mut(&self.title_order[inner_index])
                        .unwrap()
                        .node_force += repulsive_force / divider;
                }
            }
            //Gravity
            let temp = self.titles[&title_1].node_force * (-1.0) * gravity_constant / divider;
            self.titles.get_mut(&title_1).unwrap().node_force += temp;
        }

        //Loop pulling in links
        for (title_1, title_2) in self.linked_pairs.clone() {
            let dir = self.titles[&title_1].node_force - self.titles[&title_2].node_force;

            self.titles.get_mut(&title_1).unwrap().node_force -= dir / divider / divider;
            self.titles.get_mut(&title_2).unwrap().node_force += dir / divider / divider;
        }
    }
}
