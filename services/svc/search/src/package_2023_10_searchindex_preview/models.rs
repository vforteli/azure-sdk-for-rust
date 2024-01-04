#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An answer is a text passage extracted from the contents of the most relevant documents that matched the query. Answers are extracted from the top search results. Answer candidates are scored and the top answers are selected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnswerResult {
    #[doc = "The score value represents how relevant the answer is to the query relative to other answers returned for the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[doc = "The key of the document the answer was extracted from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The text passage extracted from the document contents as the answer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Same text passage as in the Text property with highlighted text phrases most relevant to the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlights: Option<String>,
}
impl AnswerResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This parameter is only valid if the query type is `semantic`. If set, the query returns answers extracted from key passages in the highest ranked documents. The number of answers returned can be configured by appending the pipe character `|` followed by the `count-<number of answers>` option after the answers parameter value, such as `extractive|count-3`. Default count is 1. The confidence threshold can be configured by appending the pipe character `|` followed by the `threshold-<confidence threshold>` option after the answers parameter value, such as `extractive|threshold-0.9`. Default threshold is 0.7."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Answers")]
pub enum Answers {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "extractive")]
    Extractive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Answers {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Answers {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Answers {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("Answers", 0u32, "none"),
            Self::Extractive => serializer.serialize_unit_variant("Answers", 1u32, "extractive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The result of Autocomplete requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutocompleteItem {
    #[doc = "The completed term."]
    pub text: String,
    #[doc = "The query along with the completed term."]
    #[serde(rename = "queryPlusText")]
    pub query_plus_text: String,
}
impl AutocompleteItem {
    pub fn new(text: String, query_plus_text: String) -> Self {
        Self { text, query_plus_text }
    }
}
#[doc = "Specifies the mode for Autocomplete. The default is 'oneTerm'. Use 'twoTerms' to get shingles and 'oneTermWithContext' to use the current context in producing autocomplete terms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AutocompleteMode {
    #[serde(rename = "oneTerm")]
    OneTerm,
    #[serde(rename = "twoTerms")]
    TwoTerms,
    #[serde(rename = "oneTermWithContext")]
    OneTermWithContext,
}
#[doc = "Parameters for fuzzy matching, and other autocomplete query behaviors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutocompleteRequest {
    #[doc = "The search text on which to base autocomplete results."]
    pub search: String,
    #[doc = "Specifies the mode for Autocomplete. The default is 'oneTerm'. Use 'twoTerms' to get shingles and 'oneTermWithContext' to use the current context in producing autocomplete terms."]
    #[serde(rename = "autocompleteMode", default, skip_serializing_if = "Option::is_none")]
    pub autocomplete_mode: Option<AutocompleteMode>,
    #[doc = "An OData expression that filters the documents used to produce completed terms for the Autocomplete result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "A value indicating whether to use fuzzy matching for the autocomplete query. Default is false. When set to true, the query will autocomplete terms even if there's a substituted or missing character in the search text. While this provides a better experience in some scenarios, it comes at a performance cost as fuzzy autocomplete queries are slower and consume more resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<bool>,
    #[doc = "A string tag that is appended to hit highlights. Must be set with highlightPreTag. If omitted, hit highlighting is disabled."]
    #[serde(rename = "highlightPostTag", default, skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[doc = "A string tag that is prepended to hit highlights. Must be set with highlightPostTag. If omitted, hit highlighting is disabled."]
    #[serde(rename = "highlightPreTag", default, skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    #[doc = "A number between 0 and 100 indicating the percentage of the index that must be covered by an autocomplete query in order for the query to be reported as a success. This parameter can be useful for ensuring search availability even for services with only one replica. The default is 80."]
    #[serde(rename = "minimumCoverage", default, skip_serializing_if = "Option::is_none")]
    pub minimum_coverage: Option<f64>,
    #[doc = "The comma-separated list of field names to consider when querying for auto-completed terms. Target fields must be included in the specified suggester."]
    #[serde(rename = "searchFields", default, skip_serializing_if = "Option::is_none")]
    pub search_fields: Option<String>,
    #[doc = "The name of the suggester as specified in the suggesters collection that's part of the index definition."]
    #[serde(rename = "suggesterName")]
    pub suggester_name: String,
    #[doc = "The number of auto-completed terms to retrieve. This must be a value between 1 and 100. The default is 5."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
impl AutocompleteRequest {
    pub fn new(search: String, suggester_name: String) -> Self {
        Self {
            search,
            autocomplete_mode: None,
            filter: None,
            fuzzy: None,
            highlight_post_tag: None,
            highlight_pre_tag: None,
            minimum_coverage: None,
            search_fields: None,
            suggester_name,
            top: None,
        }
    }
}
#[doc = "The result of Autocomplete query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutocompleteResult {
    #[doc = "A value indicating the percentage of the index that was considered by the autocomplete request, or null if minimumCoverage was not specified in the request."]
    #[serde(rename = "@search.coverage", default, skip_serializing_if = "Option::is_none")]
    pub search_coverage: Option<f64>,
    #[doc = "The list of returned Autocompleted items."]
    pub value: Vec<AutocompleteItem>,
}
impl AutocompleteResult {
    pub fn new(value: Vec<AutocompleteItem>) -> Self {
        Self {
            search_coverage: None,
            value,
        }
    }
}
#[doc = "Captions are the most representative passages from the document relatively to the search query. They are often used as document summary. Captions are only returned for queries of type 'semantic'.."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CaptionResult {
    #[doc = "A representative text passage extracted from the document most relevant to the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Same text passage as in the Text property with highlighted phrases most relevant to the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlights: Option<String>,
}
impl CaptionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This parameter is only valid if the query type is `semantic`. If set, the query returns captions extracted from key passages in the highest ranked documents. When Captions is set to `extractive`, highlighting is enabled by default, and can be configured by appending the pipe character `|` followed by the `highlight-<true/false>` option, such as `extractive|highlight-true`. Defaults to `None`."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Captions")]
pub enum Captions {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "extractive")]
    Extractive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Captions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Captions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Captions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("Captions", 0u32, "none"),
            Self::Extractive => serializer.serialize_unit_variant("Captions", 1u32, "extractive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Contains debugging information that can be used to further explore your search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentDebugInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic: Option<SemanticDebugInfo>,
}
impl DocumentDebugInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single bucket of a facet query result. Reports the number of documents with a field value falling within a particular range or having a particular value or interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetResult {
    #[doc = "The approximate count of documents falling within the bucket described by this facet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl FacetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an index action that operates on a document."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexAction {
    #[doc = "The operation to perform on a document in an indexing batch."]
    #[serde(rename = "@search.action", default, skip_serializing_if = "Option::is_none")]
    pub search_action: Option<index_action::SearchAction>,
}
impl IndexAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod index_action {
    use super::*;
    #[doc = "The operation to perform on a document in an indexing batch."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SearchAction {
        #[serde(rename = "upload")]
        Upload,
        #[serde(rename = "merge")]
        Merge,
        #[serde(rename = "mergeOrUpload")]
        MergeOrUpload,
        #[serde(rename = "delete")]
        Delete,
    }
}
#[doc = "Contains a batch of document write actions to send to the index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexBatch {
    #[doc = "The actions in the batch."]
    pub value: Vec<IndexAction>,
}
impl IndexBatch {
    pub fn new(value: Vec<IndexAction>) -> Self {
        Self { value }
    }
}
#[doc = "Response containing the status of operations for all documents in the indexing request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexDocumentsResult {
    #[doc = "The list of status information for each document in the indexing request."]
    pub value: Vec<IndexingResult>,
}
impl IndexDocumentsResult {
    pub fn new(value: Vec<IndexingResult>) -> Self {
        Self { value }
    }
}
#[doc = "Status of an indexing operation for a single document."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexingResult {
    #[doc = "The key of a document that was in the indexing request."]
    pub key: String,
    #[doc = "The error message explaining why the indexing operation failed for the document identified by the key; null if indexing succeeded."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "A value indicating whether the indexing operation succeeded for the document identified by the key."]
    pub status: bool,
    #[doc = "The status code of the indexing operation. Possible values include: 200 for a successful update or delete, 201 for successful document creation, 400 for a malformed input document, 404 for document not found, 409 for a version conflict, 422 when the index is temporarily unavailable, or 503 for when the service is too busy."]
    #[serde(rename = "statusCode")]
    pub status_code: i32,
}
impl IndexingResult {
    pub fn new(key: String, status: bool, status_code: i32) -> Self {
        Self {
            key,
            error_message: None,
            status,
            status_code,
        }
    }
}
#[doc = "A document retrieved via a document lookup operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LookupDocument {}
impl LookupDocument {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enables a debugging tool that can be used to further explore your search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QueryDebugMode")]
pub enum QueryDebugMode {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QueryDebugMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QueryDebugMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QueryDebugMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("QueryDebugMode", 0u32, "disabled"),
            Self::Semantic => serializer.serialize_unit_variant("QueryDebugMode", 1u32, "semantic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The language of the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QueryLanguage")]
pub enum QueryLanguage {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "en-gb")]
    EnGb,
    #[serde(rename = "en-in")]
    EnIn,
    #[serde(rename = "en-ca")]
    EnCa,
    #[serde(rename = "en-au")]
    EnAu,
    #[serde(rename = "fr-fr")]
    FrFr,
    #[serde(rename = "fr-ca")]
    FrCa,
    #[serde(rename = "de-de")]
    DeDe,
    #[serde(rename = "es-es")]
    EsEs,
    #[serde(rename = "es-mx")]
    EsMx,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "zh-tw")]
    ZhTw,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "pt-pt")]
    PtPt,
    #[serde(rename = "it-it")]
    ItIt,
    #[serde(rename = "ja-jp")]
    JaJp,
    #[serde(rename = "ko-kr")]
    KoKr,
    #[serde(rename = "ru-ru")]
    RuRu,
    #[serde(rename = "cs-cz")]
    CsCz,
    #[serde(rename = "nl-be")]
    NlBe,
    #[serde(rename = "nl-nl")]
    NlNl,
    #[serde(rename = "hu-hu")]
    HuHu,
    #[serde(rename = "pl-pl")]
    PlPl,
    #[serde(rename = "sv-se")]
    SvSe,
    #[serde(rename = "tr-tr")]
    TrTr,
    #[serde(rename = "hi-in")]
    HiIn,
    #[serde(rename = "ar-sa")]
    ArSa,
    #[serde(rename = "ar-eg")]
    ArEg,
    #[serde(rename = "ar-ma")]
    ArMa,
    #[serde(rename = "ar-kw")]
    ArKw,
    #[serde(rename = "ar-jo")]
    ArJo,
    #[serde(rename = "da-dk")]
    DaDk,
    #[serde(rename = "no-no")]
    NoNo,
    #[serde(rename = "bg-bg")]
    BgBg,
    #[serde(rename = "hr-hr")]
    HrHr,
    #[serde(rename = "hr-ba")]
    HrBa,
    #[serde(rename = "ms-my")]
    MsMy,
    #[serde(rename = "ms-bn")]
    MsBn,
    #[serde(rename = "sl-sl")]
    SlSl,
    #[serde(rename = "ta-in")]
    TaIn,
    #[serde(rename = "vi-vn")]
    ViVn,
    #[serde(rename = "el-gr")]
    ElGr,
    #[serde(rename = "ro-ro")]
    RoRo,
    #[serde(rename = "is-is")]
    IsIs,
    #[serde(rename = "id-id")]
    IdId,
    #[serde(rename = "th-th")]
    ThTh,
    #[serde(rename = "lt-lt")]
    LtLt,
    #[serde(rename = "uk-ua")]
    UkUa,
    #[serde(rename = "lv-lv")]
    LvLv,
    #[serde(rename = "et-ee")]
    EtEe,
    #[serde(rename = "ca-es")]
    CaEs,
    #[serde(rename = "fi-fi")]
    FiFi,
    #[serde(rename = "sr-ba")]
    SrBa,
    #[serde(rename = "sr-me")]
    SrMe,
    #[serde(rename = "sr-rs")]
    SrRs,
    #[serde(rename = "sk-sk")]
    SkSk,
    #[serde(rename = "nb-no")]
    NbNo,
    #[serde(rename = "hy-am")]
    HyAm,
    #[serde(rename = "bn-in")]
    BnIn,
    #[serde(rename = "eu-es")]
    EuEs,
    #[serde(rename = "gl-es")]
    GlEs,
    #[serde(rename = "gu-in")]
    GuIn,
    #[serde(rename = "he-il")]
    HeIl,
    #[serde(rename = "ga-ie")]
    GaIe,
    #[serde(rename = "kn-in")]
    KnIn,
    #[serde(rename = "ml-in")]
    MlIn,
    #[serde(rename = "mr-in")]
    MrIn,
    #[serde(rename = "fa-ae")]
    FaAe,
    #[serde(rename = "pa-in")]
    PaIn,
    #[serde(rename = "te-in")]
    TeIn,
    #[serde(rename = "ur-pk")]
    UrPk,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QueryLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QueryLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QueryLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("QueryLanguage", 0u32, "none"),
            Self::EnUs => serializer.serialize_unit_variant("QueryLanguage", 1u32, "en-us"),
            Self::EnGb => serializer.serialize_unit_variant("QueryLanguage", 2u32, "en-gb"),
            Self::EnIn => serializer.serialize_unit_variant("QueryLanguage", 3u32, "en-in"),
            Self::EnCa => serializer.serialize_unit_variant("QueryLanguage", 4u32, "en-ca"),
            Self::EnAu => serializer.serialize_unit_variant("QueryLanguage", 5u32, "en-au"),
            Self::FrFr => serializer.serialize_unit_variant("QueryLanguage", 6u32, "fr-fr"),
            Self::FrCa => serializer.serialize_unit_variant("QueryLanguage", 7u32, "fr-ca"),
            Self::DeDe => serializer.serialize_unit_variant("QueryLanguage", 8u32, "de-de"),
            Self::EsEs => serializer.serialize_unit_variant("QueryLanguage", 9u32, "es-es"),
            Self::EsMx => serializer.serialize_unit_variant("QueryLanguage", 10u32, "es-mx"),
            Self::ZhCn => serializer.serialize_unit_variant("QueryLanguage", 11u32, "zh-cn"),
            Self::ZhTw => serializer.serialize_unit_variant("QueryLanguage", 12u32, "zh-tw"),
            Self::PtBr => serializer.serialize_unit_variant("QueryLanguage", 13u32, "pt-br"),
            Self::PtPt => serializer.serialize_unit_variant("QueryLanguage", 14u32, "pt-pt"),
            Self::ItIt => serializer.serialize_unit_variant("QueryLanguage", 15u32, "it-it"),
            Self::JaJp => serializer.serialize_unit_variant("QueryLanguage", 16u32, "ja-jp"),
            Self::KoKr => serializer.serialize_unit_variant("QueryLanguage", 17u32, "ko-kr"),
            Self::RuRu => serializer.serialize_unit_variant("QueryLanguage", 18u32, "ru-ru"),
            Self::CsCz => serializer.serialize_unit_variant("QueryLanguage", 19u32, "cs-cz"),
            Self::NlBe => serializer.serialize_unit_variant("QueryLanguage", 20u32, "nl-be"),
            Self::NlNl => serializer.serialize_unit_variant("QueryLanguage", 21u32, "nl-nl"),
            Self::HuHu => serializer.serialize_unit_variant("QueryLanguage", 22u32, "hu-hu"),
            Self::PlPl => serializer.serialize_unit_variant("QueryLanguage", 23u32, "pl-pl"),
            Self::SvSe => serializer.serialize_unit_variant("QueryLanguage", 24u32, "sv-se"),
            Self::TrTr => serializer.serialize_unit_variant("QueryLanguage", 25u32, "tr-tr"),
            Self::HiIn => serializer.serialize_unit_variant("QueryLanguage", 26u32, "hi-in"),
            Self::ArSa => serializer.serialize_unit_variant("QueryLanguage", 27u32, "ar-sa"),
            Self::ArEg => serializer.serialize_unit_variant("QueryLanguage", 28u32, "ar-eg"),
            Self::ArMa => serializer.serialize_unit_variant("QueryLanguage", 29u32, "ar-ma"),
            Self::ArKw => serializer.serialize_unit_variant("QueryLanguage", 30u32, "ar-kw"),
            Self::ArJo => serializer.serialize_unit_variant("QueryLanguage", 31u32, "ar-jo"),
            Self::DaDk => serializer.serialize_unit_variant("QueryLanguage", 32u32, "da-dk"),
            Self::NoNo => serializer.serialize_unit_variant("QueryLanguage", 33u32, "no-no"),
            Self::BgBg => serializer.serialize_unit_variant("QueryLanguage", 34u32, "bg-bg"),
            Self::HrHr => serializer.serialize_unit_variant("QueryLanguage", 35u32, "hr-hr"),
            Self::HrBa => serializer.serialize_unit_variant("QueryLanguage", 36u32, "hr-ba"),
            Self::MsMy => serializer.serialize_unit_variant("QueryLanguage", 37u32, "ms-my"),
            Self::MsBn => serializer.serialize_unit_variant("QueryLanguage", 38u32, "ms-bn"),
            Self::SlSl => serializer.serialize_unit_variant("QueryLanguage", 39u32, "sl-sl"),
            Self::TaIn => serializer.serialize_unit_variant("QueryLanguage", 40u32, "ta-in"),
            Self::ViVn => serializer.serialize_unit_variant("QueryLanguage", 41u32, "vi-vn"),
            Self::ElGr => serializer.serialize_unit_variant("QueryLanguage", 42u32, "el-gr"),
            Self::RoRo => serializer.serialize_unit_variant("QueryLanguage", 43u32, "ro-ro"),
            Self::IsIs => serializer.serialize_unit_variant("QueryLanguage", 44u32, "is-is"),
            Self::IdId => serializer.serialize_unit_variant("QueryLanguage", 45u32, "id-id"),
            Self::ThTh => serializer.serialize_unit_variant("QueryLanguage", 46u32, "th-th"),
            Self::LtLt => serializer.serialize_unit_variant("QueryLanguage", 47u32, "lt-lt"),
            Self::UkUa => serializer.serialize_unit_variant("QueryLanguage", 48u32, "uk-ua"),
            Self::LvLv => serializer.serialize_unit_variant("QueryLanguage", 49u32, "lv-lv"),
            Self::EtEe => serializer.serialize_unit_variant("QueryLanguage", 50u32, "et-ee"),
            Self::CaEs => serializer.serialize_unit_variant("QueryLanguage", 51u32, "ca-es"),
            Self::FiFi => serializer.serialize_unit_variant("QueryLanguage", 52u32, "fi-fi"),
            Self::SrBa => serializer.serialize_unit_variant("QueryLanguage", 53u32, "sr-ba"),
            Self::SrMe => serializer.serialize_unit_variant("QueryLanguage", 54u32, "sr-me"),
            Self::SrRs => serializer.serialize_unit_variant("QueryLanguage", 55u32, "sr-rs"),
            Self::SkSk => serializer.serialize_unit_variant("QueryLanguage", 56u32, "sk-sk"),
            Self::NbNo => serializer.serialize_unit_variant("QueryLanguage", 57u32, "nb-no"),
            Self::HyAm => serializer.serialize_unit_variant("QueryLanguage", 58u32, "hy-am"),
            Self::BnIn => serializer.serialize_unit_variant("QueryLanguage", 59u32, "bn-in"),
            Self::EuEs => serializer.serialize_unit_variant("QueryLanguage", 60u32, "eu-es"),
            Self::GlEs => serializer.serialize_unit_variant("QueryLanguage", 61u32, "gl-es"),
            Self::GuIn => serializer.serialize_unit_variant("QueryLanguage", 62u32, "gu-in"),
            Self::HeIl => serializer.serialize_unit_variant("QueryLanguage", 63u32, "he-il"),
            Self::GaIe => serializer.serialize_unit_variant("QueryLanguage", 64u32, "ga-ie"),
            Self::KnIn => serializer.serialize_unit_variant("QueryLanguage", 65u32, "kn-in"),
            Self::MlIn => serializer.serialize_unit_variant("QueryLanguage", 66u32, "ml-in"),
            Self::MrIn => serializer.serialize_unit_variant("QueryLanguage", 67u32, "mr-in"),
            Self::FaAe => serializer.serialize_unit_variant("QueryLanguage", 68u32, "fa-ae"),
            Self::PaIn => serializer.serialize_unit_variant("QueryLanguage", 69u32, "pa-in"),
            Self::TeIn => serializer.serialize_unit_variant("QueryLanguage", 70u32, "te-in"),
            Self::UrPk => serializer.serialize_unit_variant("QueryLanguage", 71u32, "ur-pk"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The raw concatenated strings that were sent to the semantic enrichment process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResultDocumentRerankerInput {
    #[doc = "The raw string for the title field that was used for semantic enrichment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The raw concatenated strings for the content fields that were used for semantic enrichment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The raw concatenated strings for the keyword fields that were used for semantic enrichment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}
impl QueryResultDocumentRerankerInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of fields that were sent to the semantic enrichment process, as well as how they were used"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResultDocumentSemanticField {
    #[doc = "The name of the field that was sent to the semantic enrichment process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The way the field was used for the semantic enrichment process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<QueryResultDocumentSemanticFieldState>,
}
impl QueryResultDocumentSemanticField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The way the field was used for the semantic enrichment process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QueryResultDocumentSemanticFieldState")]
pub enum QueryResultDocumentSemanticFieldState {
    #[serde(rename = "used")]
    Used,
    #[serde(rename = "unused")]
    Unused,
    #[serde(rename = "partial")]
    Partial,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QueryResultDocumentSemanticFieldState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QueryResultDocumentSemanticFieldState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QueryResultDocumentSemanticFieldState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Used => serializer.serialize_unit_variant("QueryResultDocumentSemanticFieldState", 0u32, "used"),
            Self::Unused => serializer.serialize_unit_variant("QueryResultDocumentSemanticFieldState", 1u32, "unused"),
            Self::Partial => serializer.serialize_unit_variant("QueryResultDocumentSemanticFieldState", 2u32, "partial"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the syntax of the search query. The default is 'simple'. Use 'full' if your query uses the Lucene query syntax and 'semantic' if query syntax is not needed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "semantic")]
    Semantic,
}
#[doc = "The query parameters to use for vector search when a raw vector value is provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawVectorQuery {
    #[serde(flatten)]
    pub vector_query: VectorQuery,
    #[doc = "The vector representation of a search query."]
    pub vector: Vec<f32>,
}
impl RawVectorQuery {
    pub fn new(vector_query: VectorQuery, vector: Vec<f32>) -> Self {
        Self { vector_query, vector }
    }
}
#[doc = "A value that specifies whether we want to calculate scoring statistics (such as document frequency) globally for more consistent scoring, or locally, for lower latency. The default is 'local'. Use 'global' to aggregate scoring statistics globally before scoring. Using global scoring statistics can increase latency of search queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScoringStatistics {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "global")]
    Global,
}
#[doc = "Response containing search results from an index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchDocumentsResult {
    #[doc = "The total count of results found by the search operation, or null if the count was not requested. If present, the count may be greater than the number of results in this response. This can happen if you use the $top or $skip parameters, or if the query can't return all the requested documents in a single response."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i64>,
    #[doc = "A value indicating the percentage of the index that was included in the query, or null if minimumCoverage was not specified in the request."]
    #[serde(rename = "@search.coverage", default, skip_serializing_if = "Option::is_none")]
    pub search_coverage: Option<f64>,
    #[doc = "The facet query results for the search operation, organized as a collection of buckets for each faceted field; null if the query did not include any facet expressions."]
    #[serde(rename = "@search.facets", default, skip_serializing_if = "Option::is_none")]
    pub search_facets: Option<serde_json::Value>,
    #[doc = "The answers query results for the search operation; null if the answers query parameter was not specified or set to 'none'."]
    #[serde(
        rename = "@search.answers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_answers: Vec<AnswerResult>,
    #[doc = "Parameters for filtering, sorting, faceting, paging, and other search query behaviors."]
    #[serde(rename = "@search.nextPageParameters", default, skip_serializing_if = "Option::is_none")]
    pub search_next_page_parameters: Option<SearchRequest>,
    #[doc = "Reason that a partial response was returned for a semantic ranking request."]
    #[serde(rename = "@search.semanticPartialResponseReason", default, skip_serializing_if = "Option::is_none")]
    pub search_semantic_partial_response_reason: Option<SemanticPartialResponseReason>,
    #[doc = "Type of partial response that was returned for a semantic ranking request."]
    #[serde(rename = "@search.semanticPartialResponseType", default, skip_serializing_if = "Option::is_none")]
    pub search_semantic_partial_response_type: Option<SemanticPartialResponseType>,
    #[doc = "The sequence of results returned by the query."]
    pub value: Vec<SearchResult>,
    #[doc = "Continuation URL returned when the query can't return all the requested results in a single response. You can use this URL to formulate another GET or POST Search request to get the next part of the search response. Make sure to use the same verb (GET or POST) as the request that produced this response."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl SearchDocumentsResult {
    pub fn new(value: Vec<SearchResult>) -> Self {
        Self {
            odata_count: None,
            search_coverage: None,
            search_facets: None,
            search_answers: Vec::new(),
            search_next_page_parameters: None,
            search_semantic_partial_response_reason: None,
            search_semantic_partial_response_type: None,
            value,
            odata_next_link: None,
        }
    }
}
#[doc = "Describes an error condition for the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<SearchError>,
}
impl SearchError {
    pub fn new(message: String) -> Self {
        Self {
            code: None,
            message,
            details: Vec::new(),
        }
    }
}
#[doc = "Specifies whether any or all of the search terms must be matched in order to count the document as a match."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SearchMode {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "all")]
    All,
}
#[doc = "Parameters for filtering, sorting, faceting, paging, and other search query behaviors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    #[doc = "A value that specifies whether to fetch the total count of results. Default is false. Setting this value to true may have a performance impact. Note that the count returned is an approximation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<bool>,
    #[doc = "The list of facet expressions to apply to the search query. Each facet expression contains a field name, optionally followed by a comma-separated list of name:value pairs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub facets: Vec<String>,
    #[doc = "The OData $filter expression to apply to the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "The comma-separated list of field names to use for hit highlights. Only searchable fields can be used for hit highlighting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<String>,
    #[doc = "A string tag that is appended to hit highlights. Must be set with highlightPreTag. Default is &lt;/em&gt;."]
    #[serde(rename = "highlightPostTag", default, skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[doc = "A string tag that is prepended to hit highlights. Must be set with highlightPostTag. Default is &lt;em&gt;."]
    #[serde(rename = "highlightPreTag", default, skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    #[doc = "A number between 0 and 100 indicating the percentage of the index that must be covered by a search query in order for the query to be reported as a success. This parameter can be useful for ensuring search availability even for services with only one replica. The default is 100."]
    #[serde(rename = "minimumCoverage", default, skip_serializing_if = "Option::is_none")]
    pub minimum_coverage: Option<f64>,
    #[doc = "The comma-separated list of OData $orderby expressions by which to sort the results. Each expression can be either a field name or a call to either the geo.distance() or the search.score() functions. Each expression can be followed by asc to indicate ascending, or desc to indicate descending. The default is ascending order. Ties will be broken by the match scores of documents. If no $orderby is specified, the default sort order is descending by document match score. There can be at most 32 $orderby clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orderby: Option<String>,
    #[doc = "Specifies the syntax of the search query. The default is 'simple'. Use 'full' if your query uses the Lucene query syntax and 'semantic' if query syntax is not needed."]
    #[serde(rename = "queryType", default, skip_serializing_if = "Option::is_none")]
    pub query_type: Option<QueryType>,
    #[doc = "A value that specifies whether we want to calculate scoring statistics (such as document frequency) globally for more consistent scoring, or locally, for lower latency. The default is 'local'. Use 'global' to aggregate scoring statistics globally before scoring. Using global scoring statistics can increase latency of search queries."]
    #[serde(rename = "scoringStatistics", default, skip_serializing_if = "Option::is_none")]
    pub scoring_statistics: Option<ScoringStatistics>,
    #[doc = "A value to be used to create a sticky session, which can help getting more consistent results. As long as the same sessionId is used, a best-effort attempt will be made to target the same replica set. Be wary that reusing the same sessionID values repeatedly can interfere with the load balancing of the requests across replicas and adversely affect the performance of the search service. The value used as sessionId cannot start with a '_' character."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The list of parameter values to be used in scoring functions (for example, referencePointParameter) using the format name-values. For example, if the scoring profile defines a function with a parameter called 'mylocation' the parameter string would be \"mylocation--122.2,44.8\" (without the quotes)."]
    #[serde(
        rename = "scoringParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scoring_parameters: Vec<String>,
    #[doc = "The name of a scoring profile to evaluate match scores for matching documents in order to sort the results."]
    #[serde(rename = "scoringProfile", default, skip_serializing_if = "Option::is_none")]
    pub scoring_profile: Option<String>,
    #[doc = "Allows setting a separate search query that will be solely used for semantic reranking, semantic captions and semantic answers. Is useful for scenarios where there is a need to use different queries between the base retrieval and ranking phase, and the L2 semantic phase."]
    #[serde(rename = "semanticQuery", default, skip_serializing_if = "Option::is_none")]
    pub semantic_query: Option<String>,
    #[doc = "The name of a semantic configuration that will be used when processing documents for queries of type semantic."]
    #[serde(rename = "semanticConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub semantic_configuration: Option<String>,
    #[doc = "Allows the user to choose whether a semantic call should fail completely, or to return partial results."]
    #[serde(rename = "semanticErrorHandling", default, skip_serializing_if = "Option::is_none")]
    pub semantic_error_handling: Option<SemanticErrorHandling>,
    #[doc = "Allows the user to set an upper bound on the amount of time it takes for semantic enrichment to finish processing before the request fails."]
    #[serde(rename = "semanticMaxWaitInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub semantic_max_wait_in_milliseconds: Option<i32>,
    #[doc = "Enables a debugging tool that can be used to further explore your search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<QueryDebugMode>,
    #[doc = "A full-text search query expression; Use \"*\" or omit this parameter to match all documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[doc = "The comma-separated list of field names to which to scope the full-text search. When using fielded search (fieldName:searchExpression) in a full Lucene query, the field names of each fielded search expression take precedence over any field names listed in this parameter."]
    #[serde(rename = "searchFields", default, skip_serializing_if = "Option::is_none")]
    pub search_fields: Option<String>,
    #[doc = "Specifies whether any or all of the search terms must be matched in order to count the document as a match."]
    #[serde(rename = "searchMode", default, skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<SearchMode>,
    #[doc = "The language of the query."]
    #[serde(rename = "queryLanguage", default, skip_serializing_if = "Option::is_none")]
    pub query_language: Option<QueryLanguage>,
    #[doc = "Improve search recall by spell-correcting individual search query terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speller: Option<Speller>,
    #[doc = "This parameter is only valid if the query type is `semantic`. If set, the query returns answers extracted from key passages in the highest ranked documents. The number of answers returned can be configured by appending the pipe character `|` followed by the `count-<number of answers>` option after the answers parameter value, such as `extractive|count-3`. Default count is 1. The confidence threshold can be configured by appending the pipe character `|` followed by the `threshold-<confidence threshold>` option after the answers parameter value, such as `extractive|threshold-0.9`. Default threshold is 0.7."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<Answers>,
    #[doc = "The comma-separated list of fields to retrieve. If unspecified, all fields marked as retrievable in the schema are included."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    #[doc = "The number of search results to skip. This value cannot be greater than 100,000. If you need to scan documents in sequence, but cannot use skip due to this limitation, consider using orderby on a totally-ordered key and filter with a range query instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[doc = "The number of search results to retrieve. This can be used in conjunction with $skip to implement client-side paging of search results. If results are truncated due to server-side paging, the response will include a continuation token that can be used to issue another Search request for the next page of results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "This parameter is only valid if the query type is `semantic`. If set, the query returns captions extracted from key passages in the highest ranked documents. When Captions is set to `extractive`, highlighting is enabled by default, and can be configured by appending the pipe character `|` followed by the `highlight-<true/false>` option, such as `extractive|highlight-true`. Defaults to `None`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captions: Option<Captions>,
    #[doc = "The comma-separated list of field names used for semantic ranking."]
    #[serde(rename = "semanticFields", default, skip_serializing_if = "Option::is_none")]
    pub semantic_fields: Option<String>,
    #[doc = "The query parameters for vector and hybrid search queries."]
    #[serde(
        rename = "vectorQueries",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vector_queries: Vec<VectorQueryUnion>,
    #[doc = "Determines whether or not filters are applied before or after the vector search is performed."]
    #[serde(rename = "vectorFilterMode", default, skip_serializing_if = "Option::is_none")]
    pub vector_filter_mode: Option<VectorFilterMode>,
}
impl SearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains a document found by a search query, plus associated metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[doc = "The relevance score of the document compared to other documents returned by the query."]
    #[serde(rename = "@search.score")]
    pub search_score: f64,
    #[doc = "The relevance score computed by the semantic ranker for the top search results. Search results are sorted by the RerankerScore first and then by the Score. RerankerScore is only returned for queries of type 'semantic'."]
    #[serde(rename = "@search.rerankerScore", default, skip_serializing_if = "Option::is_none")]
    pub search_reranker_score: Option<f64>,
    #[doc = "Text fragments from the document that indicate the matching search terms, organized by each applicable field; null if hit highlighting was not enabled for the query."]
    #[serde(rename = "@search.highlights", default, skip_serializing_if = "Option::is_none")]
    pub search_highlights: Option<serde_json::Value>,
    #[doc = "Captions are the most representative passages from the document relatively to the search query. They are often used as document summary. Captions are only returned for queries of type 'semantic'."]
    #[serde(
        rename = "@search.captions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_captions: Vec<CaptionResult>,
    #[doc = "Contains debugging information that can be used to further explore your search results."]
    #[serde(
        rename = "@search.documentDebugInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub search_document_debug_info: Vec<DocumentDebugInfo>,
}
impl SearchResult {
    pub fn new(search_score: f64) -> Self {
        Self {
            search_score,
            search_reranker_score: None,
            search_highlights: None,
            search_captions: Vec::new(),
            search_document_debug_info: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SemanticDebugInfo {
    #[doc = "Description of fields that were sent to the semantic enrichment process, as well as how they were used"]
    #[serde(rename = "titleField", default, skip_serializing_if = "Option::is_none")]
    pub title_field: Option<QueryResultDocumentSemanticField>,
    #[doc = "The content fields that were sent to the semantic enrichment process, as well as how they were used"]
    #[serde(
        rename = "contentFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub content_fields: Vec<QueryResultDocumentSemanticField>,
    #[doc = "The keyword fields that were sent to the semantic enrichment process, as well as how they were used"]
    #[serde(
        rename = "keywordFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub keyword_fields: Vec<QueryResultDocumentSemanticField>,
    #[doc = "The raw concatenated strings that were sent to the semantic enrichment process."]
    #[serde(rename = "rerankerInput", default, skip_serializing_if = "Option::is_none")]
    pub reranker_input: Option<QueryResultDocumentRerankerInput>,
}
impl SemanticDebugInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Allows the user to choose whether a semantic call should fail completely, or to return partial results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SemanticErrorHandling")]
pub enum SemanticErrorHandling {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "fail")]
    Fail,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SemanticErrorHandling {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SemanticErrorHandling {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SemanticErrorHandling {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Partial => serializer.serialize_unit_variant("SemanticErrorHandling", 0u32, "partial"),
            Self::Fail => serializer.serialize_unit_variant("SemanticErrorHandling", 1u32, "fail"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Reason that a partial response was returned for a semantic ranking request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SemanticPartialResponseReason")]
pub enum SemanticPartialResponseReason {
    #[serde(rename = "maxWaitExceeded")]
    MaxWaitExceeded,
    #[serde(rename = "capacityOverloaded")]
    CapacityOverloaded,
    #[serde(rename = "transient")]
    Transient,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SemanticPartialResponseReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SemanticPartialResponseReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SemanticPartialResponseReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MaxWaitExceeded => serializer.serialize_unit_variant("SemanticPartialResponseReason", 0u32, "maxWaitExceeded"),
            Self::CapacityOverloaded => serializer.serialize_unit_variant("SemanticPartialResponseReason", 1u32, "capacityOverloaded"),
            Self::Transient => serializer.serialize_unit_variant("SemanticPartialResponseReason", 2u32, "transient"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Type of partial response that was returned for a semantic ranking request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SemanticPartialResponseType")]
pub enum SemanticPartialResponseType {
    #[serde(rename = "baseResults")]
    BaseResults,
    #[serde(rename = "rerankedResults")]
    RerankedResults,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SemanticPartialResponseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SemanticPartialResponseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SemanticPartialResponseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BaseResults => serializer.serialize_unit_variant("SemanticPartialResponseType", 0u32, "baseResults"),
            Self::RerankedResults => serializer.serialize_unit_variant("SemanticPartialResponseType", 1u32, "rerankedResults"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Improve search recall by spell-correcting individual search query terms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Speller")]
pub enum Speller {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "lexicon")]
    Lexicon,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Speller {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Speller {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Speller {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("Speller", 0u32, "none"),
            Self::Lexicon => serializer.serialize_unit_variant("Speller", 1u32, "lexicon"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Response containing suggestion query results from an index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestDocumentsResult {
    #[doc = "The sequence of results returned by the query."]
    pub value: Vec<SuggestResult>,
    #[doc = "A value indicating the percentage of the index that was included in the query, or null if minimumCoverage was not set in the request."]
    #[serde(rename = "@search.coverage", default, skip_serializing_if = "Option::is_none")]
    pub search_coverage: Option<f64>,
}
impl SuggestDocumentsResult {
    pub fn new(value: Vec<SuggestResult>) -> Self {
        Self {
            value,
            search_coverage: None,
        }
    }
}
#[doc = "Parameters for filtering, sorting, fuzzy matching, and other suggestions query behaviors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestRequest {
    #[doc = "An OData expression that filters the documents considered for suggestions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "A value indicating whether to use fuzzy matching for the suggestion query. Default is false. When set to true, the query will find suggestions even if there's a substituted or missing character in the search text. While this provides a better experience in some scenarios, it comes at a performance cost as fuzzy suggestion searches are slower and consume more resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<bool>,
    #[doc = "A string tag that is appended to hit highlights. Must be set with highlightPreTag. If omitted, hit highlighting of suggestions is disabled."]
    #[serde(rename = "highlightPostTag", default, skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[doc = "A string tag that is prepended to hit highlights. Must be set with highlightPostTag. If omitted, hit highlighting of suggestions is disabled."]
    #[serde(rename = "highlightPreTag", default, skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    #[doc = "A number between 0 and 100 indicating the percentage of the index that must be covered by a suggestion query in order for the query to be reported as a success. This parameter can be useful for ensuring search availability even for services with only one replica. The default is 80."]
    #[serde(rename = "minimumCoverage", default, skip_serializing_if = "Option::is_none")]
    pub minimum_coverage: Option<f64>,
    #[doc = "The comma-separated list of OData $orderby expressions by which to sort the results. Each expression can be either a field name or a call to either the geo.distance() or the search.score() functions. Each expression can be followed by asc to indicate ascending, or desc to indicate descending. The default is ascending order. Ties will be broken by the match scores of documents. If no $orderby is specified, the default sort order is descending by document match score. There can be at most 32 $orderby clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orderby: Option<String>,
    #[doc = "The search text to use to suggest documents. Must be at least 1 character, and no more than 100 characters."]
    pub search: String,
    #[doc = "The comma-separated list of field names to search for the specified search text. Target fields must be included in the specified suggester."]
    #[serde(rename = "searchFields", default, skip_serializing_if = "Option::is_none")]
    pub search_fields: Option<String>,
    #[doc = "The comma-separated list of fields to retrieve. If unspecified, only the key field will be included in the results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    #[doc = "The name of the suggester as specified in the suggesters collection that's part of the index definition."]
    #[serde(rename = "suggesterName")]
    pub suggester_name: String,
    #[doc = "The number of suggestions to retrieve. This must be a value between 1 and 100. The default is 5."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
impl SuggestRequest {
    pub fn new(search: String, suggester_name: String) -> Self {
        Self {
            filter: None,
            fuzzy: None,
            highlight_post_tag: None,
            highlight_pre_tag: None,
            minimum_coverage: None,
            orderby: None,
            search,
            search_fields: None,
            select: None,
            suggester_name,
            top: None,
        }
    }
}
#[doc = "A result containing a document found by a suggestion query, plus associated metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestResult {
    #[doc = "The text of the suggestion result."]
    #[serde(rename = "@search.text")]
    pub search_text: String,
}
impl SuggestResult {
    pub fn new(search_text: String) -> Self {
        Self { search_text }
    }
}
#[doc = "Determines whether or not filters are applied before or after the vector search is performed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VectorFilterMode")]
pub enum VectorFilterMode {
    #[serde(rename = "postFilter")]
    PostFilter,
    #[serde(rename = "preFilter")]
    PreFilter,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VectorFilterMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VectorFilterMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VectorFilterMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PostFilter => serializer.serialize_unit_variant("VectorFilterMode", 0u32, "postFilter"),
            Self::PreFilter => serializer.serialize_unit_variant("VectorFilterMode", 1u32, "preFilter"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The query parameters for vector and hybrid search queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorQuery {
    #[doc = "Number of nearest neighbors to return as top hits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<i32>,
    #[doc = "Vector Fields of type Collection(Edm.Single) to be included in the vector searched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[doc = "When true, triggers an exhaustive k-nearest neighbor search across all vectors within the vector index. Useful for scenarios where exact matches are critical, such as determining ground truth values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exhaustive: Option<bool>,
}
impl VectorQuery {
    pub fn new() -> Self {
        Self {
            k: None,
            fields: None,
            exhaustive: None,
        }
    }
}
#[doc = "The kind of vector query being performed."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum VectorQueryUnion {
    #[serde(rename = "vector")]
    Vector(RawVectorQuery),
    #[serde(rename = "text")]
    Text(VectorizableTextQuery),
}
#[doc = "The kind of vector query being performed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VectorQueryKind")]
pub enum VectorQueryKind {
    #[serde(rename = "vector")]
    Vector,
    #[serde(rename = "text")]
    Text,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VectorQueryKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VectorQueryKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VectorQueryKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Vector => serializer.serialize_unit_variant("VectorQueryKind", 0u32, "vector"),
            Self::Text => serializer.serialize_unit_variant("VectorQueryKind", 1u32, "text"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The query parameters to use for vector search when a text value that needs to be vectorized is provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorizableTextQuery {
    #[serde(flatten)]
    pub vector_query: VectorQuery,
    #[doc = "The text to be vectorized to perform a vector search query."]
    pub text: String,
}
impl VectorizableTextQuery {
    pub fn new(vector_query: VectorQuery, text: String) -> Self {
        Self { vector_query, text }
    }
}
