#![allow(dead_code, unused_variables)]

const MINIMUM_GRADE: f32 = 3.5; 

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn good_students(students: &Vec<&str>) -> Vec<Student> {
    let mut good_students = vec![];

    for s in students {
        let mut s = s.split(" ");
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();

            if let Ok(gpa) = gpa {
                if gpa >= MINIMUM_GRADE {
                    good_students.push(Student {name, gpa});
                }
            }
        }
    }

    good_students
}

fn good_students_combinators(students: &Vec<&str>) -> Vec<Student> {
    let good_students: Vec<Student> = students.iter()
        // combinator that receives an item and returns another
        .map(|s| {
            let mut s = s.split(" ");
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;
            
            Some(Student { name, gpa }) // return Optional because we are propagating errors via ? above
        // combinator that removes wrapper Option type returning Student types
        }).flatten()
        // combinator that returns the same type if condition is met
        .filter(|s| s.gpa >= MINIMUM_GRADE)
        .collect(); // convert into a Collection (it is necessary to set the type in the var definition above)

    good_students
}

// simpler way where flatten/filter are 
fn good_students_combinators2(students: &Vec<&str>) -> Vec<Student> {
    let good_students: Vec<Student> = students.iter()
        // combinator that receives an item and returns another type in an Option
        // item is discarded if option is None
        .filter_map(|s| {
            let mut s = s.split(" ");
            let name = s.next()?.to_owned(); // propagate an error if failure
            let gpa = s.next()?.parse::<f32>().ok()?; // propagate an error or return None if error
            
            if gpa < MINIMUM_GRADE {
                return None; // discard this item
            }

            Some(Student { name, gpa }) // return the Student
        })
        .collect(); // convert into a Collection (it is necessary to set the type in the var definition above)

    good_students
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let good_students = good_students(&students);
    for s in good_students {
        println!("{:?}", s);
    }

    for s in good_students_combinators(&students) {
        println!("{:?}", s);
    }

    for s in good_students_combinators2(&students) {
        println!("{:?}", s);
    }
}
