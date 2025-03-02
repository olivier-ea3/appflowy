use crate::{
    controller::FolderManager,
    entities::{
        app::{App, AppId, CreateAppParams, UpdateAppParams},
        trash::{RepeatedTrash, RepeatedTrashId},
        view::{CreateViewParams, RepeatedViewId, UpdateViewParams, View, ViewId},
        workspace::{CreateWorkspaceParams, RepeatedWorkspace, UpdateWorkspaceParams, Workspace, WorkspaceId},
    },
    errors::FlowyError,
    event::WorkspaceEvent,
    services::{app::event_handler::*, trash::event_handler::*, view::event_handler::*, workspace::event_handler::*},
};
use flowy_database::DBConnection;

use lib_dispatch::prelude::*;
use lib_infra::future::FutureResult;
use lib_sqlite::ConnectionPool;
use std::sync::Arc;

pub trait WorkspaceDeps: WorkspaceUser + WorkspaceDatabase {}

pub trait WorkspaceUser: Send + Sync {
    fn user_id(&self) -> Result<String, FlowyError>;
    fn token(&self) -> Result<String, FlowyError>;
}

pub trait WorkspaceDatabase: Send + Sync {
    fn db_pool(&self) -> Result<Arc<ConnectionPool>, FlowyError>;

    fn db_connection(&self) -> Result<DBConnection, FlowyError> {
        let pool = self.db_pool()?;
        let conn = pool.get().map_err(|e| FlowyError::internal().context(e))?;
        Ok(conn)
    }
}

pub fn create(folder: Arc<FolderManager>) -> Module {
    let mut module = Module::new()
        .name("Flowy-Workspace")
        .data(folder.workspace_controller.clone())
        .data(folder.app_controller.clone())
        .data(folder.view_controller.clone())
        .data(folder.trash_controller.clone())
        .data(folder.clone());

    module = module
        .event(WorkspaceEvent::CreateWorkspace, create_workspace_handler)
        .event(WorkspaceEvent::ReadCurWorkspace, read_cur_workspace_handler)
        .event(WorkspaceEvent::ReadWorkspaces, read_workspaces_handler)
        .event(WorkspaceEvent::OpenWorkspace, open_workspace_handler)
        .event(WorkspaceEvent::ReadWorkspaceApps, read_workspace_apps_handler);

    module = module
        .event(WorkspaceEvent::CreateApp, create_app_handler)
        .event(WorkspaceEvent::ReadApp, read_app_handler)
        .event(WorkspaceEvent::UpdateApp, update_app_handler)
        .event(WorkspaceEvent::DeleteApp, delete_app_handler);

    module = module
        .event(WorkspaceEvent::CreateView, create_view_handler)
        .event(WorkspaceEvent::ReadView, read_view_handler)
        .event(WorkspaceEvent::UpdateView, update_view_handler)
        .event(WorkspaceEvent::DeleteView, delete_view_handler)
        .event(WorkspaceEvent::DuplicateView, duplicate_view_handler)
        .event(WorkspaceEvent::OpenDocument, open_document_handler)
        .event(WorkspaceEvent::CloseView, close_view_handler)
        .event(WorkspaceEvent::ApplyDocDelta, document_delta_handler);

    module = module
        .event(WorkspaceEvent::ReadTrash, read_trash_handler)
        .event(WorkspaceEvent::PutbackTrash, putback_trash_handler)
        .event(WorkspaceEvent::DeleteTrash, delete_trash_handler)
        .event(WorkspaceEvent::RestoreAllTrash, restore_all_trash_handler)
        .event(WorkspaceEvent::DeleteAllTrash, delete_all_trash_handler);

    module = module.event(WorkspaceEvent::ExportDocument, export_handler);

    module
}

pub trait FolderCouldServiceV1: Send + Sync {
    fn init(&self);

    // Workspace
    fn create_workspace(&self, token: &str, params: CreateWorkspaceParams) -> FutureResult<Workspace, FlowyError>;

    fn read_workspace(&self, token: &str, params: WorkspaceId) -> FutureResult<RepeatedWorkspace, FlowyError>;

    fn update_workspace(&self, token: &str, params: UpdateWorkspaceParams) -> FutureResult<(), FlowyError>;

    fn delete_workspace(&self, token: &str, params: WorkspaceId) -> FutureResult<(), FlowyError>;

    // View
    fn create_view(&self, token: &str, params: CreateViewParams) -> FutureResult<View, FlowyError>;

    fn read_view(&self, token: &str, params: ViewId) -> FutureResult<Option<View>, FlowyError>;

    fn delete_view(&self, token: &str, params: RepeatedViewId) -> FutureResult<(), FlowyError>;

    fn update_view(&self, token: &str, params: UpdateViewParams) -> FutureResult<(), FlowyError>;

    // App
    fn create_app(&self, token: &str, params: CreateAppParams) -> FutureResult<App, FlowyError>;

    fn read_app(&self, token: &str, params: AppId) -> FutureResult<Option<App>, FlowyError>;

    fn update_app(&self, token: &str, params: UpdateAppParams) -> FutureResult<(), FlowyError>;

    fn delete_app(&self, token: &str, params: AppId) -> FutureResult<(), FlowyError>;

    // Trash
    fn create_trash(&self, token: &str, params: RepeatedTrashId) -> FutureResult<(), FlowyError>;

    fn delete_trash(&self, token: &str, params: RepeatedTrashId) -> FutureResult<(), FlowyError>;

    fn read_trash(&self, token: &str) -> FutureResult<RepeatedTrash, FlowyError>;
}

pub trait FolderCouldServiceV2: Send + Sync {}
