use crate::db;
use juniper::{FieldResult, RootNode};

#[derive(GraphQLObject)]
struct Human {
    name: String,
    email: String,
    created: bool,
}

#[derive(GraphQLObject)]
struct HumanToken {
    token: String,
}

pub struct Query;

#[juniper::object(Context = db::DbConnection)]
impl Query {
    #[graphql(arguments(id(description = "id of the human")))]
    fn user(conn: &db::DbConnection, id: String) -> FieldResult<Human> {
        println!(
            "{:?}",
            conn.exec("MATCH (tom {name: \"Tom Hanks\"}) RETURN tom")
                .unwrap()
        );

        Ok(Human {
            email: "".to_string(),
            name: "".to_string(),
            created: false,
        })
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the actual universe")]
struct HumanInput {
    email: String,
    name: String,
    password: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature must login in the universe")]
struct LoginHumanInput {
    email: String,
    password: String,
}

pub struct Mutation;

#[juniper::object(Context = db::DbConnection)]
impl Mutation {
    fn registerUser(conn: &db::DbConnection, input: HumanInput) -> FieldResult<Human> {
        let mut query = format!(
            "MATCH (h:Human) WHERE h.email = \"{}\" RETURN count(h)",
            input.email.clone()
        );

        match conn.exec(query) {
            Ok(result) => {
                let mut created: bool = false;
                for row in result.rows() {
                    let counter: i64 = row.get("count(h)")?;
                    if counter == 0 {
                        let query = format!(
                            "CREATE (h:Human {{name: \"{}\", email: \"{}\", password: \"{}\"}}) RETURN h",
                            input.name, input.email, input.password
                        );
                        conn.exec(query)?;
                        created = true;
                    }
                }
                Ok(Human {
                    name: input.name,
                    email: input.email,
                    created: created,
                })
            }
            Err(err) => Err(err)?,
        }
    }

    fn loginHuman(conn: &db::DbConnection, input: LoginHumanInput) -> FieldResult<HumanToken> {
        let mut query = format!(
            "MATCH (h:Human) WHERE h.email = \"{}\" AND h.password = \"{}\" RETURN count(h)",
            input.email.clone(),
            input.password.clone()
        );

        match conn.exec(query) {
            Ok(result) => {
                for row in result.rows() {
                    let counter: i64 = row.get("count(h)")?;
                    if counter == 1 {}
                }
                Ok(HumanToken {
                    token: "mytokxxen".to_string(),
                })
            }
            Err(err) => Err(err)?,
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
