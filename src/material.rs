use crate::vector3::Vector3;

pub type Color3 = Vector3;

#[derive(Debug, PartialEq, Clone)]
pub enum Material{
    Diffuse(Color3),
    Specular
}
