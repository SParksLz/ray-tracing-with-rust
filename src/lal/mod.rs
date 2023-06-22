use std::ops;
#[derive(Debug, Clone, Copy)]
pub struct vec3
{
    v0 : f64,
    v1 : f64,
    v2 : f64,
}

impl vec3{
    pub fn create(x:f64, y:f64, z:f64) -> Self{
        Self { v0:x, v1: y, v2: z }
    }

    pub fn create_with_scalar(v:f64) -> Self{
        Self{ v0 : v, v1 : v, v2 : v}
    }

    pub fn unit_vector(vec : Self) -> Self{
        vec / vec.length()
    }
}

impl vec3{
    //xyz
    pub fn x(&self) -> f64{
        self.v0
    }
    pub fn y(&self) -> f64{
        self.v1
    }
    pub fn z(&self) -> f64{
        self.v2
    }
    //rgb
    pub fn r(&self) -> f64{
        self.v0
    }
    pub fn g(&self) -> f64{
        self.v1
    }
    pub fn b(&self) -> f64{
        self.v2
    }

}


impl ops::Add for vec3{
    type Output = Self;
    fn add(self, other : Self) -> Self::Output{
        Self{
            v0 : self.v0 + other.v0,
            v1 : self.v1 + other.v1,
            v2 : self.v2 + other.v2,
        }
    }
}
impl ops::Neg for vec3{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self{
            v0 : -self.v0,
            v1 : -self.v1,
            v2 : -self.v2,
        }
    }
}
impl ops::Sub for vec3{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self{
            v0 : self.v0 - other.v0,
            v1 : self.v1 - other.v1,
            v2 : self.v2 - other.v2,
        }
    }
}
// / * with scalar

impl ops::Mul<f64> for vec3{
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self{
            v0 : self.v0 * other,
            v1 : self.v1 * other,
            v2 : self.v2 * other,
        }
    }
}

impl ops::Div<f64> for vec3{
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self{
            v0 : self.v0 / other,
            v1 : self.v1 / other,
            v2 : self.v2 / other,
        }
    }
}



// / * are for colors
impl ops::Mul for vec3{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self{
            v0 : self.v0 * other.v0,
            v1 : self.v1 * other.v1,
            v2 : self.v2 * other.v2,
        }
    }
}

impl ops::Div for vec3{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self{
            v0 : self.v0 / other.v0,
            v1 : self.v1 / other.v1,
            v2 : self.v2 / other.v2,
        }
    }

}




impl vec3{
    pub fn length(&self) -> f64{
        (self.v0 * self.v0 + self.v1 * self.v1 + self.v2 * self.v2).sqrt()
    }
    pub fn squared_length(&self) -> f64{
        self.v0 * self.v0 + self.v1 * self.v1 + self.v2 * self.v2
    }
    pub fn dot(&self, other : Self)->f64{
        self.v0 * other.v0 + self.v1 * other.v1 + self.v2 * other.v2
    }
    pub fn cross(&self, other : Self) -> Self{
        Self{
            v0 : self.v1 * other.v2 - self.v2 * other.v1,
            v1 : -(self.v0 * other.v2 - self.v2 * other.v0),
            v2 : (self.v0 * other.v1 - self.v1 * other.v0)
        }
    }
    // pub fn make_unit(&self) -> Self{
    //     self / Self::create_with_scalar(self.length())
    // }
}