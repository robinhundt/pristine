use crate::schemas::user::UserInput;
use juniper::{
    DefaultScalarValue, EmptyMutation, EmptySubscription, FieldResult, FromInputValue, InputValue,
    RootNode, ID,
};
use sqlx::SqlitePool;
use std::convert::TryInto;

pub mod user;
pub mod task;

use user::User;
use crate::schemas::task::{TaskInput, Task, ScheduledTask};
use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Context {
    pub pool: SqlitePool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List all Users")]
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
select id as "id: i32", name from users
        "#
        )
        .fetch_all(&context.pool)
        .await?;
        Ok(users)
    }

    #[graphql(description = "List all Tasks")]
    async fn tasks(context: &Context) -> FieldResult<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
select id as "id: i32", name, description, duration as "duration: i32" from tasks
        "#
        )
            .fetch_all(&context.pool)
            .await?;
        Ok(tasks)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Create a user")]
    async fn create_user(ctx: &Context, user: UserInput) -> FieldResult<User> {
        let id: i32 = sqlx::query!(
            r#"
insert into users (name)
values (?)
        "#,
            user.name
        )
            .execute(&ctx.pool)
            .await?
            .last_insert_rowid()
            .try_into()?;
        Ok(User {
            id,
            name: user.name,
        })
    }

    #[graphql(description = "Create a task")]
    async fn create_task(ctx: &Context, task: TaskInput) -> FieldResult<Task> {
        let id: i32 = sqlx::query!(
            r#"
insert into tasks (name, description, duration)
values (?, ?, ?)
        "#,
            task.name, task.description, task.duration
        )
            .execute(&ctx.pool)
            .await?
            .last_insert_rowid()
            .try_into()?;
        Ok(Task {
            id,
            name: task.name,
            description: task.description,
            duration: task.duration
        })
    }

    #[graphql(description = "Schedule a task for a user")]
    async fn schedule_task(ctx: &Context, user_id: i32, task_id: i32, scheduled_at: NaiveDateTime) -> FieldResult<ScheduledTask> {
        let id: i32 = sqlx::query!(
            r#"
insert into scheduled_tasks (user, task, scheduled_at)
values (?, ?, ?)
        "#,
            user_id, task_id, scheduled_at
        )
            .execute(&ctx.pool)
            .await?
            .last_insert_rowid()
            .try_into()?;
        Ok(ScheduledTask {
            id,
            completed: false,
            user: user_id,
            task: task_id,
            scheduled_at,
        })
    }

    #[graphql(description = "Complete a scheduled task")]
    async fn set_task_completed(ctx: &Context, scheduled_task_id: i32, completed: bool) -> FieldResult<ScheduledTask> {
        sqlx::query!(r#"
update scheduled_tasks
set completed = ?
where id = ?
        "#, completed, scheduled_task_id).execute(&ctx.pool).await?;
        let scheduled_task = sqlx::query_as!(ScheduledTask, r#"
select id as "id: i32", user as "user: i32", task as "task: i32", scheduled_at, completed from scheduled_tasks
where id = ?
        "#, scheduled_task_id)
            .fetch_one(&ctx.pool)
            .await?;
        Ok(scheduled_task)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
