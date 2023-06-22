use crate::lal;


pub struct ray_basic
{
    A : lal::vec3,
    B : lal::vec3,
}

impl ray_basic{
    pub fn create(a : lal::vec3, b : lal::vec3) -> Self{
        Self {
            A: a,
            B: b,
        }
    }
    pub fn origin(&self) -> lal::vec3{
        self.A
    }

    pub fn direction(&self) -> lal::vec3{
        self.B
    }

    pub fn point_at_parameter(&self, t : f64) -> lal::vec3{
        self.A + self.B * t
    }
}