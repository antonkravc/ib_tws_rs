use std::convert::From;
use std::fmt;
use std::i32;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataTypeMsg {
    pub req_id: i32,
    pub market_data_type: MarketDataType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum MarketDataType {
    REALTIME = 1,
    FROZEN = 2,
    DELAYED = 3,
    DELAYED_FROZEN = 4,
}

impl TryFrom<i32> for MarketDataType {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::REALTIME as i32 => Ok(Self::REALTIME),
            x if x == Self::FROZEN as i32 => Ok(Self::FROZEN),
            x if x == Self::DELAYED as i32 => Ok(Self::DELAYED),
            x if x == Self::DELAYED_FROZEN as i32 => Ok(Self::DELAYED_FROZEN),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MarketDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            MarketDataType::REALTIME => "Real-Time",
            MarketDataType::FROZEN => "Frozen",
            MarketDataType::DELAYED => "Delayed",
            MarketDataType::DELAYED_FROZEN => "Delayed-Frozen",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthMktDataDescription {
    pub exchange: String,
    pub sec_type: String,
    pub listing_exch: String,
    pub service_data_type: String,
    pub agg_group: i32,
}

impl DepthMktDataDescription {
    pub fn new(
        exchange: &str,
        sec_type: &str,
        listing_exch: &str,
        service_data_type: &str,
        agg_group: i32,
    ) -> Self {
        DepthMktDataDescription {
            exchange: exchange.to_string(),
            sec_type: sec_type.to_string(),
            listing_exch: listing_exch.to_string(),
            service_data_type: service_data_type.to_string(),
            agg_group,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Decimal,
    pub count: i32,
    pub wap: Decimal,
}

impl baseline::bar::OHLC for Bar {
    type Type = f64;

    fn open(&self) -> Self::Type {
        self.open
    }

    fn high(&self) -> Self::Type {
        self.high
    }

    fn low(&self) -> Self::Type {
        self.low
    }

    fn close(&self) -> Self::Type {
        self.close
    }
}

impl baseline::bar::Volume for Bar {
    type Type = Decimal;

    fn volume(&self) -> Self::Type {
        self.volume
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalTick {
    pub time: i64,
    pub price: f64,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalTickBidAsk {
    pub time: i64,
    pub mask: i32,
    pub price_bid: f64,
    pub price_ask: f64,
    pub size_bid: i64,
    pub size_ask: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalTickLast {
    pub time: i64,
    pub mask: i32,
    pub price: f64,
    pub size: i64,
    pub exchange: String,
    pub special_conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramEntry {
    pub price: f64,
    pub size: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TickAttr {
    pub can_auto_execute: bool,
    pub past_limit: bool,
    pub pre_open: bool,
    pub unreported: bool,
    pub bid_past_low: bool,
    pub ask_past_high: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickByTick {
    pub tick_type: TickByTickType,
    // 0 - None, 1 - Last, 2 - AllLast, 3 -BidAsk, 4 - MidPoint
    pub time: i64,
    pub price: f64,
    pub size: i64,
    pub attribs: TickAttr,
    pub exchange: String,
    pub special_conditions: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub ask_price: f64,
    pub ask_size: i64,
    pub mid_point: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum TickByTickType {
    Last = 1,
    AllLast = 2,
    BidAsk = 3,
    MidPoint = 4,
}

impl TryFrom<i32> for TickByTickType {
    type Error = ();
    fn try_from(code: i32) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(Self::Last),
            2 => Ok(Self::AllLast),
            3 => Ok(Self::BidAsk),
            4 => Ok(Self::MidPoint),
            _ => Err(())
        }
    }
}


impl std::fmt::Display for TickByTickType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum TickType {
    BID_SIZE = 0,
    BID = 1,
    ASK = 2,
    ASK_SIZE = 3,
    LAST = 4,
    LAST_SIZE = 5,
    HIGH = 6,
    LOW = 7,
    VOLUME = 8,
    CLOSE = 9,
    BID_OPTION = 10,
    ASK_OPTION = 11,
    LAST_OPTION = 12,
    MODEL_OPTION = 13,
    OPEN = 14,
    LOW_13_WEEK = 15,
    HIGH_13_WEEK = 16,
    LOW_26_WEEK = 17,
    HIGH_26_WEEK = 18,
    LOW_52_WEEK = 19,
    HIGH_52_WEEK = 20,
    AVG_VOLUME = 21,
    OPEN_INTEREST = 22,
    OPTION_HISTORICAL_VOL = 23,
    OPTION_IMPLIED_VOL = 24,
    OPTION_BID_EXCH = 25,
    OPTION_ASK_EXCH = 26,
    OPTION_CALL_OPEN_INTEREST = 27,
    OPTION_PUT_OPEN_INTEREST = 28,
    OPTION_CALL_VOLUME = 29,
    OPTION_PUT_VOLUME = 30,
    INDEX_FUTURE_PREMIUM = 31,
    BID_EXCH = 32,
    ASK_EXCH = 33,
    AUCTION_VOLUME = 34,
    AUCTION_PRICE = 35,
    AUCTION_IMBALANCE = 36,
    MARK_PRICE = 37,
    BID_EFP_COMPUTATION = 38,
    ASK_EFP_COMPUTATION = 39,
    LAST_EFP_COMPUTATION = 40,
    OPEN_EFP_COMPUTATION = 41,
    HIGH_EFP_COMPUTATION = 42,
    LOW_EFP_COMPUTATION = 43,
    CLOSE_EFP_COMPUTATION = 44,
    LAST_TIMESTAMP = 45,
    SHORTABLE = 46,
    FUNDAMENTAL_RATIOS = 47,
    RT_VOLUME = 48,
    HALTED = 49,
    BID_YIELD = 50,
    ASK_YIELD = 51,
    LAST_YIELD = 52,
    CUST_OPTION_COMPUTATION = 53,
    TRADE_COUNT = 54,
    TRADE_RATE = 55,
    VOLUME_RATE = 56,
    LAST_RTH_TRADE = 57,
    RT_HISTORICAL_VOL = 58,
    IB_DIVIDENDS = 59,
    BOND_FACTOR_MULTIPLIER = 60,
    REGULATORY_IMBALANCE = 61,
    NEWS_TICK = 62,
    SHORT_TERM_VOLUME_3_MIN = 63,
    SHORT_TERM_VOLUME_5_MIN = 64,
    SHORT_TERM_VOLUME_10_MIN = 65,
    DELAYED_BID = 66,
    DELAYED_ASK = 67,
    DELAYED_LAST = 68,
    DELAYED_BID_SIZE = 69,
    DELAYED_ASK_SIZE = 70,
    DELAYED_LAST_SIZE = 71,
    DELAYED_HIGH = 72,
    DELAYED_LOW = 73,
    DELAYED_VOLUME = 74,
    DELAYED_CLOSE = 75,
    DELAYED_OPEN = 76,
    RT_TRD_VOLUME = 77,
    CREDITMAN_MARK_PRICE = 78,
    CREDITMAN_SLOW_MARK_PRICE = 79,
    DELAYED_BID_OPTION = 80,
    DELAYED_ASK_OPTION = 81,
    DELAYED_LAST_OPTION = 82,
    DELAYED_MODEL_OPTION = 83,
    LAST_EXCH = 84,
    LAST_REG_TIME = 85,
    FUTURES_OPEN_INTEREST = 86,
    AVG_OPT_VOLUME = 87,
    DELAYED_LAST_TIMESTAMP = 88,
    SHORTABLE_SHARES = 89,
    DELAYED_HALTED = 90,
    REUTERS_2_MUTUAL_FUNDS = 91,
    ETF_NAV_CLOSE = 92,
    ETF_NAV_PRIOR_CLOSE = 93,
    ETF_NAV_BID = 94,
    ETF_NAV_ASK = 95,
    ETF_NAV_LAST = 96,
    ETF_FROZEN_NAV_LAST = 97,
    ETF_NAV_HIGH = 98,
    ETF_NAV_LOW = 99,
    SOCIAL_MARKET_ANALYTICS = 100,
    ESTIMATED_IPO_MIDPOINT = 101,
    FINAL_IPO_LAST = 102,
    UNKNOWN = i32::MAX,
}

impl Default for TickType {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl fmt::Display for TickType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TickType::BID_SIZE => "bidSize",
            TickType::BID => "bidPrice",
            TickType::ASK => "askPrice",
            TickType::ASK_SIZE => "askSize",
            TickType::LAST => "lastPrice",
            TickType::LAST_SIZE => "lastSize",
            TickType::HIGH => "high",
            TickType::LOW => "low",
            TickType::VOLUME => "volume",
            TickType::CLOSE => "close",
            TickType::BID_OPTION => "bidOptComp",
            TickType::ASK_OPTION => "askOptComp",
            TickType::LAST_OPTION => "lastOptComp",
            TickType::MODEL_OPTION => "modelOptComp",
            TickType::OPEN => "open",
            TickType::LOW_13_WEEK => "13WeekLow",
            TickType::HIGH_13_WEEK => "13WeekHigh",
            TickType::LOW_26_WEEK => "26WeekLow",
            TickType::HIGH_26_WEEK => "26WeekHigh",
            TickType::LOW_52_WEEK => "52WeekLow",
            TickType::HIGH_52_WEEK => "52WeekHigh",
            TickType::AVG_VOLUME => "AvgVolume",
            TickType::OPEN_INTEREST => "OpenInterest",
            TickType::OPTION_HISTORICAL_VOL => "OptionHistoricalVolatility",
            TickType::OPTION_IMPLIED_VOL => "OptionImpliedVolatility",
            TickType::OPTION_BID_EXCH => "OptionBidExchStr",
            TickType::OPTION_ASK_EXCH => "OptionAskExchStr",
            TickType::OPTION_CALL_OPEN_INTEREST => "OptionCallOpenInterest",
            TickType::OPTION_PUT_OPEN_INTEREST => "OptionPutOpenInterest",
            TickType::OPTION_CALL_VOLUME => "OptionCallVolume",
            TickType::OPTION_PUT_VOLUME => "OptionPutVolume",
            TickType::INDEX_FUTURE_PREMIUM => "IndexFuturePremium",
            TickType::BID_EXCH => "bidExch",
            TickType::ASK_EXCH => "askExch",
            TickType::AUCTION_VOLUME => "auctionVolume",
            TickType::AUCTION_PRICE => "auctionPrice",
            TickType::AUCTION_IMBALANCE => "auctionImbalance",
            TickType::MARK_PRICE => "markPrice",
            TickType::BID_EFP_COMPUTATION => "bidEFP",
            TickType::ASK_EFP_COMPUTATION => "askEFP",
            TickType::LAST_EFP_COMPUTATION => "lastEFP",
            TickType::OPEN_EFP_COMPUTATION => "openEFP",
            TickType::HIGH_EFP_COMPUTATION => "highEFP",
            TickType::LOW_EFP_COMPUTATION => "lowEFP",
            TickType::CLOSE_EFP_COMPUTATION => "closeEFP",
            TickType::LAST_TIMESTAMP => "lastTimestamp",
            TickType::SHORTABLE => "shortable",
            TickType::FUNDAMENTAL_RATIOS => "fundamentals",
            TickType::RT_VOLUME => "RTVolume",
            TickType::HALTED => "halted",
            TickType::BID_YIELD => "bidYield",
            TickType::ASK_YIELD => "askYield",
            TickType::LAST_YIELD => "lastYield",
            TickType::CUST_OPTION_COMPUTATION => "custOptComp",
            TickType::TRADE_COUNT => "trades",
            TickType::TRADE_RATE => "trades/min",
            TickType::VOLUME_RATE => "volume/min",
            TickType::LAST_RTH_TRADE => "lastRTHTrade",
            TickType::RT_HISTORICAL_VOL => "RTHistoricalVol",
            TickType::IB_DIVIDENDS => "IBDividends",
            TickType::BOND_FACTOR_MULTIPLIER => "bondFactorMultiplier",
            TickType::REGULATORY_IMBALANCE => "regulatoryImbalance",
            TickType::NEWS_TICK => "newsTick",
            TickType::SHORT_TERM_VOLUME_3_MIN => "shortTermVolume3Min",
            TickType::SHORT_TERM_VOLUME_5_MIN => "shortTermVolume5Min",
            TickType::SHORT_TERM_VOLUME_10_MIN => "shortTermVolume10Min",
            TickType::DELAYED_BID => "delayedBid",
            TickType::DELAYED_ASK => "delayedAsk",
            TickType::DELAYED_LAST => "delayedLast",
            TickType::DELAYED_BID_SIZE => "delayedBidSize",
            TickType::DELAYED_ASK_SIZE => "delayedAskSize",
            TickType::DELAYED_LAST_SIZE => "delayedLastSize",
            TickType::DELAYED_HIGH => "delayedHigh",
            TickType::DELAYED_LOW => "delayedLow",
            TickType::DELAYED_VOLUME => "delayedVolume",
            TickType::DELAYED_CLOSE => "delayedClose",
            TickType::DELAYED_OPEN => "delayedOpen",
            TickType::RT_TRD_VOLUME => "rtTrdVolume",
            TickType::CREDITMAN_MARK_PRICE => "creditmanMarkPrice",
            TickType::CREDITMAN_SLOW_MARK_PRICE => "creditmanSlowMarkPrice",
            TickType::DELAYED_BID_OPTION => "delayedBidOptComp",
            TickType::DELAYED_ASK_OPTION => "delayedAskOptComp",
            TickType::DELAYED_LAST_OPTION => "delayedLastOptComp",
            TickType::DELAYED_MODEL_OPTION => "delayedModelOptComp",
            TickType::LAST_EXCH => "lastExchange",
            TickType::LAST_REG_TIME => "lastRegTime",
            TickType::FUTURES_OPEN_INTEREST => "futuresOpenInterest",
            TickType::AVG_OPT_VOLUME => "avgOptVolume",
            TickType::DELAYED_LAST_TIMESTAMP => "delayedLastTimestamp",
            TickType::UNKNOWN => "unknown",
            TickType::SHORTABLE_SHARES => "shortableShares",
            TickType::DELAYED_HALTED => "delayedHalted",
            TickType::REUTERS_2_MUTUAL_FUNDS => "reuters2MutualFunds",
            TickType::ETF_NAV_CLOSE => "etfNavClose",
            TickType::ETF_NAV_PRIOR_CLOSE => "etfNavPriorClose",
            TickType::ETF_NAV_BID => "etfNavBid",
            TickType::ETF_NAV_ASK => "etfNavAsk",
            TickType::ETF_NAV_LAST => "etfNavLast",
            TickType::ETF_FROZEN_NAV_LAST => "etfFrozenNavLast",
            TickType::ETF_NAV_HIGH => "etfNavHigh",
            TickType::ETF_NAV_LOW => "etfNavLow",
            TickType::SOCIAL_MARKET_ANALYTICS => "socialMarketAnalytics",
            TickType::ESTIMATED_IPO_MIDPOINT => "estimatedIPOMidpoint",
            TickType::FINAL_IPO_LAST => "finalIPOLast",
        };
        write!(f, "{}", s)
    }
}

impl From<i32> for TickType {
    fn from(code: i32) -> TickType {
        match code {
            0 => TickType::BID_SIZE,
            1 => TickType::BID,
            2 => TickType::ASK,
            3 => TickType::ASK_SIZE,
            4 => TickType::LAST,
            5 => TickType::LAST_SIZE,
            6 => TickType::HIGH,
            7 => TickType::LOW,
            8 => TickType::VOLUME,
            9 => TickType::CLOSE,
            10 => TickType::BID_OPTION,
            11 => TickType::ASK_OPTION,
            12 => TickType::LAST_OPTION,
            13 => TickType::MODEL_OPTION,
            14 => TickType::OPEN,
            15 => TickType::LOW_13_WEEK,
            16 => TickType::HIGH_13_WEEK,
            17 => TickType::LOW_26_WEEK,
            18 => TickType::HIGH_26_WEEK,
            19 => TickType::LOW_52_WEEK,
            20 => TickType::HIGH_52_WEEK,
            21 => TickType::AVG_VOLUME,
            22 => TickType::OPEN_INTEREST,
            23 => TickType::OPTION_HISTORICAL_VOL,
            24 => TickType::OPTION_IMPLIED_VOL,
            25 => TickType::OPTION_BID_EXCH,
            26 => TickType::OPTION_ASK_EXCH,
            27 => TickType::OPTION_CALL_OPEN_INTEREST,
            28 => TickType::OPTION_PUT_OPEN_INTEREST,
            29 => TickType::OPTION_CALL_VOLUME,
            30 => TickType::OPTION_PUT_VOLUME,
            31 => TickType::INDEX_FUTURE_PREMIUM,
            32 => TickType::BID_EXCH,
            33 => TickType::ASK_EXCH,
            34 => TickType::AUCTION_VOLUME,
            35 => TickType::AUCTION_PRICE,
            36 => TickType::AUCTION_IMBALANCE,
            37 => TickType::MARK_PRICE,
            38 => TickType::BID_EFP_COMPUTATION,
            39 => TickType::ASK_EFP_COMPUTATION,
            40 => TickType::LAST_EFP_COMPUTATION,
            41 => TickType::OPEN_EFP_COMPUTATION,
            42 => TickType::HIGH_EFP_COMPUTATION,
            43 => TickType::LOW_EFP_COMPUTATION,
            44 => TickType::CLOSE_EFP_COMPUTATION,
            45 => TickType::LAST_TIMESTAMP,
            46 => TickType::SHORTABLE,
            47 => TickType::FUNDAMENTAL_RATIOS,
            48 => TickType::RT_VOLUME,
            49 => TickType::HALTED,
            50 => TickType::BID_YIELD,
            51 => TickType::ASK_YIELD,
            52 => TickType::LAST_YIELD,
            53 => TickType::CUST_OPTION_COMPUTATION,
            54 => TickType::TRADE_COUNT,
            55 => TickType::TRADE_RATE,
            56 => TickType::VOLUME_RATE,
            57 => TickType::LAST_RTH_TRADE,
            58 => TickType::RT_HISTORICAL_VOL,
            59 => TickType::IB_DIVIDENDS,
            60 => TickType::BOND_FACTOR_MULTIPLIER,
            61 => TickType::REGULATORY_IMBALANCE,
            62 => TickType::NEWS_TICK,
            63 => TickType::SHORT_TERM_VOLUME_3_MIN,
            64 => TickType::SHORT_TERM_VOLUME_5_MIN,
            65 => TickType::SHORT_TERM_VOLUME_10_MIN,
            66 => TickType::DELAYED_BID,
            67 => TickType::DELAYED_ASK,
            68 => TickType::DELAYED_LAST,
            69 => TickType::DELAYED_BID_SIZE,
            70 => TickType::DELAYED_ASK_SIZE,
            71 => TickType::DELAYED_LAST_SIZE,
            72 => TickType::DELAYED_HIGH,
            73 => TickType::DELAYED_LOW,
            74 => TickType::DELAYED_VOLUME,
            75 => TickType::DELAYED_CLOSE,
            76 => TickType::DELAYED_OPEN,
            77 => TickType::RT_TRD_VOLUME,
            78 => TickType::CREDITMAN_MARK_PRICE,
            79 => TickType::CREDITMAN_SLOW_MARK_PRICE,
            80 => TickType::DELAYED_BID_OPTION,
            81 => TickType::DELAYED_ASK_OPTION,
            82 => TickType::DELAYED_LAST_OPTION,
            83 => TickType::DELAYED_MODEL_OPTION,
            84 => TickType::LAST_EXCH,
            85 => TickType::LAST_REG_TIME,
            86 => TickType::FUTURES_OPEN_INTEREST,
            87 => TickType::AVG_OPT_VOLUME,
            88 => TickType::DELAYED_LAST_TIMESTAMP,
            89 => TickType::SHORTABLE_SHARES,
            90 => TickType::DELAYED_HALTED,
            91 => TickType::REUTERS_2_MUTUAL_FUNDS,
            92 => TickType::ETF_NAV_CLOSE,
            93 => TickType::ETF_NAV_PRIOR_CLOSE,
            94 => TickType::ETF_NAV_BID,
            95 => TickType::ETF_NAV_ASK,
            96 => TickType::ETF_NAV_LAST,
            97 => TickType::ETF_FROZEN_NAV_LAST,
            98 => TickType::ETF_NAV_HIGH,
            99 => TickType::ETF_NAV_LOW,
            100 => TickType::SOCIAL_MARKET_ANALYTICS,
            101 => TickType::ESTIMATED_IPO_MIDPOINT,
            102 => TickType::FINAL_IPO_LAST,
            _ => TickType::UNKNOWN,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum GenericTick {
    /// Currently for stocks.
    OptionVolume = 100,
    /// Currently for stocks.
    OptionOpenInterest = 101,
    /// Currently for stocks.
    HistoricalVolatility = 104,
    /// Currently for stocks.
    AverageOptionVolume = 105,
    /// Currently for stocks.
    OptionImpliedVolatility = 106,
    IndexFuturePremium = 162,
    MiscellaneousStats = 165,
    /// Used in TWS P&L computations
    MarkPrice = 221,
    /// Volumes, price, and imbalance
    AuctionValues = 225,
    /// Contains the last trade price, last trade size, last trade time, total volume, VWAP, and
    /// single trade flag.
    RtVolume = 233,
    Shortable = 236,
    Inventory = 256,
    FundamentalRatios = 258,
    RealtimeHistoricalVolatility = 411,
    IbDividends = 456
}
