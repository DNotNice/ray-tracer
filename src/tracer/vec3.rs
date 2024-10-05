use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub} ;

pub type Colour = Vec3;

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
    pub fn write_colour(self)  {
        let i_r = (255.999 * self.x).round() as u16;
        let i_g = (255.999 * self.y).round() as u16;
        let i_b = (255.999 * self.z).round() as u16;
        print!("{} {} {}\n", i_r, i_g, i_b);
    }
    pub fn cross(mut self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

}

impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, rhs: Vec3  ) -> Vec3 {
        Vec3 { 
               x: self.x + rhs.x ,
               y: self.y + rhs.y ,
               z: self.z + rhs.z 
            }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 { 
               x: self.x * rhs ,
               y: self.y * rhs ,
               z: self.z * rhs 
            }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { 
               x: self.x * rhs.x ,
               y: self.y * rhs.y ,
               z: self.z * rhs.z 
            }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
               x: self.x / rhs ,
               y: self.y / rhs ,
               z: self.z / rhs 
            }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { 
               x: self.x - rhs.x ,
               y: self.y - rhs.y ,
               z: self.z - rhs.z 
            }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -&self.x,
            y: -&self.y,
            z: -&self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ,{} , {})" ,self.x , self.y , self.z)
    }
}

