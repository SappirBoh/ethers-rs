use super::{U128, U256, U512, U64};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt, time::Duration};
use strum::{AsRefStr, EnumCount, EnumIter, EnumString, VariantNames};

// compatibility re-export
#[doc(hidden)]
pub use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
#[doc(hidden)]
pub type ParseChainError = TryFromPrimitiveError<Chain>;

// When adding a new chain:
//   1. add new variant to the Chain enum;
//   2. add extra information in the last `impl` block (explorer URLs, block time) when applicable;
//   3. (optional) add aliases:
//     - Strum (in kebab-case): `#[strum(to_string = "<main>", serialize = "<aliasX>", ...)]`
//      `to_string = "<main>"` must be present and will be used in `Display`, `Serialize`
//      and `FromStr`, while `serialize = "<aliasX>"` will be appended to `FromStr`.
//      More info: <https://docs.rs/strum/latest/strum/additional_attributes/index.html#attributes-on-variants>
//     - Serde (in snake_case): `#[serde(alias = "<aliasX>", ...)]`
//      Aliases are appended to the `Deserialize` implementation.
//      More info: <https://serde.rs/variant-attrs.html>
//     - Add a test at the bottom of the file

// We don't derive Serialize because it is manually implemented using AsRef<str> and it would
// break a lot of things since Serialize is `kebab-case` vs Deserialize `snake_case`.
// This means that the Chain type is not "round-trippable", because the Serialize and Deserialize
// implementations do not use the same case style.

/// An Ethereum EIP-155 chain.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,         // AsRef<str>, fmt::Display and serde::Serialize
    VariantNames,     // Chain::VARIANTS
    EnumString,       // FromStr, TryFrom<&str>
    EnumIter,         // Chain::iter
    EnumCount,        // Chain::COUNT
    TryFromPrimitive, // TryFrom<u64>
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "kebab-case")]
#[repr(u64)]
pub enum Chain {
    #[strum(to_string = "mainnet", serialize = "ethlive")]
    #[serde(alias = "ethlive")]
    Mainnet = 1,
    Morden = 2,
    Ropsten = 3,
    Rinkeby = 4,
    Goerli = 5,
    Kovan = 42,
    Holesky = 17000,
    Sepolia = 11155111,
    Hoodi = 560048,

    Optimism = 10,
    OptimismKovan = 69,
    OptimismGoerli = 420,
    OptimismSepolia = 11155420,

    Arbitrum = 42161,
    ArbitrumTestnet = 421611,
    ArbitrumGoerli = 421613,
    ArbitrumSepolia = 421614,
    ArbitrumNova = 42170,

    Cronos = 25,
    CronosTestnet = 338,

    Rsk = 30,

    #[strum(to_string = "bsc", serialize = "binance-smart-chain")]
    #[serde(alias = "bsc")]
    BinanceSmartChain = 56,
    #[strum(to_string = "bsc-testnet", serialize = "binance-smart-chain-testnet")]
    #[serde(alias = "bsc_testnet")]
    BinanceSmartChainTestnet = 97,

    Poa = 99,
    Sokol = 77,

    #[serde(alias = "scroll_sepolia", alias = "scroll_sepolia_testnet")]
    ScrollSepolia = 534351,
    Scroll = 534352,
    ScrollAlphaTestnet = 534353,

    Metis = 1088,

    #[strum(to_string = "xdai", serialize = "gnosis", serialize = "gnosis-chain")]
    #[serde(alias = "xdai", alias = "gnosis", alias = "gnosis_chain")]
    Gnosis = 100,

    Polygon = 137,
    #[strum(to_string = "mumbai", serialize = "polygon-mumbai")]
    #[serde(alias = "mumbai")]
    PolygonMumbai = 80001,
    #[strum(to_string = "amoy", serialize = "polygon-amoy")]
    #[serde(alias = "amoy")]
    PolygonAmoy = 80002,
    #[strum(serialize = "polygon-zkevm", serialize = "zkevm")]
    #[serde(alias = "zkevm", alias = "polygon_zkevm")]
    PolygonZkEvm = 1101,
    #[strum(serialize = "polygon-zkevm-testnet", serialize = "zkevm-testnet")]
    #[serde(alias = "zkevm_testnet", alias = "polygon_zkevm_testnet")]
    PolygonZkEvmTestnet = 1442,

    Fantom = 250,
    FantomTestnet = 4002,

