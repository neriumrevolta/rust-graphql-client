use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use exitfailure::ExitFailure;

struct Order {
    category: String,
}

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/schema.graphql",
query_path = "src/query.graphql",
response_derives = "Debug"
)]
pub struct MyQuery;

async fn perform_my_query(variables: my_query::Variables) -> Result<Response<my_query::ResponseData>, Box<dyn Error>> {
    let request_body = MyQuery::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post("https://api.thegraph.com/subgraphs/name/decentraland/marketplace").json(&request_body).send().await?;
    let response_body: Response<my_query::ResponseData> = res.json().await?;

    Ok(response_body)
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let variables = my_query::Variables;
    let result = perform_my_query(variables).await;

    println!("{:?}", result.unwrap().data.unwrap());

    Ok(())
}
