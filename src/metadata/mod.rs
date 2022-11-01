use std::array;
use std::fmt::{Debug, Display, Formatter};

use chrono::{SecondsFormat, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize, Serializer};
// use serde_aux::prelude::serde_introspect;
use uuid::Uuid;

use display::Chromaticity;

use crate::MDFType::{CMV29, CMV40};

pub mod cmv29;
pub mod cmv40;
pub mod display;
pub mod levels;

pub const XML_PREFIX: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
pub const DOLBY_XMLNS_PREFIX: &str = "http://www.dolby.com/schemas/dvmd/";

/// UUID v4.
#[derive(Debug, Clone, Serialize)]
pub struct UUIDv4(String);

impl UUIDv4 {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for UUIDv4 {
    fn default() -> Self {
        Self(Uuid::default().to_string())
    }
}

pub const CMV40_MIN_VERSION: Version = Version {
    major: 4,
    minor: 0,
    revision: 2,
};

// #[derive(Debug)]
// pub enum CMVersion {
//     CMV29,
//     CMV40,
// }
//
// impl Default for CMVersion {
//     fn default() -> Self {
//         Self::CMV40
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MDFType<T> {
    CMV29(T),
    CMV40(T),
}

impl<T> MDFType<T> {
    pub fn into_inner(self) -> T {
        match self {
            CMV29(t) | CMV40(t) => t,
        }
    }

    pub fn with_new_inner<U>(&mut self, value: U) -> MDFType<U> {
        match self {
            CMV29(_) => CMV29(value),
            CMV40(_) => CMV40(value),
        }
    }
}

impl<T> Default for MDFType<T>
where
    T: Default,
{
    fn default() -> Self {
        CMV40(T::default())
    }
}

impl<T, I> Serialize for MDFType<T>
where
    T: IntoIterator<Item = I> + Copy,
    I: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", &self))
    }
}

impl<T, I> Display for MDFType<T>
where
    T: IntoIterator<Item = I> + Copy,
    I: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let join_str = match &self {
            CMV29(_) => ",",
            CMV40(_) => " ",
        };

        write!(f, "{}", self.into_inner().into_iter().join(join_str))
    }
}

pub trait IntoCMV29<T> {
    /// Convert inner `MDFType` to `CMV29(T)`.
    fn into_cmv29(self) -> T;
}

impl<T, U> IntoCMV29<Option<U>> for Option<T>
where
    T: IntoCMV29<U>,
{
    fn into_cmv29(self) -> Option<U> {
        self.map(|i| i.into_cmv29())
    }
}

impl<T, U> IntoCMV29<Vec<U>> for Vec<T>
where
    T: IntoCMV29<U>,
{
    fn into_cmv29(self) -> Vec<U> {
        self.into_iter().map(|b| b.into_cmv29()).collect::<Vec<_>>()
    }
}

impl<T> IntoCMV29<Self> for MDFType<T> {
    fn into_cmv29(self) -> Self {
        match self {
            CMV29(t) | CMV40(t) => CMV29(t),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum Eotf {
    #[serde(rename = "$primitive=pq")]
    #[default]
    Pq,
    #[serde(rename = "$primitive=linear")]
    Linear,
    #[serde(rename = "$primitive=gamma_bt1886")]
    GammaBT1886,
    #[serde(rename = "$primitive=gamma_dci")]
    GammaDCI,
    #[serde(rename = "$primitive=gamma_22")]
    Gamma22,
    #[serde(rename = "$primitive=gamma_24")]
    Gamma24,
    #[serde(rename = "$primitive=hlg")]
    Hlg,
}

#[derive(Debug, Clone, Serialize)]
pub enum ColorSpace {
    #[serde(rename = "$primitive=rgb")]
    Rgb,
    // #[serde(rename = "$primitive=xyz")]
    // Xyz,
    // #[serde(rename = "$primitive=ycbcr_bt709")]
    // YCbCrBT709,
    // #[serde(rename = "$primitive=ycbcr_bt2020")]
    // YCbCrBT2020,
    // #[serde(rename = "$primitive=ycbcr_native")]
    // YCbCrNative,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ApplicationType {
    #[serde(rename = "$primitive=ALL")]
    All,
    #[serde(rename = "$primitive=HOME")]
    Home,
    // #[serde(rename = "$primitive=CINEMA")]
    // Cinema,
}

#[derive(Debug, Clone, Serialize)]
pub enum SignalRange {
    #[serde(rename = "$primitive=computer")]
    Computer,
    // #[serde(rename = "$primitive=video")]
    // Video,
}

pub const XML_VERSION_LIST: &[[usize; 3]] = &[[2, 0, 5], [4, 0, 2], [5, 1, 0]];

pub enum XMLVersion {
    V205,
    V402,
    V510,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Version {
    major: usize,
    minor: usize,
    revision: usize,
}

impl Version {
    pub fn get_dolby_xmlns(&self) -> String {
        DOLBY_XMLNS_PREFIX.to_string() + &self.into_iter().join("_")
    }
}

impl From<[usize; 3]> for Version {
    fn from(u: [usize; 3]) -> Self {
        Self {
            major: u[0],
            minor: u[1],
            revision: u[2],
        }
    }
}

impl From<XMLVersion> for Version {
    fn from(u: XMLVersion) -> Self {
        Self::from(XML_VERSION_LIST[u as usize])
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::from(XML_VERSION_LIST[0])
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}.{}.{}", &self.major, &self.minor, &self.revision)
        write!(f, "{}", self.into_iter().join("."))
    }
}

impl IntoIterator for Version {
    type Item = usize;
    type IntoIter = array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.major, self.minor, self.revision].into_iter()
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Version {
    // pub fn from_summary(summary:)
}

#[derive(Debug, Serialize)]
pub struct RevisionHistory {
    #[serde(rename = "Revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<Revision>>,
}

impl RevisionHistory {
    pub fn new() -> Self {
        Self {
            revisions: Some(vec![Revision::new()]),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Revision {
    #[serde(rename = "DateTime")]
    pub date_time: DateTime,
    #[serde(rename = "$unflatten=Author")]
    pub author: String,
    #[serde(rename = "$unflatten=Software")]
    pub software: String,
    #[serde(rename = "$unflatten=SoftwareVersion")]
    pub software_version: String,
    #[serde(rename = "$unflatten=Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl Revision {
    pub fn new() -> Self {
        Self {
            date_time: DateTime::new(),
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            software: env!("CARGO_PKG_NAME").to_string(),
            software_version: env!("CARGO_PKG_VERSION").to_string(),
            comment: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DateTime(String);

impl DateTime {
    pub fn new() -> Self {
        Self(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}

// Format: f32,f32 in CMv2.9, f32 f32 in CMv4.0
#[derive(Debug, Clone, Default, Serialize)]
pub struct Primaries {
    #[serde(rename = "$unflatten=Red")]
    pub red: MDFType<Chromaticity>,
    #[serde(rename = "$unflatten=Green")]
    pub green: MDFType<Chromaticity>,
    #[serde(rename = "$unflatten=Blue")]
    pub blue: MDFType<Chromaticity>,
}

impl From<display::Primaries> for Primaries {
    fn from(p: display::Primaries) -> Self {
        Self {
            red: CMV40(p.red),
            green: CMV40(p.green),
            blue: CMV40(p.blue),
        }
    }
}

impl IntoCMV29<Self> for Primaries {
    fn into_cmv29(self) -> Self {
        Self {
            red: self.red.into_cmv29(),
            green: self.green.into_cmv29(),
            blue: self.blue.into_cmv29(),
        }
    }
}
