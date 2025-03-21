use {serde::Deserialize, serde_with::serde_as};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealRequest {
    /// Unique ID of the solution (per driver competition), to reveal.
    pub solution_id: u64,
    /// Auction ID in which the specified solution ID is competing.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub auction_id: i64,
}
