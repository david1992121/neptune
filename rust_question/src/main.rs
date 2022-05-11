#[derive(Debug)]
pub struct School {
  pub school_name: String,
  pub students: Vec<Student>,
}

#[derive(Debug)]
pub struct Student {
  pub id: u64, 
  pub name: String,
}

fn main() {
    let mut school = create_school();

    //Your code here!

    println!("{:?}", school);    
}

fn create_school() -> School {
  let students = vec![
    Student{ 
      id: 0,
      name: "rusted_ruby".to_string(),
    }, 
    Student {
      id: 1, 
      name: "tempest_io".to_string(),
    },
    Student {
      id: 2, 
      name: "".to_string(),
    },
    Student {
      id: 3, 
      name: "azul_developer".to_string(),
    },
  ];

  School {
    school_name: "Rust University".to_string(),
    students
  }
}
