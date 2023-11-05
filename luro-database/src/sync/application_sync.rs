use twilight_model::oauth::PartialApplication;

pub enum ApplicationSync {
    PartialApplication(PartialApplication),
}

impl From<PartialApplication> for ApplicationSync {
    fn from(app: PartialApplication) -> Self {
        Self::PartialApplication(app)
    }
}