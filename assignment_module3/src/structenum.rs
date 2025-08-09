#[derive(Debug, PartialEq)]
enum BookStatus {
    Available,
    CheckedOut(i32),
    BeingRead,
    InRepair(String),
    Lost,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    status: BookStatus,
}

impl Book {
    // 1. new method
    fn new(title: String, author: String) -> Self {
        Book {
            title,
            author,
            status: BookStatus::Available,
        }
    }

    // 2. check_out method
    fn check_out(&mut self, days: i32) {
        self.status = BookStatus::CheckedOut(days);
    }

    // 3. return_book method
    fn return_book(&mut self) {
        self.status = BookStatus::Available;
    }

    // 4. send_for_repair method
    fn send_for_repair(&mut self, notes: String) {
        self.status = BookStatus::InRepair(notes);
    }

    // 5. mark_as_being_read method
    fn mark_as_being_read(&mut self) {
        self.status = BookStatus::BeingRead;
    }

    // 6. report_lost method
    fn report_lost(&mut self) {
        self.status = BookStatus::Lost;
    }

    // 7. display_status method
    fn display_status(&self) -> String {
        match &self.status {
            BookStatus::Available => format!("{} is available for borrowing.", self.title),
            BookStatus::CheckedOut(days) => format!("{} is checked out. Days until due: {}", self.title, days),
            BookStatus::BeingRead => format!("{} is currently being read.", self.title),
            BookStatus::InRepair(notes) => format!("{} is in repair. Notes: {}", self.title, notes),
            BookStatus::Lost => format!("{} has been reported lost.", self.title),
        }
    }
}
