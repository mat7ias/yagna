use ya_market_decentralized::testing::proposal_util::exchange_draft_proposals;
use ya_market_decentralized::testing::{MarketsNetwork, OwnerType};

use ya_client::model::market::proposal::State;

#[cfg_attr(not(feature = "market-test-suite"), ignore)]
#[actix_rt::test]
async fn test_get_proposal_rest() -> Result<(), anyhow::Error> {
    let network = MarketsNetwork::new("test_get_proposal_rest")
        .await
        .add_market_instance("Requestor1")
        .await?
        .add_market_instance("Provider1")
        .await?;

    let req_market = network.get_market("Requestor1");
    let req_id = network.get_default_id("Requestor1");
    let prov_market = network.get_market("Provider1");
    let prov_id = network.get_default_id("Provider1");

    // Requestor side
    let proposal_id = exchange_draft_proposals(&network, "Requestor1", "Provider1").await?;
    let result = req_market.get_proposal(&proposal_id, &req_id).await;

    assert!(result.is_ok());
    let proposal = result.unwrap();

    assert_eq!(proposal.state()?, &State::Draft);
    assert_eq!(proposal.proposal_id()?, &proposal_id.to_string());
    assert_eq!(proposal.issuer_id()?, &prov_id.identity.to_string());
    assert!(proposal.prev_proposal_id().is_ok());

    // Provider side
    let proposal_id = proposal_id.translate(OwnerType::Provider);
    let result = prov_market.get_proposal(&proposal_id, &req_id).await;

    assert!(result.is_ok());
    let proposal = result.unwrap();

    assert_eq!(proposal.state()?, &State::Draft);
    assert_eq!(proposal.proposal_id()?, &proposal_id.to_string());
    assert_eq!(proposal.issuer_id()?, &prov_id.identity.to_string());
    assert!(proposal.prev_proposal_id().is_ok());
    Ok(())
}