    Moonbeam = 1284,
    MoonbeamDev = 1281,

    Moonriver = 1285,

    Moonbase = 1287,

    Dev = 1337,
    #[strum(to_string = "anvil-hardhat", serialize = "anvil", serialize = "hardhat")]
    #[serde(alias = "anvil", alias = "hardhat")]
    AnvilHardhat = 31337,

    Evmos = 9001,
    EvmosTestnet = 9000,

    Chiado = 10200,

    Oasis = 26863,
    OasisSapphire = 23294,

    Emerald = 42262,
    EmeraldTestnet = 42261,

    FilecoinMainnet = 314,
    FilecoinCalibrationTestnet = 314159,

    Avalanche = 43114,
    #[strum(to_string = "fuji", serialize = "avalanche-fuji")]
    #[serde(alias = "fuji")]
    AvalancheFuji = 43113,

    Celo = 42220,
    CeloAlfajores = 44787,
    CeloBaklava = 62320,

    Aurora = 1313161554,
    AuroraTestnet = 1313161555,

    Canto = 7700,
    CantoTestnet = 740,

    Boba = 288,

    #[strum(to_string = "base")]
    #[serde(alias = "base")]
    Base = 8453,
    #[strum(to_string = "base-goerli")]
    #[serde(alias = "base_goerli")]
    BaseGoerli = 84531,
    #[strum(to_string = "base-sepolia")]
    #[serde(alias = "base_sepolia")]
    BaseSepolia = 84532,

    Blast = 81457,
    BlastSepolia = 168587773,

    Linea = 59144,
    LineaTestnet = 59140,

    #[strum(to_string = "zksync")]
    #[serde(alias = "zksync")]
    ZkSync = 324,
    #[strum(to_string = "zksync-testnet")]
    #[serde(alias = "zksync_testnet")]
    ZkSyncTestnet = 280,

    #[strum(to_string = "mantle")]
    #[serde(alias = "mantle")]
    Mantle = 5000,
    #[strum(to_string = "mantle-testnet")]
    #[serde(alias = "mantle_testnet")]
    MantleTestnet = 5001,

    Viction = 88,

    Zora = 7777777,
    ZoraGoerli = 999,
    ZoraSepolia = 999999999,

    Mode = 34443,
    ModeSepolia = 919,

    Elastos = 20,

    Degen = 666666666,
}

// === impl Chain ===

// This must be implemented manually so we avoid a conflict with `TryFromPrimitive` where it treats
// the `#[default]` attribute as its own `#[num_enum(default)]`
impl Default for Chain {
    fn default() -> Self {
        Self::Mainnet
    }
}

macro_rules! impl_into_numeric {
    ($($ty:ty)+) => {$(
        impl From<Chain> for $ty {
            fn from(chain: Chain) -> Self {
                u64::from(chain).into()
            }
        }
    )+};
}

macro_rules! impl_try_from_numeric {
    ($($native:ty)+ ; $($primitive:ty)*) => {
        $(
            impl TryFrom<$native> for Chain {
                type Error = ParseChainError;

                fn try_from(value: $native) -> Result<Self, Self::Error> {
                    (value as u64).try_into()
                }
            }
        )+

        $(
            impl TryFrom<$primitive> for Chain {
                type Error = ParseChainError;

                fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                    if value.bits() > 64 {
                        // `TryFromPrimitiveError` only has a `number` field which has the same type
                        // as the `#[repr(_)]` attribute on the enum.
                        return Err(ParseChainError { number: value.low_u64() })
                    }
                    value.low_u64().try_into()
                }
            }
        )*
    };
}

impl From<Chain> for u64 {
    fn from(chain: Chain) -> Self {
        chain as u64
    }
}

impl_into_numeric!(u128 U64 U128 U256 U512);

impl TryFrom<U64> for Chain {
    type Error = ParseChainError;

    fn try_from(value: U64) -> Result<Self, Self::Error> {
        value.low_u64().try_into()
    }
}

impl_try_from_numeric!(u8 u16 u32 usize; U128 U256 U512);

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.as_ref())
    }
}

impl Serialize for Chain {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(self.as_ref())
    }
}

