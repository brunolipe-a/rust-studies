// Fix the code so that it compiles.

struct Student {
    name: String,
    marks: u8,
    grade: char,
}

impl Student {
    fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
            grade: 'X',
        }
    }
}

fn set_student_grade(student: &mut Student) {
    match student.marks {
        0..=60 => student.grade = 'C',
        61..=80 => student.grade = 'B',
        81.. => student.grade = 'A',
    }

    println!("{} got {}!", student.name, student.grade);
}
fn main() {
    let mut students = vec![
        Student::new("Harry", 75),
        Student::new("Hermione", 99),
        Student::new("Ron", 60),
    ];

    students.iter_mut().for_each(set_student_grade);

    // for student in &students {
    //     println!("{} got {}!", student.name, student.grade);
    // }
}
