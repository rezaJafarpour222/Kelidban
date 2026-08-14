#[derive(Debug, Clone)]
pub enum FormField {
    Text {
        label: String,
        value: String,
    },
    Toggle {
        label: String,
        value: bool,
    },
    Select {
        label: String,
        options: Vec<Options>,
        selected: usize,
    },
}
#[derive(Debug, Clone)]
pub enum Options {
    GeneratePassword,
    EnterPassword,
}
fn init_form_fields() -> Vec<FormField> {
    vec![
        FormField::Text {
            label: "Title".to_string(),
            value: String::new(),
        },
        FormField::Text {
            label: "Username".to_string(),
            value: String::new(),
        },
        FormField::Select {
            label: "Password".to_string(),
            options: vec![Options::GeneratePassword, Options::EnterPassword],
            selected: 0,
        },
        FormField::Text {
            label: "URL".to_string(),
            value: String::new(),
        },
        FormField::Text {
            label: "Notes".to_string(),
            value: String::new(),
        },
    ]
}
