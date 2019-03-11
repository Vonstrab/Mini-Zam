#[derive(Debug, PartialEq, Clone)]
pub enum Mlvalue {
    Entier(i64),
    Environement(Vec<Mlvalue>),
    Fermeture(usize, Vec<Mlvalue>),
}

impl Mlvalue {
    pub fn as_int(&self) -> i64 {
        println!("self {:?}",self );
        match self {
            Mlvalue::Entier(n) => *n,
            _ => panic!("Not an Entier"),
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Mlvalue::Entier(1) => true,
            Mlvalue::Entier(0) => false,
            _ => panic!("Not a Bool"),
        }
    }

    pub fn as_env(&self) -> Vec<Mlvalue> {
        match self {
            Mlvalue::Entier(x)=>vec![Mlvalue::Entier(*x)],
            Mlvalue::Environement(env) => env.clone(),
            _ => panic!("Not an Env"),
        }
    }
    pub fn to_entier(value: bool) -> Mlvalue {
        match value {
            true => Mlvalue::Entier(1),
            false => Mlvalue::Entier(0),
        }
    }
}