// NB: all utility functions *should* be explicitly exhaustive (not use `_` matcher) so we don't
//     forget to update them when adding a new `Chain` variant.
#[allow(clippy::match_like_matches_macro)]
impl Chain {
    /// Returns the chain's average blocktime, if applicable.
    ///
    /// It can be beneficial to know the average blocktime to adjust the polling of an HTTP provider
    /// for example.
    ///
    /// **Note:** this is not an accurate average, but is rather a sensible default derived from
    /// blocktime charts such as [Etherscan's](https://etherscan.com/chart/blocktime)
    /// or [Polygonscan's](https://polygonscan.com/chart/blocktime).
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_core::types::Chain;
    /// use std::time::Duration;
    ///
    /// assert_eq!(
    ///     Chain::Mainnet.average_blocktime_hint(),
    ///     Some(Duration::from_millis(12_000)),
    /// );
    /// assert_eq!(
    ///     Chain::Optimism.average_blocktime_hint(),
    ///     Some(Duration::from_millis(2_000)),
    /// );
    /// ```
    pub const fn average_blocktime_hint(&self) -> Option<Duration> {
        use Chain::*;

        let ms = match self {
            Mainnet => 12_000,
            Arbitrum | ArbitrumTestnet | ArbitrumGoerli | ArbitrumSepolia | ArbitrumNova => 1_300,
            Optimism | OptimismGoerli | OptimismSepolia => 2_000,
            Polygon | PolygonMumbai | PolygonAmoy => 2_100,
            Moonbeam | Moonriver => 12_500,
            BinanceSmartChain | BinanceSmartChainTestnet => 3_000,
            Avalanche | AvalancheFuji => 2_000,
            Fantom | FantomTestnet => 1_200,
            Cronos | CronosTestnet | Canto | CantoTestnet => 5_700,
            Evmos | EvmosTestnet => 1_900,
            Aurora | AuroraTestnet => 1_100,
            Oasis => 5_500,
            OasisSapphire => 5_500,
            Emerald => 6_000,
            Dev | AnvilHardhat => 200,
            Celo | CeloAlfajores | CeloBaklava => 5_000,
            FilecoinCalibrationTestnet | FilecoinMainnet => 30_000,
            Scroll | ScrollSepolia | ScrollAlphaTestnet => 3_000,
            Gnosis | Chiado => 5_000,
            Viction => 2_000,
            Mode | ModeSepolia => 2_000,
            Elastos => 5_000,
            Degen => 622,
            // Explicitly exhaustive. See NB above.
            Morden | Ropsten | Rinkeby | Goerli | Kovan | Sepolia | Holesky | Hoodi | Moonbase |
            MoonbeamDev | OptimismKovan | Poa | Sokol | Rsk | EmeraldTestnet | Boba | Base |
            BaseGoerli | BaseSepolia | Blast | BlastSepolia | ZkSync | ZkSyncTestnet |
            PolygonZkEvm | PolygonZkEvmTestnet | Metis | Linea | LineaTestnet | Mantle |
            MantleTestnet | Zora | ZoraGoerli | ZoraSepolia => return None,
        };

        Some(Duration::from_millis(ms))
    }

    /// Returns whether the chain implements EIP-1559 (with the type 2 EIP-2718 transaction type).
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_core::types::Chain;
    ///
    /// assert!(!Chain::Mainnet.is_legacy());
    /// assert!(Chain::Celo.is_legacy());
    /// ```
    pub const fn is_legacy(&self) -> bool {
        use Chain::*;

        match self {
            // Known legacy chains / non EIP-1559 compliant
            OptimismKovan |
            Fantom |
            FantomTestnet |
            BinanceSmartChain |
            BinanceSmartChainTestnet |
            ArbitrumTestnet |
            Rsk |
            Oasis |
            Emerald |
            EmeraldTestnet |
            Celo |
            CeloAlfajores |
            CeloBaklava |
            Boba |
            ZkSync |
            ZkSyncTestnet |
            Mantle |
            MantleTestnet |
            PolygonZkEvm |
            PolygonZkEvmTestnet |
            Metis |
            Viction |
            Scroll |
            ScrollSepolia |
            Elastos => true,

            // Known EIP-1559 chains
            Mainnet |
            Goerli |
            Sepolia |
            Holesky |
            Hoodi |
            Base |
            BaseGoerli |
            BaseSepolia |
            Blast |
            BlastSepolia |
            Optimism |
            OptimismGoerli |
            OptimismSepolia |
            Polygon |
            PolygonMumbai |
            PolygonAmoy |
            Avalanche |
            AvalancheFuji |
            Arbitrum |
            ArbitrumGoerli |
            ArbitrumSepolia |
            ArbitrumNova |
            FilecoinMainnet |
            Linea |
            LineaTestnet |
            FilecoinCalibrationTestnet |
            Gnosis |
            Chiado |
            Mode |
            ModeSepolia |
            Zora |
            ZoraGoerli |
            ZoraSepolia |
            OasisSapphire |
            Degen => false,

            // Unknown / not applicable, default to false for backwards compatibility
            Dev | AnvilHardhat | Morden | Ropsten | Rinkeby | Cronos | CronosTestnet | Kovan |
            Sokol | Poa | Moonbeam | MoonbeamDev | Moonriver | Moonbase | Evmos |
            EvmosTestnet | Aurora | AuroraTestnet | Canto | CantoTestnet | ScrollAlphaTestnet => {
                false
            }
        }
    }

    /// Returns whether the chain supports the `PUSH0` opcode or not.
    ///
    /// For more information, see EIP-3855:
    /// `<https://eips.ethereum.org/EIPS/eip-3855>`
    pub const fn supports_push0(&self) -> bool {
        match self {
            Chain::Mainnet | Chain::Goerli | Chain::Sepolia | Chain::Gnosis | Chain::Chiado => true,
            _ => false,
        }
    }

    /// Returns the chain's blockchain explorer and its API (Etherscan and Etherscan-like) URLs.
    ///
    /// Returns `(API_URL, BASE_URL)`
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_core::types::Chain;
    ///
    /// assert_eq!(
    ///     Chain::Mainnet.etherscan_urls(),
    ///     Some(("https://api.etherscan.io/api", "https://etherscan.io"))
    /// );
    /// assert_eq!(
    ///     Chain::Avalanche.etherscan_urls(),
    ///     Some(("https://api.snowtrace.io/api", "https://snowtrace.io"))
    /// );
    /// assert_eq!(Chain::AnvilHardhat.etherscan_urls(), None);
    /// ```
    pub const fn etherscan_urls(&self) -> Option<(&'static str, &'static str)> {
        use Chain::*;

        let urls = match self {
            Mainnet => ("https://api.etherscan.io/api", "https://etherscan.io"),
            Ropsten => ("https://api-ropsten.etherscan.io/api", "https://ropsten.etherscan.io"),
            Kovan => ("https://api-kovan.etherscan.io/api", "https://kovan.etherscan.io"),
            Rinkeby => ("https://api-rinkeby.etherscan.io/api", "https://rinkeby.etherscan.io"),
            Goerli => ("https://api-goerli.etherscan.io/api", "https://goerli.etherscan.io"),
            Sepolia => ("https://api-sepolia.etherscan.io/api", "https://sepolia.etherscan.io"),
            Holesky => ("https://api-holesky.etherscan.io/api", "https://holesky.etherscan.io"),
            Hoodi => ("https://api-hoodi.etherscan.io/api", "https://hoodi.etherscan.io"),

            Polygon => ("https://api.polygonscan.com/api", "https://polygonscan.com"),
            PolygonMumbai => {
                ("https://api-testnet.polygonscan.com/api", "https://mumbai.polygonscan.com")
            }
            PolygonAmoy => ("https://rpc-amoy.polygon.technology", "https://www.oklink.com/amoy"),

            PolygonZkEvm => {
                ("https://api-zkevm.polygonscan.com/api", "https://zkevm.polygonscan.com")
            }
            PolygonZkEvmTestnet => (
                "https://api-testnet-zkevm.polygonscan.com/api",
                "https://testnet-zkevm.polygonscan.com",
            ),

            Avalanche => ("https://api.snowtrace.io/api", "https://snowtrace.io"),
            AvalancheFuji => {
                ("https://api-testnet.snowtrace.io/api", "https://testnet.snowtrace.io")
            }

            Optimism => {
                ("https://api-optimistic.etherscan.io/api", "https://optimistic.etherscan.io")
            }
            OptimismGoerli => (
                "https://api-goerli-optimistic.etherscan.io/api",
                "https://goerli-optimism.etherscan.io",
            ),
            OptimismKovan => (
                "https://api-kovan-optimistic.etherscan.io/api",
                "https://kovan-optimistic.etherscan.io",
            ),
            OptimismSepolia => (
                "https://api-sepolia-optimistic.etherscan.io/api",
                "https://sepolia-optimism.etherscan.io",
            ),

            Fantom => ("https://api.ftmscan.com/api", "https://ftmscan.com"),
            FantomTestnet => ("https://api-testnet.ftmscan.com/api", "https://testnet.ftmscan.com"),

            BinanceSmartChain => ("https://api.bscscan.com/api", "https://bscscan.com"),
            BinanceSmartChainTestnet => {
                ("https://api-testnet.bscscan.com/api", "https://testnet.bscscan.com")
            }

            Arbitrum => ("https://api.arbiscan.io/api", "https://arbiscan.io"),
            ArbitrumTestnet => {
                ("https://api-testnet.arbiscan.io/api", "https://testnet.arbiscan.io")
            }
            ArbitrumGoerli => ("https://api-goerli.arbiscan.io/api", "https://goerli.arbiscan.io"),
            ArbitrumSepolia => {
                ("https://api-sepolia.arbiscan.io/api", "https://sepolia.arbiscan.io")
            }
            ArbitrumNova => ("https://api-nova.arbiscan.io/api", "https://nova.arbiscan.io/"),

            Cronos => ("https://api.cronoscan.com/api", "https://cronoscan.com"),
            CronosTestnet => {
                ("https://api-testnet.cronoscan.com/api", "https://testnet.cronoscan.com")
            }

            Moonbeam => ("https://api-moonbeam.moonscan.io/api", "https://moonbeam.moonscan.io/"),
            Moonbase => ("https://api-moonbase.moonscan.io/api", "https://moonbase.moonscan.io/"),
            Moonriver => ("https://api-moonriver.moonscan.io/api", "https://moonriver.moonscan.io"),

            Gnosis => ("https://api.gnosisscan.io/api", "https://gnosisscan.io"),

            Scroll => ("https://api.scrollscan.com/api", "https://scrollscan.com"),
            ScrollSepolia => {
                ("https://api-sepolia.scrollscan.com/api", "https://sepolia.scrollscan.com")
            }
            ScrollAlphaTestnet => {
                ("https://alpha-blockscout.scroll.io/api", "https://alpha-blockscout.scroll.io/")
            }

            Metis => (
                "https://api.routescan.io/v2/network/mainnet/evm/1088/etherscan/api",
                "https://explorer.metis.io/",
            ),

            Chiado => {
                ("https://blockscout.chiadochain.net/api", "https://blockscout.chiadochain.net")
            }

            FilecoinCalibrationTestnet => (
                "https://api.calibration.node.glif.io/rpc/v1",
                "https://calibration.filfox.info/en",
            ),

            Sokol => ("https://blockscout.com/poa/sokol/api", "https://blockscout.com/poa/sokol"),

            Poa => ("https://blockscout.com/poa/core/api", "https://blockscout.com/poa/core"),

            Rsk => ("https://blockscout.com/rsk/mainnet/api", "https://blockscout.com/rsk/mainnet"),

            Oasis => ("https://scan.oasischain.io/api", "https://scan.oasischain.io/"),

            OasisSapphire => ("https://explorer.oasis.io/mainnet/sapphire/api", "https://explorer.oasis.io/mainnet/sapphire/"),

            Emerald => {
                ("https://explorer.emerald.oasis.dev/api", "https://explorer.emerald.oasis.dev/")
            }
            EmeraldTestnet => (
                "https://testnet.explorer.emerald.oasis.dev/api",
                "https://testnet.explorer.emerald.oasis.dev/",
            ),

            Aurora => ("https://api.aurorascan.dev/api", "https://aurorascan.dev"),
            AuroraTestnet => {
                ("https://testnet.aurorascan.dev/api", "https://testnet.aurorascan.dev")
            }

            Evmos => ("https://evm.evmos.org/api", "https://evm.evmos.org/"),
            EvmosTestnet => ("https://evm.evmos.dev/api", "https://evm.evmos.dev/"),

            Celo => ("https://explorer.celo.org/mainnet/api", "https://explorer.celo.org/mainnet"),
            CeloAlfajores => {
                ("https://explorer.celo.org/alfajores/api", "https://explorer.celo.org/alfajores")
            }
            CeloBaklava => {
                ("https://explorer.celo.org/baklava/api", "https://explorer.celo.org/baklava")
            }

            Canto => ("https://evm.explorer.canto.io/api", "https://evm.explorer.canto.io/"),
            CantoTestnet => (
                "https://testnet-explorer.canto.neobase.one/api",
                "https://testnet-explorer.canto.neobase.one/",
            ),

            Boba => ("https://api.bobascan.com/api", "https://bobascan.com"),

            Base => ("https://api.basescan.org/api", "https://basescan.org"),

            BaseGoerli => ("https://api-goerli.basescan.org/api", "https://goerli.basescan.org"),
            BaseSepolia => ("https://api-sepolia.basescan.org/api", "https://sepolia.basescan.org"),

            Blast => (
                "https://api.routescan.io/v2/network/mainnet/evm/81457/etherscan",
                "https://blastscan.io",
            ),
            BlastSepolia => (
                "https://api.routescan.io/v2/network/testnet/evm/168587773/etherscan",
                "https://testnet.blastscan.io",
            ),

            ZkSync => {
                ("https://zksync2-mainnet-explorer.zksync.io/", "https://explorer.zksync.io/")
            }
            ZkSyncTestnet => (
                "https://zksync2-testnet-explorer.zksync.dev/",
                "https://goerli.explorer.zksync.io/",
            ),
            Linea => ("https://api.lineascan.build/api", "https://lineascan.build/"),
            LineaTestnet => {
                ("https://explorer.goerli.linea.build/api", "https://explorer.goerli.linea.build/")
            }
            Mantle => ("https://explorer.mantle.xyz/api", "https://explorer.mantle.xyz"),
            MantleTestnet => {
                ("https://explorer.testnet.mantle.xyz/api", "https://explorer.testnet.mantle.xyz")
            }

            Zora => ("https://explorer.zora.energy/api", "https://explorer.zora.energy"),
            ZoraGoerli => {
                ("https://testnet.explorer.zora.energy/api", "https://testnet.explorer.zora.energy")
            }
            ZoraSepolia => {
                ("https://sepolia.explorer.zora.energy/api", "https://sepolia.explorer.zora.energy")
            }

            AnvilHardhat | Dev | Morden | MoonbeamDev | FilecoinMainnet => {
                // this is explicitly exhaustive so we don't forget to add new urls when adding a
                // new chain
                return None;
            }
            Viction => ("https://www.vicscan.xyz/api", "https://www.vicscan.xyz"),

            Mode => ("https://explorer.mode.network/api", "https://explorer.mode.network"),
            ModeSepolia => (
                "https://sepolia.explorer.mode.network/api",
                "https://sepolia.explorer.mode.network",
            ),
            Elastos => ("https://api.elastos.io/eth", "https://esc.elastos.io/"),
            Degen => ("https://explorer.degen.tips/api", "https://explorer.degen.tips"),
        };

        Some(urls)
    }

    /// Returns the chain's blockchain explorer's API key environment variable's default name.
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_core::types::Chain;
    ///
    /// assert_eq!(Chain::Mainnet.etherscan_api_key_name(), Some("ETHERSCAN_API_KEY"));
    /// assert_eq!(Chain::AnvilHardhat.etherscan_api_key_name(), None);
    /// ```
    pub const fn etherscan_api_key_name(&self) -> Option<&'static str> {
        use Chain::*;

        let api_key_name = match self {
            Mainnet |
            Morden |
            Ropsten |
            Kovan |
            Rinkeby |
            Goerli |
            Holesky |
            Hoodi |
            Optimism |
            OptimismGoerli |
            OptimismKovan |
            OptimismSepolia |
            BinanceSmartChain |
            BinanceSmartChainTestnet |
            Arbitrum |
            ArbitrumTestnet |
            ArbitrumGoerli |
            ArbitrumSepolia |
            ArbitrumNova |
            Cronos |
            CronosTestnet |
            Aurora |
            AuroraTestnet |
            Celo |
            CeloAlfajores |
            CeloBaklava |
            Base |
            Linea |
            Mantle |
            MantleTestnet |
            BaseGoerli |
            BaseSepolia |
            Blast |
            BlastSepolia |
            Gnosis |
            Scroll |
            ScrollSepolia => "ETHERSCAN_API_KEY",

            Avalanche | AvalancheFuji => "SNOWTRACE_API_KEY",

            Polygon | PolygonMumbai | PolygonZkEvm | PolygonZkEvmTestnet | PolygonAmoy => {
                "POLYGONSCAN_API_KEY"
            }

            Fantom | FantomTestnet => "FTMSCAN_API_KEY",

            Moonbeam | Moonbase | MoonbeamDev | Moonriver => "MOONSCAN_API_KEY",

            Canto | CantoTestnet | Zora | ZoraGoerli | ZoraSepolia | Mode | ModeSepolia => {
                "BLOCKSCOUT_API_KEY"
            }

            Boba => "BOBASCAN_API_KEY",

            // Explicitly exhaustive. See NB above.
            ScrollAlphaTestnet |
            Metis |
            Chiado |
            Sepolia |
            Rsk |
            Sokol |
            Poa |
            Oasis |
            OasisSapphire |
            Emerald |
            EmeraldTestnet |
            Evmos |
            EvmosTestnet |
            AnvilHardhat |
            Dev |
            ZkSync |
            ZkSyncTestnet |
            FilecoinMainnet |
            LineaTestnet |
            Viction |
            FilecoinCalibrationTestnet |
            Elastos |
            Degen => return None,
        };

