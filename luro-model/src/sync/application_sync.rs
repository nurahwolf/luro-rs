use twilight_model::oauth::PartialApplication;

pub enum ApplicationSync<'a> {
    PartialApplication(&'a PartialApplication),
}

impl<'a> From<&'a PartialApplication> for ApplicationSync<'a> {
    fn from(app: &'a PartialApplication) -> Self {
        Self::PartialApplication(app)
    }
}
