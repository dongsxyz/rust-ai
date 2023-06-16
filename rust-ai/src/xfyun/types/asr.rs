#[derive(Clone, Copy, Debug)]
pub enum Language {
    /// 中文，通用方言（包括普通话、天津、河北、东北、甘肃、山东、太原）
    Chinese,
    /// 英文
    English,
    /// 日语
    Japanese,
    /// 韩语
    Korean,
    /// 俄语
    Russian,
    /// 法语
    French,
    /// 西班牙语
    Spanish,
    /// 越南语
    Vietnamese,
    /// 西南官话（包括四川、重庆、云南、贵州）
    ChineseXinanese,
    /// 粤语
    ChineseCantonese,
    ///  河南
    ChineseHenanese,
    /// 维吾尔语
    ChineseUyghur,
    /// 藏语
    ChineseTibetan,
    /// 阿拉伯语
    Arabic,
    /// 德语
    German,
    /// 意大利语
    Italian,
}

impl Into<&str> for Language {
    fn into(self) -> &'static str {
        match self {
            Self::Chinese => "cn",
            Self::English => "en",
            Self::Japanese => "ja",
            Self::Korean => "ko",
            Self::Russian => "ru",
            Self::French => "fr",
            Self::Spanish => "es",
            Self::Vietnamese => "vi",
            Self::Arabic => "ar",
            Self::ChineseXinanese => "cn_xinanese",
            Self::ChineseCantonese => "cn_cantonese",
            Self::ChineseHenanese => "cn_henanese",
            Self::ChineseUyghur => "cn_uyghur",
            Self::ChineseTibetan => "cn_tibetan",
            Self::German => "de",
            Self::Italian => "it",
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "cn" => Self::Chinese,
            "en" => Self::English,
            "ja" => Self::Japanese,
            "ko" => Self::Korean,
            "ru" => Self::Russian,
            "fr" => Self::French,
            "es" => Self::Spanish,
            "vi" => Self::Vietnamese,
            "ar" => Self::Arabic,
            "cn_xinanese" => Self::ChineseXinanese,
            "cn_cantonese" => Self::ChineseCantonese,
            "cn_henanese" => Self::ChineseHenanese,
            "cn_uyghur" => Self::ChineseUyghur,
            "cn_tibetan" => Self::ChineseTibetan,
            "de" => Self::German,
            "it" => Self::Italian,
            _ => panic!("Unsupported input value: {}", value),
        }
    }
}

impl serde::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(Into::<&'static str>::into(self.clone()))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ProfessionalDomain {
    /// 法律
    Court,
    /// 教育
    Edu,
    /// 金融
    Finance,
    /// 医疗
    Medical,
    /// 科技
    Tech,
    /// 人文历史
    Culture,
    /// 运营商
    Isp,
    /// 体育
    Sport,
    /// 政府
    Gov,
    /// 游戏
    Game,
    /// 电商
    Ecom,
    /// 军事
    Mil,
    /// 企业
    Com,
    /// 生活
    Life,
    /// 娱乐
    Ent,
    /// 汽车
    Car,
}

impl Into<&str> for ProfessionalDomain {
    fn into(self) -> &'static str {
        match self {
            Self::Court => "court",
            Self::Edu => "edu",
            Self::Finance => "finance",
            Self::Medical => "medical",
            Self::Tech => "tech",
            Self::Culture => "culture",
            Self::Isp => "isp",
            Self::Sport => "sport",
            Self::Gov => "gov",
            Self::Game => "game",
            Self::Ecom => "ecom",
            Self::Mil => "mil",
            Self::Com => "com",
            Self::Life => "life",
            Self::Ent => "ent",
            Self::Car => "car",
        }
    }
}

impl From<&str> for ProfessionalDomain {
    fn from(value: &str) -> Self {
        match value {
            "court" => Self::Court,
            "edu" => Self::Edu,
            "finance" => Self::Finance,
            "medical" => Self::Medical,
            "tech" => Self::Tech,
            "culture" => Self::Culture,
            "isp" => Self::Isp,
            "sport" => Self::Sport,
            "gov" => Self::Gov,
            "game" => Self::Game,
            "ecom" => Self::Ecom,
            "mil" => Self::Mil,
            "com" => Self::Com,
            "life" => Self::Life,
            "ent" => Self::Ent,
            "car" => Self::Car,
            _ => panic!("Unsupported value: {}", value),
        }
    }
}

impl serde::Serialize for ProfessionalDomain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(Into::<&'static str>::into(self.clone()))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AudioMode {
    /// 文件流
    FileStream,
    /// 音频url外链
    ExternalUrl,
}

impl Into<&str> for AudioMode {
    fn into(self) -> &'static str {
        match self {
            Self::FileStream => "fileStream",
            Self::ExternalUrl => "urlLink",
        }
    }
}

impl From<&str> for AudioMode {
    fn from(value: &str) -> Self {
        match value {
            "fileStream" => Self::FileStream,
            "urlLink" => Self::ExternalUrl,
            _ => panic!("Unsupported value: {value}"),
        }
    }
}
impl serde::Serialize for AudioMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(Into::<&'static str>::into(self.clone()))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WavType {
    /// 标准pcm/wav
    StandardWav,
    /// 非标准 wav (默认)
    NonstandardWav,
}

impl Into<u8> for WavType {
    fn into(self) -> u8 {
        match self {
            Self::StandardWav => 1,
            Self::NonstandardWav => 0,
        }
    }
}

impl From<u8> for WavType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::StandardWav,
            0 => Self::NonstandardWav,
            _ => panic!("Unsupported value: {value}"),
        }
    }
}
impl serde::Serialize for WavType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(Into::<u8>::into(self.clone()))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LanguageMixType {
    /// 自动中英文模式
    MixedChineseEnglish,
    /// 中文模式（可能包含少量英文）
    MainlyChinese,
    /// 纯中文模式（不包含英文）
    ChineseOnly,
}

impl Into<u8> for LanguageMixType {
    fn into(self) -> u8 {
        match self {
            Self::MixedChineseEnglish => 1,
            Self::MainlyChinese => 2,
            Self::ChineseOnly => 4,
        }
    }
}

impl From<u8> for LanguageMixType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::MixedChineseEnglish,
            2 => Self::MainlyChinese,
            4 => Self::ChineseOnly,
            _ => panic!("Unsupported value: {value}"),
        }
    }
}
impl serde::Serialize for LanguageMixType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(Into::<u8>::into(self.clone()))
    }
}
