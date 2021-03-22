use juniper::{GraphQLInputObject, FieldResult};
use crate::schemas::{Context, user::User};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub duration: i32,

}

#[juniper::graphql_object]
impl Task {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Task Input")]
pub struct TaskInput {
    pub name: String,
    pub description: String,
    pub duration: i32,
}


#[derive(Debug)]
pub struct ScheduledTask {
    pub id: i32,
    pub scheduled_at: NaiveDateTime,
    pub completed: bool,
    pub user: i32,
    pub task: i32
}

#[juniper::graphql_object(Context = Context)]
impl ScheduledTask {
    fn id(&self) -> i32 {
        self.id
    }

    fn scheduled_at(&self) -> &NaiveDateTime {
        &self.scheduled_at
    }

    fn completed(&self) -> bool {
        self.completed
    }

    #[graphql(description = "Retrieve user")]
    async fn user(&self, ctx: &Context) -> FieldResult<User> {
        let user = sqlx::query_as!(User,r#"
select id as "id: i32", name  from users where id = ?
        "#, self.user).fetch_one(&ctx.pool).await?;
        Ok(user)
    }

    #[graphql(description = "Retrieve task")]
    async fn task(&self, ctx: &Context) -> FieldResult<Task> {
        let task = sqlx::query_as!(Task,r#"
select id as "id: i32", name, description, duration as "duration: i32"  from tasks where id = ?
        "#, self.task).fetch_one(&ctx.pool).await?;
        Ok(task)
    }
}