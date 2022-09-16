#[doc = "`Coordonate(uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Coordonate {
    pub lat: ethers::core::types::U256,
    pub long: ethers::core::types::U256,
}
#[doc = "`Pixel((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Pixel {
    pub a: Coordonate,
    pub b: Coordonate,
    pub c: Coordonate,
    pub d: Coordonate,
}
