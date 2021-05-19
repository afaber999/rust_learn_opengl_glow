extern crate nalgebra_glm as glm;

pub enum Directions {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Backward,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    position    : glm::Vec3,
    front       : glm::Vec3,
    up          : glm::Vec3,  
    right       : glm::Vec3,  
    world_up    : glm::Vec3,  
    
    yaw         : f32,
    pitch       : f32, 
    speed       : f32,
    sensitivity : f32,
    zoom        : f32,   
}

impl Camera {
    pub fn new( position: glm::Vec3 ) -> Self {

        let yaw = -90.0f32;
        let pitch = 0.0f32;
        let world_up = glm::vec3(0.0,1.0,0.0 );
        let front = Camera::calc_front(yaw, pitch);
        let right = Camera::calc_right(&front, &world_up);
        let up = Camera::calc_up(&right, &front);

        Self {
            position,
            front,
            up,  
            right,  
            world_up ,  
            yaw,
            pitch, 
            speed       : 2.5f32,
            sensitivity : 0.1f32,
            zoom        : 45.0f32, 
        }
    }


    pub fn get_zoom(&self) -> f32 {self.zoom}
    pub fn get_position(&self) -> &glm::Vec3 {&self.position}
    pub fn get_front(&self) -> &glm::Vec3 {&self.front}

    pub fn get_view_matrix(&self) -> glm::Mat4 
    {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    
    pub fn key_interact(&mut self, direction: Directions ) {

        let velocity = self.speed * 0.16f32;

        match direction {
            Directions::Forward => {
                self.position += self.front * velocity;
            }
            Directions::Left => {
                self.position -= self.right * velocity;
            }
            Directions::Right => {
                self.position += self.right * velocity;
            }
            Directions::Up => {
                self.position += self.up * velocity;
            }
            Directions::Down => {
                self.position -= self.up * velocity;
            }
            Directions::Backward => {
                self.position -= self.front * velocity;
            }
        }
    }

    pub fn mouse_interact(&mut self,  dx: f32, dy : f32 ) {

        self.yaw =  self.yaw  + dx * self.sensitivity;
        self.pitch = (self.pitch - dy * self.sensitivity).max(-89.0).min(89.0);
    
        self.front = Camera::calc_front(self.yaw, self.pitch);
        self.right = Camera::calc_right(&self.front, &self.world_up);
        self.up = Camera::calc_up(&self.right, &self.front);
    }

    pub fn scroll_wheel_interact(&mut self,  delta: f32 ) {
        let new_zoom = (self.zoom + delta).max(1.0).min(55.0);
        //println!("Scroll {} + delta {} => {}", self.zoom, delta, new_zoom );
        self.zoom = new_zoom;
    }

    fn calc_front(yaw:f32, pitch: f32) -> glm::Vec3 {
        let ya = yaw.to_radians();
        let pa = pitch.to_radians();

        glm::vec3(
            ya.cos() * pa.cos(),
            pa.sin(),
            ya.sin() * pa.cos() )
            .normalize()
    }

    fn calc_right(front :&glm::Vec3, world_up: &glm::Vec3) -> glm::Vec3 {
        glm::cross(front, world_up).normalize()
    }

    fn calc_up(right :&glm::Vec3, front: &glm::Vec3) -> glm::Vec3 {
        glm::cross(right, front).normalize()
    }
}
