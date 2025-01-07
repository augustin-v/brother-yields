use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/query.graphql",
    response_derives = "Debug"
)]
pub struct GetPools;
