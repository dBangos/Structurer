use crate::egui::Vec2;
use crate::Structurer;
use rand::Rng;
impl Structurer {
    pub fn node_physics(&mut self) {
        // Physics code based on https://editor.p5js.org/JeromePaddick/sketches/bjA_UOPip
        let divider: f32 = 7.0;
        let gravity_constant: f32 = 0.1;
        let force_constant: f32 = 3000.0;
        //Loop pulling in links
        for (title_index_1, title_index_2) in self.linked_pairs.clone() {
            let dir = self.titles[title_index_1].node_physics_position
                - self.titles[title_index_2].node_physics_position;

            if !self.titles[title_index_1].node_currnetly_clicked {
                self.titles[title_index_1].node_physics_position -= dir / divider / divider / 2.0;
            }

            if !self.titles[title_index_2].node_currnetly_clicked {
                self.titles[title_index_2].node_physics_position += dir / divider / divider / 2.0;
            }
        }
        for index in 0..self.titles.len() {
            //Repulsive forces
            for inner_index in index..self.titles.len() {
                if self.titles[index].id == self.titles[inner_index].id {
                    continue;
                } else {
                    let dir = self.titles[inner_index].node_physics_position
                        - self.titles[index].node_physics_position;
                    let repulsive_force: Vec2;
                    if dir.length() != 0.0 {
                        repulsive_force = dir / (dir.length() * dir.length()) * force_constant;
                    } else {
                        let random_val_1 = rand::thread_rng().gen_range(-10.0..10.0);
                        let random_val_2 = rand::thread_rng().gen_range(-10.0..10.0);
                        repulsive_force = Vec2::new(random_val_1, random_val_2);
                    }
                    if !self.titles[index].node_currnetly_clicked {
                        self.titles[index].node_physics_position -= repulsive_force / divider;
                    }
                    if !self.titles[inner_index].node_currnetly_clicked {
                        self.titles[inner_index].node_physics_position += repulsive_force / divider;
                    }
                }
            }
            //Gravity
            if !self.titles[index].node_currnetly_clicked {
                let temp =
                    self.titles[index].node_physics_position * (-1.0) * gravity_constant / divider;
                self.titles[index].node_physics_position += temp;
            } else if !self.stop_clicked_nodes {
                //This is the last check of the node.
                //Leaving it true means it can't be affected by physics
                self.titles[index].node_currnetly_clicked = false;
            }
        }
    }
}
