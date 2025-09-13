use std::rc::Rc;

#[derive(Debug)]
pub struct Type{
    pub name: String
}

pub fn base_types() -> Vec<Type>{
    vec![
        Type{
            name: "int".to_owned()
        },
        Type{
            name: "float".to_owned()
        },
        Type{
            name: "Vec2".to_owned()
        },
        Type{
            name: "void".to_owned()
        }
    ]
}

#[derive(Debug)]
pub struct Value{
    pub typ: Rc<Type>,
    pub data: ValueData
}

#[derive(Debug)]
pub enum ValueData{
    Int(i32),
    Float(f32),
}