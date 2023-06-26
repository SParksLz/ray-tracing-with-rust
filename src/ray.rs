use crate::lal;


pub struct ray_basic
{
    A : lal::vector::vec3,
    B : lal::vector::vec3,
}

impl ray_basic{
    pub fn create(a : lal::vector::vec3, b : lal::vector::vec3) -> Self{
        Self {
            A: a,
            B: b,
        }
    }
    pub fn origin(&self) -> lal::vector::vec3{
        self.A
    }

    pub fn direction(&self) -> lal::vector::vec3{
        self.B
    }

    pub fn point_at_parameter(&self, t : f64) -> lal::vector::vec3{
        self.A + self.B * t
    }
}