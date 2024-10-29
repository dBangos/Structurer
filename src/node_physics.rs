use crate::egui::Vec2;
use crate::Structurer;
use rand::Rng;
impl Structurer {
    pub fn node_physics(&mut self) {
        // Physics code based on https://editor.p5js.org/JeromePaddick/sketches/bjA_UOPip
        //Loop pulling in links
        for (title_index_1, title_index_2) in self.linked_pairs.clone() {
            let dir = self.titles[title_index_1].node_physics_position
                - self.titles[title_index_2].node_physics_position;

            if !self.titles[title_index_1].node_currnetly_clicked {
                self.titles[title_index_1].node_physics_position -=
                    dir / 98.0 * self.node_view_controls.link_pull;
            }

            if !self.titles[title_index_2].node_currnetly_clicked {
                self.titles[title_index_2].node_physics_position +=
                    dir / 98.0 * self.node_view_controls.link_pull;
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
                        repulsive_force = dir / (dir.length() * dir.length())
                            * 3000.0
                            * self.node_view_controls.node_repulsion;
                    } else {
                        let random_val_1 = rand::thread_rng().gen_range(-10.0..10.0);
                        let random_val_2 = rand::thread_rng().gen_range(-10.0..10.0);
                        repulsive_force = Vec2::new(random_val_1, random_val_2);
                    }
                    if !self.titles[index].node_currnetly_clicked {
                        self.titles[index].node_physics_position -= repulsive_force / 7.0;
                    }
                    if !self.titles[inner_index].node_currnetly_clicked {
                        self.titles[inner_index].node_physics_position += repulsive_force / 7.0;
                    }
                }
            }
            //Gravity
            if !self.titles[index].node_currnetly_clicked {
                let temp = self.titles[index].node_physics_position * (-0.1) / 7.0
                    * self.node_view_controls.gravity;
                self.titles[index].node_physics_position += temp;
            } else if !self.node_view_controls.stop_clicked_nodes {
                //This is the last check of the node.
                //Leaving it true means it can't be affected by physics
                self.titles[index].node_currnetly_clicked = false;
            }
        }
    }
}
