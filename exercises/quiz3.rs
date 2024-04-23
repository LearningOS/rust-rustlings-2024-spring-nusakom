// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub grade_type: GradeType, // 新增一个字段，用于表示成绩类型
}

#[derive(PartialEq, Debug)]
enum GradeType {
    Numeric,
    Alphabetic,
}

impl ReportCard {
    pub fn new(grade: f32, student_name: String, student_age: u8, grade_type: GradeType) -> Self {
        ReportCard {
            grade,
            student_name,
            student_age,
            grade_type,
        }
    }

    pub fn print(&self) -> String {
        match self.grade_type {
            GradeType::Numeric => format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade),
            GradeType::Alphabetic => {
                let grade_letter = match self.grade {
                    90.0..=100.0 => 'A',
                    80.0..=89.9 => 'B',
                    70.0..=79.9 => 'C',
                    60.0..=69.9 => 'D',
                    _ => 'F',
                };
                format!("{} ({}) - achieved a grade of {}",
                    &self.student_name, &self.student_age, grade_letter)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard::new(2.1, "Tom Wriggle".to_string(), 12, GradeType::Numeric);
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard::new(95.0, "Gary Plotter".to_string(), 11, GradeType::Alphabetic);
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A"
        );
    }
}
