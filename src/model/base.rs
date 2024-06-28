use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};

use modql::field::HasFields;
use modql::SIden;
use sea_query::{
  ConditionalStatement, Expr, Iden, IntoIden, PostgresQueryBuilder, Query, TableRef
};
use sea_query_binder::SqlxBinder;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

#[derive(Iden)]
pub enum ColumnIden {
  Id,
}

pub trait DbBmc {
  const TABLE: &'static str;

  fn table_ref() -> TableRef {
    TableRef::Table(SIden(Self::TABLE).into_iden())
  }
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
  MC: DbBmc,
  E: HasFields,
{
  let db = mm.db();

  // Prep data
  let fields = data.not_none_fields(); // all_fields()
  let (columns, sea_values) = fields.for_sea_insert();

  // -- Build query
  let mut query = Query::insert();
  query
    .into_table(MC::table_ref())
    .columns(columns)
    .values(sea_values)?
    .returning(Query::returning().columns([ColumnIden::Id]));

  // -- Exec query
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
  let (id,) = sqlx::query_as_with::<_, (i64,), _>(&sql, values)
    .fetch_one(db)
    .await?;

  Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
  MC: DbBmc,
  E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
  E: HasFields,
{
  let db = mm.db();

  // -- Build query
  let mut query = Query::select();
  query
    .from(MC::table_ref())
    .columns(E::field_column_refs())
    .and_where(Expr::col(ColumnIden::Id).eq(id));

  // -- Exec query
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
  let task = sqlx::query_as_with::<_, E, _>(&sql, values)
    .fetch_optional(db)
    .await?
    .ok_or(Error::EntityNotFound {
      entity: MC::TABLE,
      id,
    })?;

  // let task: E = sqlb::select()
  //   .table(MC::TABLE)
  //   .columns(E::field_names())
  //   .and_where("id", "=", id)
  //   .fetch_optional(db)
  //   .await?
  //   .ok_or(Error::EntityNotFound {
  //     entity: MC::TABLE,
  //     id,
  //   })?;

  Ok(task)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
  MC: DbBmc,
  E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
  E: HasFields,
{
  let db = mm.db();

  // -- Build query
  let mut query = Query::select();
  query.from(MC::table_ref()).columns(E::field_column_refs());

  // -- Exec query
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
  let entits = sqlx::query_as_with::<_, E, _>(&sql, values)
    .fetch_all(db)
    .await?;

  Ok(entits)
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
  MC: DbBmc,
  E: HasFields,
{
  let db = mm.db();

  // Prep data
  let fields = data.not_none_fields();
  let fields = fields.for_sea_update();

  // Build query
  let mut query = Query::update();
  query
    .table(MC::table_ref())
    .values(fields)
    .and_where(Expr::col(ColumnIden::Id).eq(id));

  // Extract query
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
  let count = sqlx::query_with(&sql, values)
    .execute(db)
    .await?
    .rows_affected();

  // Check result
  if count == 0 {
    return Err(Error::EntityNotFound {
      entity: MC::TABLE,
      id,
    });
  } else {
    return Ok(());
  }
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
  MC: DbBmc,
{
  let db = mm.db();

  // -- Build query
  let mut query = Query::delete();
  query
    .from_table(MC::table_ref())
    .and_where(Expr::col(ColumnIden::Id).eq(id));

  // -- Exec query
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
  let count = sqlx::query_with(&sql, values)
    .execute(db)
    .await?
    .rows_affected();

  if count == 0 {
    return Err(Error::EntityNotFound {
      entity: MC::TABLE,
      id,
    });
  } else {
    return Ok(());
  }
}
