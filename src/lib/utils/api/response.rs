use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Fail,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct JSendApiResponse<D>
where
    D: 'static,
{
    pub status: ResponseStatus,
    pub message: Option<String>,
    pub data: Option<D>,
}

impl<D> JSendApiResponse<D> {
    pub fn into_data(self) -> Option<D> {
        self.data
    }
}
