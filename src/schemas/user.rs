use juniper::{GraphQLInputObject, FieldResult};
use crate::schemas::Context;
use crate::schemas::task::ScheduledTask;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    #[graphql(description = "Retrieve scheduled tasks")]
    async fn scheduled_tasks(&self, ctx: &Context) -> FieldResult<Vec<ScheduledTask>> {
        let scheduled_tasks = sqlx::query_as!(ScheduledTask, r#"
select users.id as "user: i32", scheduled_tasks.id as "id: i32", scheduled_tasks.task as "task: i32", scheduled_at, completed
from scheduled_tasks
inner join users
on users.id = scheduled_tasks.user
        "#).fetch_all(&ctx.pool).await?;
        Ok(scheduled_tasks)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
}
