use crate::{entities::folder_info::FolderDelta, errors::CollaborateError, synchronizer::RevisionSyncObject};
use lib_ot::core::{Delta, OperationTransformable, PlainTextAttributes};

pub struct ServerFolder {
    folder_id: String,
    delta: FolderDelta,
}

impl ServerFolder {
    pub fn from_delta(folder_id: &str, delta: FolderDelta) -> Self {
        Self {
            folder_id: folder_id.to_owned(),
            delta,
        }
    }
}

impl RevisionSyncObject<PlainTextAttributes> for ServerFolder {
    fn id(&self) -> &str {
        &self.folder_id
    }

    fn compose(&mut self, other: &Delta<PlainTextAttributes>) -> Result<(), CollaborateError> {
        let new_delta = self.delta.compose(other)?;
        self.delta = new_delta;
        Ok(())
    }

    fn transform(
        &self,
        other: &Delta<PlainTextAttributes>,
    ) -> Result<(Delta<PlainTextAttributes>, Delta<PlainTextAttributes>), CollaborateError> {
        let value = self.delta.transform(other)?;
        Ok(value)
    }

    fn to_json(&self) -> String {
        self.delta.to_json()
    }

    fn set_delta(&mut self, new_delta: Delta<PlainTextAttributes>) {
        self.delta = new_delta;
    }
}
