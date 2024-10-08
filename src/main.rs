use std::fs::File;
use std::io::{self, Read, Write};
#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {let grade = match self.grade {
        GradeLevel::Bachelor => "Bachelor",
        GradeLevel::Master => "Master",
        GradeLevel::PhD => "PhD",
    };

    let major = match self.major {
        Major::ComputerScience => "Computer Science",
        Major::ElectricalEngineering => "Electrical Engineering",
    };

    println!(
        "Hello, my name is {}. I am a {} student majoring in {}.",
        self.name, grade, major
    );
        }
    }

fn main() {
    let s1 = Student::new("Juan Carlos".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();
}
