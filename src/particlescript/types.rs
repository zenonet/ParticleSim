#[derive(Debug)]
pub struct Type{
    name: String
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
    ]
}