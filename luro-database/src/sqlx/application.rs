use twilight_model::oauth::PartialApplication;

mod count_applications;
mod update_application;

pub enum DbApplicationType {
    PartialApplication(PartialApplication)
}

pub struct DbApplication {
    pub application_id: i64,
}