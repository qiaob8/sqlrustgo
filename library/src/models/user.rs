pub trait User {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn borrow_limit(&self) -> u32;
    fn user_type(&self) -> &str;
}

pub struct Student {
    pub student_id: String,
    pub name: String,
}

impl Student {
    pub fn new(student_id: String, name: String) -> Self {
        Self { student_id, name }
    }
}

impl User for Student {
    fn id(&self) -> &str {
        &self.student_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn borrow_limit(&self) -> u32 {
        5
    }

    fn user_type(&self) -> &str {
        "学生"
    }
}

pub struct Teacher {
    pub teacher_id: String,
    pub name: String,
    pub department: String,
}

impl Teacher {
    pub fn new(teacher_id: String, name: String, department: String) -> Self {
        Self {
            teacher_id,
            name,
            department,
        }
    }
}

impl User for Teacher {
    fn id(&self) -> &str {
        &self.teacher_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn borrow_limit(&self) -> u32 {
        10
    }

    fn user_type(&self) -> &str {
        "教师"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new("S001".to_string(), "张三".to_string());
        assert_eq!(student.id(), "S001");
        assert_eq!(student.name(), "张三");
        assert_eq!(student.borrow_limit(), 5);
        assert_eq!(student.user_type(), "学生");
    }

    #[test]
    fn test_teacher_creation() {
        let teacher = Teacher::new(
            "T001".to_string(),
            "李教授".to_string(),
            "计算机学院".to_string(),
        );
        assert_eq!(teacher.id(), "T001");
        assert_eq!(teacher.name(), "李教授");
        assert_eq!(teacher.borrow_limit(), 10);
        assert_eq!(teacher.user_type(), "教师");
    }
}
