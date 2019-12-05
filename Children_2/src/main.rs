struct Mr_Asim {
    name: String
}

struct Mr_Allah_Bakhsh {
    name: String
}

pub trait Children {
    fn smile(&self)->bool;
    fn face_expression(&self)->bool;
    fn innocent(&self)->String;
}

pub fn adopt_mr_asim_T<T: Children>(item1: T, item2: T) ->String{
    format!("I would love to adopt these children_qualities")
}

pub fn adopt_mr_allah_bakhsh_T<T: Children>(item1: T, item2: T) ->String{
    format!("I would love to adopt these children_qualities")
}

impl Children for Mr_Asim {
    fn smile(&self)->bool{
        true
    }
    fn face_expression(&self)->bool{
        true
    }
    fn innocent(&self)->String{
        "uwaaaaaa".to_string()
    }
}

impl Children for Mr_Allah_Bakhsh {
    fn smile(&self)->bool{
        true
    }
    fn face_expression(&self)->bool{
        true
    }
    fn innocent(&self)->String{
        "hoooooo".to_string()
    }
}

fn main() {
    let mr_asim_1 = Mr_Asim {
       name: "Ali".to_string()
    };
    let mr_asim_2 = Mr_Asim {
        name: "Zeeshan".to_string()
    };
    println!("{}", mr_asim_1.smile());
    println!("{}", mr_asim_1.face_expression());
    println!("{}", mr_asim_1.innocent());
    let mr_allah_bakhsh_1 = Mr_Allah_Bakhsh {
        name: "Waqas".to_string()
    };
    let mr_allah_bakhsh_2 = Mr_Allah_Bakhsh {
        name: "Ahmed".to_string()
    };
    println!("{}", mr_allah_bakhsh_2.smile());
    println!("{}", mr_allah_bakhsh_2.face_expression());
    println!("{}", mr_allah_bakhsh_2.innocent());
}