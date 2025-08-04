use crate::grand_plan::meme_economy::meme_lord::meme_lord_struct::MemeLord;
use crate::grand_plan::solana_integration::compute_marketplace::compute_marketplace_struct::ComputeMarketplace;
use crate::grand_plan::solana_integration::market_maker::market_maker_struct::MarketMaker;
use crate::grand_plan::meme_economy::media_campaign::media_campaign_struct::MediaCampaign;

pub struct EconomicSocialLayerComponents {
    pub meme_lord: MemeLord,
    pub compute_marketplace: ComputeMarketplace,
    pub market_maker: MarketMaker,
    pub media_campaign: MediaCampaign,
}

pub fn initialize_economic_social_layer_components() -> EconomicSocialLayerComponents {
    let meme_lord = MemeLord::new("genesis_lord".to_string(), 10000.0);
    let compute_marketplace = ComputeMarketplace::new();
    let market_maker = MarketMaker::new();
    let media_campaign = MediaCampaign::new("genesis_campaign".to_string(), vec!["Eb".to_string()], "early_adopters".to_string(), 500.0);

    EconomicSocialLayerComponents {
        meme_lord,
        compute_marketplace,
        market_maker,
        media_campaign,
    }
}
