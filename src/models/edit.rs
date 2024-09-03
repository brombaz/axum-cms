use modql::{field::Fields, filter::{FilterGroups, FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString}};
use sea_query::{Condition, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_with::serde_as;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, FromRow};

use crate::ctx::Ctx;

use super::{base::{self, DbBmc}, AppState, ModelResult};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, FromRow, Fields)]
/// Complete "Edit" model as-is in the database
pub struct Edit {
	pub id: i64,
	pub editor_id: i64,
	pub post_id: i64,
	pub new_content: String,
	pub status: EditStatus,
	#[serde_as(as = "Rfc3339")]
	pub created_at: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub updated_at: OffsetDateTime
}


/// Complete "Edit Status" enum as-is in the database
// #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "edit_status")]
#[derive(Clone, Debug, Deserialize, strum_macros::Display, Serialize, sqlx::Type)]
#[sqlx(type_name = "edit_status")]
pub enum EditStatus {
	PENDING,
	ACCEPTED,
	REJECTED
}

impl From<EditStatus> for sea_query::Value {
	fn from(val: EditStatus) -> Self {
		val.to_string().into()
	}
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required to create an edit suggestion in the database
pub struct EditForCreate {
	pub post_id: i64,
	pub new_content: String,
	pub editor_id: i64
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to create an edit suggestion in the database
pub struct EditForCreateRequestBody {
	pub post_id: i64,
	pub new_content: String,
}


#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to edit an edit_suggestion
pub struct EditForUpdate {
	pub new_content: Option<String>,
}


#[derive(Serialize, Debug)]
/// Struct holding fields to be sent to the client as a resulting EditSuggestion
pub struct EditForResult {
	pub status: EditStatus,
	pub new_content: String
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct EditFilter {
	id: Option<OpValsInt64>,

	editor_id: Option<OpValsInt64>,
	post_id: Option<OpValsInt64>,
	new_content: Option<OpValsString>,
	status: Option<OpValsBool>
}

#[derive(Iden)]
enum EditIden {
	Id,
	PostId,
	EditorId
}


pub struct EditBmc;

impl DbBmc for EditBmc {
	const TABLE: &'static str = "edits";
}

impl EditBmc {
	pub async fn create(
		ctx: &Ctx,
		app_state: &AppState,
		data: EditForCreate,
	) -> ModelResult<i64> {
		let db = app_state.db();
		base::create::<EditBmc, _>(ctx, app_state, data).await
	}

	pub async fn get(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<Edit> {
		base::get::<Self, _>(ctx, app_state, id).await // Underscore on the second generic parameter because we return a model of author, the compiler can infer
	}

	pub async fn list(ctx: &Ctx, app_state: &AppState, filters: Option<EditFilter>, list_options: Option<ListOptions>) -> ModelResult<Vec<Edit>> {
		base::list::<Self, _, _>(ctx, app_state, filters, list_options).await
	}
	
	pub async fn update(ctx: &Ctx, app_state: &AppState, id: i64, post_e: EditForUpdate) -> ModelResult<()> {
		base::update::<Self, _>(ctx, app_state, id, post_e).await
	}


	pub async fn delete(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<()> {
		base::delete::<Self>(ctx, app_state, id).await
	}
}

// region:    --- Tests
#[cfg(test)]

mod tests {
	#![allow(unused)]
	use crate::{_dev_utils, models::{author::AuthorBmc, post::PostBmc, ModelError}};

	use super::*;
	use anyhow::{Ok, Result};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_editors = &[("name", "email@mail", "password")];
		let fx_posts = &[("test_list_ok-post 01", "content 01", 1000)];

		_dev_utils::seed_posts(&ctx, &app_state, fx_posts).await?;
		_dev_utils::seed_authors(&ctx, &app_state, fx_editors).await?;

		let posts = PostBmc::list(&app_state, None, None).await?;
		let editors = AuthorBmc::list(&app_state, None, None).await?;
		let post = &posts[0];
		let editor = &editors[0];

		let fx_new_content = "Here is a suggestion";
		let fx_post_id = post.id;
		let fx_editor_id = editor.id;
		
		// -- Exec
		let edit_c = EditForCreate {
			new_content: fx_new_content.to_string(),
			post_id: fx_post_id,
			editor_id: fx_editor_id
		};

		
		// -- Check
		let id = EditBmc::create(&ctx, &app_state, edit_c).await?;
		let edit = EditBmc::get(&ctx, &app_state, id).await?;


		assert_eq!(edit.new_content, fx_new_content);
		assert_eq!(edit.post_id, fx_post_id);
		assert_eq!(edit.editor_id, fx_editor_id);

		// -- Clean
		EditBmc::delete(&ctx, &app_state, id).await?;
		
		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = EditBmc::get(&ctx, &app_state, fx_id).await;

		// println!("{:?}", res);
		assert!(
			matches!(
				res,
				Err(ModelError::EntityNotFound {
					entity: "edits",
					id: fx_id
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests