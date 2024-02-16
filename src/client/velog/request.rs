use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct VelogGqlBody {
    pub query: String,
    pub variables: VelogGqlVariablesInput
}

#[derive(Serialize)]
pub(crate) struct VelogGqlVariablesInput {
    pub input: VelogGqlVariables
}

#[derive(Serialize)]
pub(crate) struct VelogGqlVariables {
    pub limit: usize,
    pub offset: usize,
    pub timeframe: String
}