        Some(api_key_name)
    }

    /// Returns the chain's blockchain explorer's API key, from the environment variable with the
    /// name specified in [`etherscan_api_key_name`](Chain::etherscan_api_key_name).
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_core::types::Chain;
    ///
    /// let chain = Chain::Mainnet;
    /// std::env::set_var(chain.etherscan_api_key_name().unwrap(), "KEY");
    /// assert_eq!(chain.etherscan_api_key().as_deref(), Some("KEY"));
    /// ```
    pub fn etherscan_api_key(&self) -> Option<String> {
        self.etherscan_api_key_name().and_then(|name| std::env::var(name).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn default() {
        assert_eq!(serde_json::to_string(&Chain::default()).unwrap(), "\"mainnet\"");
    }

    #[test]
    fn enum_iter() {
        assert_eq!(Chain::COUNT, Chain::iter().size_hint().0);
    }

    #[test]
    fn roundtrip_string() {
        for chain in Chain::iter() {
            let chain_string = chain.to_string();
            assert_eq!(chain_string, format!("{chain}"));
            assert_eq!(chain_string.as_str(), chain.as_ref());
            assert_eq!(serde_json::to_string(&chain).unwrap(), format!("\"{chain_string}\""));

            assert_eq!(chain_string.parse::<Chain>().unwrap(), chain);
        }
    }

    #[test]
    fn roundtrip_serde() {
        for chain in Chain::iter() {
            let chain_string = serde_json::to_string(&chain).unwrap();
            let chain_string = chain_string.replace('-', "_");
            assert_eq!(serde_json::from_str::<'_, Chain>(&chain_string).unwrap(), chain);
        }
    }

    #[test]
    fn aliases() {
        use Chain::*;

        // kebab-case
        const ALIASES: &[(Chain, &[&str])] = &[
            (Mainnet, &["ethlive"]),
            (BinanceSmartChain, &["bsc", "binance-smart-chain"]),
            (BinanceSmartChainTestnet, &["bsc-testnet", "binance-smart-chain-testnet"]),
            (Gnosis, &["gnosis", "gnosis-chain"]),
            (PolygonMumbai, &["mumbai"]),
            (PolygonZkEvm, &["zkevm", "polygon-zkevm"]),
            (PolygonZkEvmTestnet, &["zkevm-testnet", "polygon-zkevm-testnet"]),
            (AnvilHardhat, &["anvil", "hardhat"]),
            (AvalancheFuji, &["fuji"]),
            (ZkSync, &["zksync"]),
            (Mantle, &["mantle"]),
            (MantleTestnet, &["mantle-testnet"]),
            (Viction, &["viction"]),
            (Base, &["base"]),
            (BaseGoerli, &["base-goerli"]),
            (BaseSepolia, &["base-sepolia"]),
        ];

        for &(chain, aliases) in ALIASES {
            for &alias in aliases {
                assert_eq!(alias.parse::<Chain>().unwrap(), chain);
                let s = alias.to_string().replace('-', "_");
                assert_eq!(serde_json::from_str::<Chain>(&format!("\"{s}\"")).unwrap(), chain);
            }
        }
    }

    #[test]
    fn serde_to_string_match() {
        for chain in Chain::iter() {
            let chain_serde = serde_json::to_string(&chain).unwrap();
            let chain_string = format!("\"{chain}\"");
            assert_eq!(chain_serde, chain_string);
        }
    }
}
