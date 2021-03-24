use graphql_client::{GraphQLQuery, Response};
use serde_derive::{Deserialize, Serialize};
use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result as AnyhowResult;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Category {
    Estate, Parcel, Wearable, End
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    category: Category,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct MyQuery;

async fn perform_my_query(
    variables: my_query::Variables,
) -> Result<Response<my_query::ResponseData>, Error> {
    let request_body = MyQuery::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/decentraland/marketplace")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<my_query::ResponseData> = res.json().await?;

    Ok(response_body)
}

#[tokio::main]
async fn main() -> AnyhowResult<(), Error> {
    let variables = my_query::Variables;
    let orders_list = perform_my_query(variables)
    .await?
    .data
    .ok_or(anyhow!("Query failed"))?
    .orders;

    let mut categories: Vec<Category> = Vec::new();
    for raw_order in orders_list {
        let order_value = serde_json::to_value(raw_order).expect("Failed converting raw order to json value");
        let order: Order = serde_json::from_value(order_value).expect("Failed converting json value to order object"); 
        categories.push(order.category);
    };

    println!("{:?}", categories);
    Ok(())
}
