use crate::vector3::Vector3;

type Color3 = Vector3;

pub enum Material{
    Diffuse(Color3),
    Specular
}
