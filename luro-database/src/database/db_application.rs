use twilight_model::oauth::PartialApplication;

use crate::DbApplicationType;

impl From<PartialApplication> for DbApplicationType {
    fn from(app: PartialApplication) -> Self {
        Self::PartialApplication(app)
    }
}
