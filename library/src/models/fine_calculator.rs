pub trait FineCalculator {
    fn calculate(&self, overdue_days: i64) -> f64;
    
    fn fine_rate(&self) -> f64;
}

pub struct DefaultFineCalculator {
    rate_per_day: f64,
}

impl DefaultFineCalculator {
    pub fn new(rate_per_day: f64) -> Self {
        Self { rate_per_day }
    }
}

impl Default for DefaultFineCalculator {
    fn default() -> Self {
        Self::new(0.2)
    }
}

impl FineCalculator for DefaultFineCalculator {
    fn calculate(&self, overdue_days: i64) -> f64 {
        if overdue_days <= 0 {
            0.0
        } else {
            overdue_days as f64 * self.rate_per_day
        }
    }
    
    fn fine_rate(&self) -> f64 {
        self.rate_per_day
    }
}

pub struct StudentFineCalculator {
    rate_per_day: f64,
    max_fine: f64,
}

impl StudentFineCalculator {
    pub fn new(rate_per_day: f64, max_fine: f64) -> Self {
        Self { rate_per_day, max_fine }
    }
}

impl Default for StudentFineCalculator {
    fn default() -> Self {
        Self::new(0.1, 50.0)
    }
}

impl FineCalculator for StudentFineCalculator {
    fn calculate(&self, overdue_days: i64) -> f64 {
        if overdue_days <= 0 {
            0.0
        } else {
            let fine = overdue_days as f64 * self.rate_per_day;
            fine.min(self.max_fine)
        }
    }
    
    fn fine_rate(&self) -> f64 {
        self.rate_per_day
    }
}

pub struct TeacherFineCalculator {
    rate_per_day: f64,
}

impl TeacherFineCalculator {
    pub fn new(rate_per_day: f64) -> Self {
        Self { rate_per_day }
    }
}

impl Default for TeacherFineCalculator {
    fn default() -> Self {
        Self::new(0.05)
    }
}

impl FineCalculator for TeacherFineCalculator {
    fn calculate(&self, overdue_days: i64) -> f64 {
        if overdue_days <= 0 {
            0.0
        } else {
            overdue_days as f64 * self.rate_per_day
        }
    }
    
    fn fine_rate(&self) -> f64 {
        self.rate_per_day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fine_calculator() {
        let calculator = DefaultFineCalculator::default();
        assert_eq!(calculator.calculate(0), 0.0);
        assert_eq!(calculator.calculate(5), 1.0);
        assert_eq!(calculator.calculate(10), 2.0);
    }

    #[test]
    fn test_student_fine_calculator() {
        let calculator = StudentFineCalculator::new(0.1, 50.0);
        assert_eq!(calculator.calculate(0), 0.0);
        assert_eq!(calculator.calculate(10), 1.0);
        assert_eq!(calculator.calculate(600), 50.0);
    }

    #[test]
    fn test_teacher_fine_calculator() {
        let calculator = TeacherFineCalculator::default();
        assert_eq!(calculator.calculate(0), 0.0);
        assert_eq!(calculator.calculate(10), 0.5);
        assert_eq!(calculator.calculate(20), 1.0);
    }
}
