use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Message {
    pub severity: Severity,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Report {
    pub errors: usize,
    pub warnings: usize,
    pub messages: Vec<Message>,
}

impl Report {
    pub fn valid(&self) -> bool {
        self.errors == 0
    }
}

pub(crate) fn push_error(report: &mut Report, msg: impl Into<String>) {
    report.errors += 1;
    push_message(report, Severity::Error, msg);
}

#[inline]
fn push_message(report: &mut Report, severity: Severity, msg: impl Into<String>) {
    report.messages.push(Message {
        severity,
        message: msg.into(),
    });
}

#[inline]
pub(crate) fn push_error_times(report: &mut Report, msg: &'static str, n: usize) {
    if n == 0 {
        return;
    }

    report.errors += n;
    report.messages.reserve(n);

    let msg = msg.to_owned();
    for _ in 1..n {
        report.messages.push(Message {
            severity: Severity::Error,
            message: msg.clone(),
        });
    }
    report.messages.push(Message {
        severity: Severity::Error,
        message: msg,
    });
}

pub(crate) fn push_warning(report: &mut Report, msg: impl Into<String>) {
    report.warnings += 1;
    push_message(report, Severity::Warning, msg);
}

pub(crate) fn push_warning_level(
    report: &mut Report,
    warning_level: i32,
    level: i32,
    msg: impl Into<String>,
) {
    if level <= warning_level {
        push_warning(report, msg);
    }
}

#[cfg(test)]
mod push_error_times_tests {
    use super::{Report, Severity, push_error_times, push_warning};

    #[test]
    fn pushes_zero_errors_without_allocating_messages() {
        let mut report = Report::default();
        push_error_times(&mut report, "Invalid input.", 0);

        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn pushes_one_error_without_off_by_one() {
        let mut report = Report::default();
        push_error_times(&mut report, "Invalid input.", 1);

        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(report.messages[0].message, "Invalid input.");
    }

    #[test]
    fn pushes_n_errors_and_messages() {
        let mut report = Report::default();
        push_error_times(&mut report, "Invalid input.", 3);
        assert_eq!(report.errors, 3);
        assert_eq!(report.messages.len(), 3);
        assert!(
            report
                .messages
                .iter()
                .all(|m| m.severity == Severity::Error && m.message == "Invalid input.")
        );
    }

    #[test]
    fn pushes_two_errors_without_off_by_one() {
        let mut report = Report::default();
        push_error_times(&mut report, "Invalid input.", 2);

        assert_eq!(report.errors, 2);
        assert_eq!(report.messages.len(), 2);
        assert!(
            report
                .messages
                .iter()
                .all(|m| m.severity == Severity::Error && m.message == "Invalid input.")
        );
    }

    #[test]
    fn pushes_nothing_when_n_is_zero() {
        let mut report = Report::default();
        push_error_times(&mut report, "Invalid input.", 0);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn n_is_zero_is_a_noop_on_nonempty_report() {
        let mut report = Report::default();
        report.errors = 5;
        report.messages.push(super::Message {
            severity: Severity::Error,
            message: "x".to_string(),
        });

        push_error_times(&mut report, "Invalid input.", 0);

        assert_eq!(report.errors, 5);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "x");
    }

    #[test]
    fn appends_messages_without_affecting_warnings() {
        let mut report = Report::default();
        push_warning(&mut report, "Warn.");

        push_error_times(&mut report, "Invalid input.", 2);

        assert_eq!(report.errors, 2);
        assert_eq!(report.warnings, 1);
        assert_eq!(report.messages.len(), 3);
        assert_eq!(report.messages[0].severity, Severity::Warning);
        assert_eq!(report.messages[0].message, "Warn.");
        assert!(
            report.messages[1..]
                .iter()
                .all(|m| { m.severity == Severity::Error && m.message == "Invalid input." })
        );
    }

    #[test]
    fn appends_one_error_without_affecting_warnings() {
        let mut report = Report::default();
        push_warning(&mut report, "Warn.");

        push_error_times(&mut report, "Invalid input.", 1);

        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 1);
        assert_eq!(report.messages.len(), 2);
        assert_eq!(report.messages[0].severity, Severity::Warning);
        assert_eq!(report.messages[0].message, "Warn.");
        assert_eq!(report.messages[1].severity, Severity::Error);
        assert_eq!(report.messages[1].message, "Invalid input.");
    }

    #[test]
    fn can_be_called_multiple_times_and_appends_messages() {
        let mut report = Report::default();
        push_error_times(&mut report, "A", 2);
        push_error_times(&mut report, "B", 1);

        assert_eq!(report.errors, 3);
        assert_eq!(report.messages.len(), 3);
        assert_eq!(report.messages[0].message, "A");
        assert_eq!(report.messages[1].message, "A");
        assert_eq!(report.messages[2].message, "B");
        assert!(
            report
                .messages
                .iter()
                .all(|m| m.severity == Severity::Error)
        );
    }

    #[test]
    fn appends_single_error_to_nonempty_report() {
        let mut report = Report::default();
        report.errors = 2;
        report.warnings = 1;
        report.messages.push(super::Message {
            severity: Severity::Warning,
            message: "Warn.".to_string(),
        });

        push_error_times(&mut report, "Invalid input.", 1);

        assert_eq!(report.errors, 3);
        assert_eq!(report.warnings, 1);
        assert_eq!(report.messages.len(), 2);
        assert_eq!(report.messages[0].message, "Warn.");
        assert_eq!(report.messages[1].severity, Severity::Error);
        assert_eq!(report.messages[1].message, "Invalid input.");
    }
}

#[cfg(test)]
mod push_error_warning_tests {
    use super::{Report, Severity, push_error, push_warning};

    #[test]
    fn push_error_increments_error_count_and_records_message() {
        let mut report = Report::default();
        push_error(&mut report, "Bad.");
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(report.messages[0].message, "Bad.");
    }

    #[test]
    fn push_warning_increments_warning_count_and_records_message() {
        let mut report = Report::default();
        push_warning(&mut report, "Warn.");
        assert_eq!(report.errors, 0);
        assert_eq!(report.warnings, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Warning);
        assert_eq!(report.messages[0].message, "Warn.");
    }
}
