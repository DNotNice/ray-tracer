

#[derive(Clone, Copy, Default)]
pub struct Vec3{
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Vec3{
    pub fn new(x :f64 , y : f64 , z : f64) -> Vec3 {
        Vec3 { x:x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x:0.0, y: 0.0, z: 0.0 }
    }

    pub fn dot(A: Vec3 , B:Vec3) -> f64 {
        A.x * B.x + A.y * B.y + A.z * B.z
    }

    pub fn length(&self) ->f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn unit(&self) ->Vec3 {
        let length = self.length();
        Vec3::new(self.x/length , self.y/length , self.z /length)
    }

}