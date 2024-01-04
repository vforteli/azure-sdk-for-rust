#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The AML skill allows you to extend AI enrichment with a custom Azure Machine Learning (AML) model. Once an AML model is trained and deployed, an AML skill integrates it into AI enrichment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "(Required for no authentication or key authentication) The scoring URI of the AML service to which the JSON payload will be sent. Only the https URI scheme is allowed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "(Required for key authentication) The key for the AML service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "(Required for token authentication). The Azure Resource Manager resource ID of the AML service. It should be in the format subscriptions/{guid}/resourceGroups/{resource-group-name}/Microsoft.MachineLearningServices/workspaces/{workspace-name}/services/{service_name}."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "(Optional) When specified, indicates the timeout for the http client making the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "(Optional for token authentication). The region the AML service is deployed in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "(Optional) When specified, indicates the number of calls the indexer will make in parallel to the endpoint you have provided. You can decrease this value if your endpoint is failing under too high of a request load, or raise it if your endpoint is able to accept more requests and you would like an increase in the performance of the indexer. If not set, a default value of 5 is used. The degreeOfParallelism can be set to a maximum of 10 and a minimum of 1."]
    #[serde(rename = "degreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism: Option<i32>,
}
impl AmlSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            uri: None,
            key: None,
            resource_id: None,
            timeout: None,
            region: None,
            degree_of_parallelism: None,
        }
    }
}
#[doc = "Specifies some text and analysis components used to break that text into tokens."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    #[doc = "The text to break into tokens."]
    pub text: String,
    #[doc = "Defines the names of all text analyzers supported by the search engine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<LexicalAnalyzerName>,
    #[doc = "Defines the names of all tokenizers supported by the search engine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer: Option<LexicalTokenizerName>,
    #[doc = "Defines the names of all text normalizers supported by the search engine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<LexicalNormalizerName>,
    #[doc = "An optional list of token filters to use when breaking the given text."]
    #[serde(
        rename = "tokenFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_filters: Vec<TokenFilterName>,
    #[doc = "An optional list of character filters to use when breaking the given text."]
    #[serde(
        rename = "charFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub char_filters: Vec<CharFilterName>,
}
impl AnalyzeRequest {
    pub fn new(text: String) -> Self {
        Self {
            text,
            analyzer: None,
            tokenizer: None,
            normalizer: None,
            token_filters: Vec::new(),
            char_filters: Vec::new(),
        }
    }
}
#[doc = "The result of testing an analyzer on text."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyzeResult {
    #[doc = "The list of tokens returned by the analyzer specified in the request."]
    pub tokens: Vec<AnalyzedTokenInfo>,
}
impl AnalyzeResult {
    pub fn new(tokens: Vec<AnalyzedTokenInfo>) -> Self {
        Self { tokens }
    }
}
#[doc = "Information about a token returned by an analyzer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedTokenInfo {
    #[doc = "The token returned by the analyzer."]
    pub token: String,
    #[doc = "The index of the first character of the token in the input text."]
    #[serde(rename = "startOffset")]
    pub start_offset: i32,
    #[doc = "The index of the last character of the token in the input text."]
    #[serde(rename = "endOffset")]
    pub end_offset: i32,
    #[doc = "The position of the token in the input text relative to other tokens. The first token in the input text has position 0, the next has position 1, and so on. Depending on the analyzer used, some tokens might have the same position, for example if they are synonyms of each other."]
    pub position: i32,
}
impl AnalyzedTokenInfo {
    pub fn new(token: String, start_offset: i32, end_offset: i32, position: i32) -> Self {
        Self {
            token,
            start_offset,
            end_offset,
            position,
        }
    }
}
#[doc = "Converts alphabetic, numeric, and symbolic Unicode characters which are not in the first 127 ASCII characters (the \"Basic Latin\" Unicode block) into their ASCII equivalents, if such equivalents exist. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsciiFoldingTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A value indicating whether the original token will be kept. Default is false."]
    #[serde(rename = "preserveOriginal", default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<bool>,
}
impl AsciiFoldingTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            preserve_original: None,
        }
    }
}
#[doc = "Credentials of a registered application created for your search service, used for authenticated access to the encryption keys stored in Azure Key Vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureActiveDirectoryApplicationCredentials {
    #[doc = "An AAD Application ID that was granted the required access permissions to the Azure Key Vault that is to be used when encrypting your data at rest. The Application ID should not be confused with the Object ID for your AAD Application."]
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "The authentication key of the specified AAD application."]
    #[serde(rename = "applicationSecret", default, skip_serializing_if = "Option::is_none")]
    pub application_secret: Option<String>,
}
impl AzureActiveDirectoryApplicationCredentials {
    pub fn new(application_id: String) -> Self {
        Self {
            application_id,
            application_secret: None,
        }
    }
}
#[doc = "Allows you to generate a vector embedding for a given text input using the Azure OpenAI resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOpenAiEmbeddingSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The resource URI for your Azure OpenAI resource."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "ID of your Azure OpenAI model deployment on the designated resource."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "API key for the designated Azure OpenAI resource."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "Abstract base type for data identities."]
    #[serde(rename = "authIdentity", default, skip_serializing_if = "Option::is_none")]
    pub auth_identity: Option<SearchIndexerDataIdentityUnion>,
}
impl AzureOpenAiEmbeddingSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            resource_uri: None,
            deployment_id: None,
            api_key: None,
            auth_identity: None,
        }
    }
}
#[doc = "Specifies the parameters for connecting to the Azure OpenAI resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureOpenAiParameters {
    #[doc = "The resource URI of the Azure OpenAI resource."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "ID of the Azure OpenAI model deployment on the designated resource."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "API key of the designated Azure OpenAI resource."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "Abstract base type for data identities."]
    #[serde(rename = "authIdentity", default, skip_serializing_if = "Option::is_none")]
    pub auth_identity: Option<SearchIndexerDataIdentityUnion>,
}
impl AzureOpenAiParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the Azure OpenAI resource used to vectorize a query string."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOpenAiVectorizer {
    #[serde(flatten)]
    pub vector_search_vectorizer: VectorSearchVectorizer,
    #[doc = "Specifies the parameters for connecting to the Azure OpenAI resource."]
    #[serde(rename = "azureOpenAIParameters", default, skip_serializing_if = "Option::is_none")]
    pub azure_open_ai_parameters: Option<AzureOpenAiParameters>,
}
impl AzureOpenAiVectorizer {
    pub fn new(vector_search_vectorizer: VectorSearchVectorizer) -> Self {
        Self {
            vector_search_vectorizer,
            azure_open_ai_parameters: None,
        }
    }
}
#[doc = "Ranking function based on the Okapi BM25 similarity algorithm. BM25 is a TF-IDF-like algorithm that includes length normalization (controlled by the 'b' parameter) as well as term frequency saturation (controlled by the 'k1' parameter)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bm25Similarity {
    #[doc = "This property controls the scaling function between the term frequency of each matching terms and the final relevance score of a document-query pair. By default, a value of 1.2 is used. A value of 0.0 means the score does not scale with an increase in term frequency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    #[doc = "This property controls how the length of a document affects the relevance score. By default, a value of 0.75 is used. A value of 0.0 means no length normalization is applied, while a value of 1.0 means the score is fully normalized by the length of the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
}
impl Bm25Similarity {
    pub fn new() -> Self {
        Self { k1: None, b: None }
    }
}
#[doc = "Base type for character filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharFilter {
    #[doc = "The name of the char filter. It must only contain letters, digits, spaces, dashes or underscores, can only start and end with alphanumeric characters, and is limited to 128 characters."]
    pub name: String,
}
impl CharFilter {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "A URI fragment specifying the type of char filter."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum CharFilterUnion {
    #[serde(rename = "#Microsoft.Azure.Search.MappingCharFilter")]
    MicrosoftAzureSearchMappingCharFilter(MappingCharFilter),
    #[serde(rename = "#Microsoft.Azure.Search.PatternReplaceCharFilter")]
    MicrosoftAzureSearchPatternReplaceCharFilter(PatternReplaceCharFilter),
}
#[doc = "Defines the names of all character filters supported by the search engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CharFilterName")]
pub enum CharFilterName {
    #[serde(rename = "html_strip")]
    HtmlStrip,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CharFilterName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CharFilterName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CharFilterName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::HtmlStrip => serializer.serialize_unit_variant("CharFilterName", 0u32, "html_strip"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Forms bigrams of CJK terms that are generated from the standard tokenizer. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CjkBigramTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The scripts to ignore."]
    #[serde(
        rename = "ignoreScripts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ignore_scripts: Vec<CjkBigramTokenFilterScripts>,
    #[doc = "A value indicating whether to output both unigrams and bigrams (if true), or just bigrams (if false). Default is false."]
    #[serde(rename = "outputUnigrams", default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams: Option<bool>,
}
impl CjkBigramTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            ignore_scripts: Vec::new(),
            output_unigrams: None,
        }
    }
}
#[doc = "Scripts that can be ignored by CjkBigramTokenFilter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CjkBigramTokenFilterScripts {
    #[serde(rename = "han")]
    Han,
    #[serde(rename = "hiragana")]
    Hiragana,
    #[serde(rename = "katakana")]
    Katakana,
    #[serde(rename = "hangul")]
    Hangul,
}
#[doc = "Legacy similarity algorithm which uses the Lucene TFIDFSimilarity implementation of TF-IDF. This variation of TF-IDF introduces static document length normalization as well as coordinating factors that penalize documents that only partially match the searched queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassicSimilarity {}
impl ClassicSimilarity {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Grammar-based tokenizer that is suitable for processing most European-language documents. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassicTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Default is 255. Tokens longer than the maximum length are split. The maximum token length that can be used is 300 characters."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
}
impl ClassicTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
        }
    }
}
#[doc = "Base type for describing any Azure AI service resource attached to a skillset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccount {
    #[doc = "Description of the Azure AI service resource attached to a skillset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CognitiveServicesAccount {
    pub fn new() -> Self {
        Self { description: None }
    }
}
#[doc = "A URI fragment specifying the type of Azure AI service resource attached to a skillset."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum CognitiveServicesAccountUnion {
    #[serde(rename = "#Microsoft.Azure.Search.CognitiveServicesByKey")]
    MicrosoftAzureSearchCognitiveServicesByKey(CognitiveServicesAccountKey),
    #[serde(rename = "#Microsoft.Azure.Search.DefaultCognitiveServices")]
    MicrosoftAzureSearchDefaultCognitiveServices(DefaultCognitiveServicesAccount),
}
#[doc = "The multi-region account key of an Azure AI service resource that's attached to a skillset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountKey {
    #[serde(flatten)]
    pub cognitive_services_account: CognitiveServicesAccount,
    #[doc = "The key used to provision the Azure AI service resource attached to a skillset."]
    pub key: String,
}
impl CognitiveServicesAccountKey {
    pub fn new(cognitive_services_account: CognitiveServicesAccount, key: String) -> Self {
        Self {
            cognitive_services_account,
            key,
        }
    }
}
#[doc = "Construct bigrams for frequently occurring terms while indexing. Single terms are still indexed too, with bigrams overlaid. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonGramTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The set of common words."]
    #[serde(rename = "commonWords")]
    pub common_words: Vec<String>,
    #[doc = "A value indicating whether common words matching will be case insensitive. Default is false."]
    #[serde(rename = "ignoreCase", default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[doc = "A value that indicates whether the token filter is in query mode. When in query mode, the token filter generates bigrams and then removes common words and single terms followed by a common word. Default is false."]
    #[serde(rename = "queryMode", default, skip_serializing_if = "Option::is_none")]
    pub query_mode: Option<bool>,
}
impl CommonGramTokenFilter {
    pub fn new(token_filter: TokenFilter, common_words: Vec<String>) -> Self {
        Self {
            token_filter,
            common_words,
            ignore_case: None,
            query_mode: None,
        }
    }
}
#[doc = "A skill that enables scenarios that require a Boolean operation to determine the data to assign to an output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConditionalSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
}
impl ConditionalSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self { search_indexer_skill }
    }
}
#[doc = "Defines options to control Cross-Origin Resource Sharing (CORS) for an index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsOptions {
    #[doc = "The list of origins from which JavaScript code will be granted access to your index. Can contain a list of hosts of the form {protocol}://{fully-qualified-domain-name}[:{port#}], or a single '*' to allow all origins (not recommended)."]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Vec<String>,
    #[doc = "The duration for which browsers should cache CORS preflight responses. Defaults to 5 minutes."]
    #[serde(rename = "maxAgeInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub max_age_in_seconds: Option<i64>,
}
impl CorsOptions {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self {
            allowed_origins,
            max_age_in_seconds: None,
        }
    }
}
#[doc = "Allows you to take control over the process of converting text into indexable/searchable tokens. It's a user-defined configuration consisting of a single predefined tokenizer and one or more filters. The tokenizer is responsible for breaking text into tokens, and the filters for modifying tokens emitted by the tokenizer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomAnalyzer {
    #[serde(flatten)]
    pub lexical_analyzer: LexicalAnalyzer,
    #[doc = "Defines the names of all tokenizers supported by the search engine."]
    pub tokenizer: LexicalTokenizerName,
    #[doc = "A list of token filters used to filter out or modify the tokens generated by a tokenizer. For example, you can specify a lowercase filter that converts all characters to lowercase. The filters are run in the order in which they are listed."]
    #[serde(
        rename = "tokenFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_filters: Vec<TokenFilterName>,
    #[doc = "A list of character filters used to prepare input text before it is processed by the tokenizer. For instance, they can replace certain characters or symbols. The filters are run in the order in which they are listed."]
    #[serde(
        rename = "charFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub char_filters: Vec<CharFilterName>,
}
impl CustomAnalyzer {
    pub fn new(lexical_analyzer: LexicalAnalyzer, tokenizer: LexicalTokenizerName) -> Self {
        Self {
            lexical_analyzer,
            tokenizer,
            token_filters: Vec::new(),
            char_filters: Vec::new(),
        }
    }
}
#[doc = "An object that contains information about the matches that were found, and related metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEntity {
    #[doc = "The top-level entity descriptor. Matches in the skill output will be grouped by this name, and it should represent the \"normalized\" form of the text being found."]
    pub name: String,
    #[doc = "This field can be used as a passthrough for custom metadata about the matched text(s). The value of this field will appear with every match of its entity in the skill output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "This field can be used as a passthrough for custom metadata about the matched text(s). The value of this field will appear with every match of its entity in the skill output."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "This field can be used as a passthrough for custom metadata about the matched text(s). The value of this field will appear with every match of its entity in the skill output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[doc = "This field can be used as a passthrough for custom metadata about the matched text(s). The value of this field will appear with every match of its entity in the skill output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Defaults to false. Boolean value denoting whether comparisons with the entity name should be sensitive to character casing. Sample case insensitive matches of \"Microsoft\" could be: microsoft, microSoft, MICROSOFT."]
    #[serde(rename = "caseSensitive", default, skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[doc = "Defaults to false. Boolean value denoting whether comparisons with the entity name should be sensitive to accent."]
    #[serde(rename = "accentSensitive", default, skip_serializing_if = "Option::is_none")]
    pub accent_sensitive: Option<bool>,
    #[doc = "Defaults to 0. Maximum value of 5. Denotes the acceptable number of divergent characters that would still constitute a match with the entity name. The smallest possible fuzziness for any given match is returned. For instance, if the edit distance is set to 3, \"Windows10\" would still match \"Windows\", \"Windows10\" and \"Windows 7\". When case sensitivity is set to false, case differences do NOT count towards fuzziness tolerance, but otherwise do."]
    #[serde(rename = "fuzzyEditDistance", default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_edit_distance: Option<i32>,
    #[doc = "Changes the default case sensitivity value for this entity. It be used to change the default value of all aliases caseSensitive values."]
    #[serde(rename = "defaultCaseSensitive", default, skip_serializing_if = "Option::is_none")]
    pub default_case_sensitive: Option<bool>,
    #[doc = "Changes the default accent sensitivity value for this entity. It be used to change the default value of all aliases accentSensitive values."]
    #[serde(rename = "defaultAccentSensitive", default, skip_serializing_if = "Option::is_none")]
    pub default_accent_sensitive: Option<bool>,
    #[doc = "Changes the default fuzzy edit distance value for this entity. It can be used to change the default value of all aliases fuzzyEditDistance values."]
    #[serde(rename = "defaultFuzzyEditDistance", default, skip_serializing_if = "Option::is_none")]
    pub default_fuzzy_edit_distance: Option<i32>,
    #[doc = "An array of complex objects that can be used to specify alternative spellings or synonyms to the root entity name."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aliases: Vec<CustomEntityAlias>,
}
impl CustomEntity {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            type_: None,
            subtype: None,
            id: None,
            case_sensitive: None,
            accent_sensitive: None,
            fuzzy_edit_distance: None,
            default_case_sensitive: None,
            default_accent_sensitive: None,
            default_fuzzy_edit_distance: None,
            aliases: Vec::new(),
        }
    }
}
#[doc = "A complex object that can be used to specify alternative spellings or synonyms to the root entity name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEntityAlias {
    #[doc = "The text of the alias."]
    pub text: String,
    #[doc = "Determine if the alias is case sensitive."]
    #[serde(rename = "caseSensitive", default, skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[doc = "Determine if the alias is accent sensitive."]
    #[serde(rename = "accentSensitive", default, skip_serializing_if = "Option::is_none")]
    pub accent_sensitive: Option<bool>,
    #[doc = "Determine the fuzzy edit distance of the alias."]
    #[serde(rename = "fuzzyEditDistance", default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_edit_distance: Option<i32>,
}
impl CustomEntityAlias {
    pub fn new(text: String) -> Self {
        Self {
            text,
            case_sensitive: None,
            accent_sensitive: None,
            fuzzy_edit_distance: None,
        }
    }
}
#[doc = "A skill looks for text from a custom, user-defined list of words and phrases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEntityLookupSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The language codes supported for input text by CustomEntityLookupSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<CustomEntityLookupSkillLanguage>,
    #[doc = "Path to a JSON or CSV file containing all the target text to match against. This entity definition is read at the beginning of an indexer run. Any updates to this file during an indexer run will not take effect until subsequent runs. This config must be accessible over HTTPS."]
    #[serde(rename = "entitiesDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub entities_definition_uri: Option<String>,
    #[doc = "The inline CustomEntity definition."]
    #[serde(
        rename = "inlineEntitiesDefinition",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inline_entities_definition: Vec<CustomEntity>,
    #[doc = "A global flag for CaseSensitive. If CaseSensitive is not set in CustomEntity, this value will be the default value."]
    #[serde(rename = "globalDefaultCaseSensitive", default, skip_serializing_if = "Option::is_none")]
    pub global_default_case_sensitive: Option<bool>,
    #[doc = "A global flag for AccentSensitive. If AccentSensitive is not set in CustomEntity, this value will be the default value."]
    #[serde(rename = "globalDefaultAccentSensitive", default, skip_serializing_if = "Option::is_none")]
    pub global_default_accent_sensitive: Option<bool>,
    #[doc = "A global flag for FuzzyEditDistance. If FuzzyEditDistance is not set in CustomEntity, this value will be the default value."]
    #[serde(rename = "globalDefaultFuzzyEditDistance", default, skip_serializing_if = "Option::is_none")]
    pub global_default_fuzzy_edit_distance: Option<i32>,
}
impl CustomEntityLookupSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            entities_definition_uri: None,
            inline_entities_definition: Vec::new(),
            global_default_case_sensitive: None,
            global_default_accent_sensitive: None,
            global_default_fuzzy_edit_distance: None,
        }
    }
}
#[doc = "The language codes supported for input text by CustomEntityLookupSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CustomEntityLookupSkillLanguage")]
pub enum CustomEntityLookupSkillLanguage {
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "pt")]
    Pt,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CustomEntityLookupSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CustomEntityLookupSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CustomEntityLookupSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Da => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 0u32, "da"),
            Self::De => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 1u32, "de"),
            Self::En => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 2u32, "en"),
            Self::Es => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 3u32, "es"),
            Self::Fi => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 4u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 5u32, "fr"),
            Self::It => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 6u32, "it"),
            Self::Ko => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 7u32, "ko"),
            Self::Pt => serializer.serialize_unit_variant("CustomEntityLookupSkillLanguage", 8u32, "pt"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Allows you to configure normalization for filterable, sortable, and facetable fields, which by default operate with strict matching. This is a user-defined configuration consisting of at least one or more filters, which modify the token that is stored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomNormalizer {
    #[serde(flatten)]
    pub lexical_normalizer: LexicalNormalizer,
    #[doc = "A list of token filters used to filter out or modify the input token. For example, you can specify a lowercase filter that converts all characters to lowercase. The filters are run in the order in which they are listed."]
    #[serde(
        rename = "tokenFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_filters: Vec<TokenFilterName>,
    #[doc = "A list of character filters used to prepare input text before it is processed. For instance, they can replace certain characters or symbols. The filters are run in the order in which they are listed."]
    #[serde(
        rename = "charFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub char_filters: Vec<CharFilterName>,
}
impl CustomNormalizer {
    pub fn new(lexical_normalizer: LexicalNormalizer) -> Self {
        Self {
            lexical_normalizer,
            token_filters: Vec::new(),
            char_filters: Vec::new(),
        }
    }
}
#[doc = "Specifies a user-defined vectorizer for generating the vector embedding of a query string. Integration of an external vectorizer is achieved using the custom Web API interface of a skillset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomVectorizer {
    #[serde(flatten)]
    pub vector_search_vectorizer: VectorSearchVectorizer,
    #[doc = "Specifies the properties for connecting to a user-defined vectorizer."]
    #[serde(rename = "customWebApiParameters", default, skip_serializing_if = "Option::is_none")]
    pub custom_web_api_parameters: Option<CustomWebApiParameters>,
}
impl CustomVectorizer {
    pub fn new(vector_search_vectorizer: VectorSearchVectorizer) -> Self {
        Self {
            vector_search_vectorizer,
            custom_web_api_parameters: None,
        }
    }
}
#[doc = "Specifies the properties for connecting to a user-defined vectorizer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomWebApiParameters {
    #[doc = "The URI of the Web API providing the vectorizer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "A dictionary of http request headers."]
    #[serde(rename = "httpHeaders", default, skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<WebApiHttpHeaders>,
    #[doc = "The method for the HTTP request."]
    #[serde(rename = "httpMethod", default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[doc = "The desired timeout for the request. Default is 30 seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "Applies to custom endpoints that connect to external code in an Azure function or some other application that provides the transformations. This value should be the application ID created for the function or app when it was registered with Azure Active Directory. When specified, the vectorization connects to the function or app using a managed ID (either system or user-assigned) of the search service and the access token of the function or app, using this value as the resource id for creating the scope of the access token."]
    #[serde(rename = "authResourceId", default, skip_serializing_if = "Option::is_none")]
    pub auth_resource_id: Option<String>,
    #[doc = "Abstract base type for data identities."]
    #[serde(rename = "authIdentity", default, skip_serializing_if = "Option::is_none")]
    pub auth_identity: Option<SearchIndexerDataIdentityUnion>,
}
impl CustomWebApiParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A URI fragment specifying the type of data change detection policy."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum DataChangeDetectionPolicyUnion {
    #[serde(rename = "#Microsoft.Azure.Search.HighWaterMarkChangeDetectionPolicy")]
    MicrosoftAzureSearchHighWaterMarkChangeDetectionPolicy(HighWaterMarkChangeDetectionPolicy),
    #[serde(rename = "#Microsoft.Azure.Search.SqlIntegratedChangeTrackingPolicy")]
    MicrosoftAzureSearchSqlIntegratedChangeTrackingPolicy(SqlIntegratedChangeTrackingPolicy),
}
#[doc = "A URI fragment specifying the type of data deletion detection policy."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum DataDeletionDetectionPolicyUnion {
    #[serde(rename = "#Microsoft.Azure.Search.NativeBlobSoftDeleteDeletionDetectionPolicy")]
    MicrosoftAzureSearchNativeBlobSoftDeleteDeletionDetectionPolicy(NativeBlobSoftDeleteDeletionDetectionPolicy),
    #[serde(rename = "#Microsoft.Azure.Search.SoftDeleteColumnDeletionDetectionPolicy")]
    MicrosoftAzureSearchSoftDeleteColumnDeletionDetectionPolicy(SoftDeleteColumnDeletionDetectionPolicy),
}
#[doc = "Represents credentials that can be used to connect to a datasource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceCredentials {
    #[doc = "The connection string for the datasource. Set to `<unchanged>` (with brackets) if you don't want the connection string updated. Set to `<redacted>` if you want to remove the connection string value from the datasource."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
}
impl DataSourceCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the data to extract from Azure blob storage and tells the indexer which data to extract from image content when \"imageAction\" is set to a value other than \"none\".  This applies to embedded image content in a .PDF or other application, or image files such as .jpg and .png, in Azure blobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataToExtract")]
pub enum DataToExtract {
    #[serde(rename = "storageMetadata")]
    StorageMetadata,
    #[serde(rename = "allMetadata")]
    AllMetadata,
    #[serde(rename = "contentAndMetadata")]
    ContentAndMetadata,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataToExtract {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataToExtract {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataToExtract {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StorageMetadata => serializer.serialize_unit_variant("DataToExtract", 0u32, "storageMetadata"),
            Self::AllMetadata => serializer.serialize_unit_variant("DataToExtract", 1u32, "allMetadata"),
            Self::ContentAndMetadata => serializer.serialize_unit_variant("DataToExtract", 2u32, "contentAndMetadata"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for DataToExtract {
    fn default() -> Self {
        Self::ContentAndMetadata
    }
}
#[doc = "An empty object that represents the default Azure AI service resource for a skillset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultCognitiveServicesAccount {
    #[serde(flatten)]
    pub cognitive_services_account: CognitiveServicesAccount,
}
impl DefaultCognitiveServicesAccount {
    pub fn new(cognitive_services_account: CognitiveServicesAccount) -> Self {
        Self {
            cognitive_services_account,
        }
    }
}
#[doc = "Decomposes compound words found in many Germanic languages. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DictionaryDecompounderTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The list of words to match against."]
    #[serde(rename = "wordList")]
    pub word_list: Vec<String>,
    #[doc = "The minimum word size. Only words longer than this get processed. Default is 5. Maximum is 300."]
    #[serde(rename = "minWordSize", default, skip_serializing_if = "Option::is_none")]
    pub min_word_size: Option<i32>,
    #[doc = "The minimum subword size. Only subwords longer than this are outputted. Default is 2. Maximum is 300."]
    #[serde(rename = "minSubwordSize", default, skip_serializing_if = "Option::is_none")]
    pub min_subword_size: Option<i32>,
    #[doc = "The maximum subword size. Only subwords shorter than this are outputted. Default is 15. Maximum is 300."]
    #[serde(rename = "maxSubwordSize", default, skip_serializing_if = "Option::is_none")]
    pub max_subword_size: Option<i32>,
    #[doc = "A value indicating whether to add only the longest matching subword to the output. Default is false."]
    #[serde(rename = "onlyLongestMatch", default, skip_serializing_if = "Option::is_none")]
    pub only_longest_match: Option<bool>,
}
impl DictionaryDecompounderTokenFilter {
    pub fn new(token_filter: TokenFilter, word_list: Vec<String>) -> Self {
        Self {
            token_filter,
            word_list,
            min_word_size: None,
            min_subword_size: None,
            max_subword_size: None,
            only_longest_match: None,
        }
    }
}
#[doc = "Defines a function that boosts scores based on distance from a geographic location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistanceScoringFunction {
    #[serde(flatten)]
    pub scoring_function: ScoringFunction,
    #[doc = "Provides parameter values to a distance scoring function."]
    pub distance: DistanceScoringParameters,
}
impl DistanceScoringFunction {
    pub fn new(scoring_function: ScoringFunction, distance: DistanceScoringParameters) -> Self {
        Self {
            scoring_function,
            distance,
        }
    }
}
#[doc = "Provides parameter values to a distance scoring function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistanceScoringParameters {
    #[doc = "The name of the parameter passed in search queries to specify the reference location."]
    #[serde(rename = "referencePointParameter")]
    pub reference_point_parameter: String,
    #[doc = "The distance in kilometers from the reference location where the boosting range ends."]
    #[serde(rename = "boostingDistance")]
    pub boosting_distance: f64,
}
impl DistanceScoringParameters {
    pub fn new(reference_point_parameter: String, boosting_distance: f64) -> Self {
        Self {
            reference_point_parameter,
            boosting_distance,
        }
    }
}
#[doc = "A skill that extracts content from a file within the enrichment pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentExtractionSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The parsingMode for the skill. Will be set to 'default' if not defined."]
    #[serde(rename = "parsingMode", default, skip_serializing_if = "Option::is_none")]
    pub parsing_mode: Option<String>,
    #[doc = "The type of data to be extracted for the skill. Will be set to 'contentAndMetadata' if not defined."]
    #[serde(rename = "dataToExtract", default, skip_serializing_if = "Option::is_none")]
    pub data_to_extract: Option<String>,
    #[doc = "A dictionary of configurations for the skill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}
impl DocumentExtractionSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            parsing_mode: None,
            data_to_extract: None,
            configuration: None,
        }
    }
}
#[doc = "Generates n-grams of the given size(s) starting from the front or the back of an input token. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNGramTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The minimum n-gram length. Default is 1. Must be less than the value of maxGram."]
    #[serde(rename = "minGram", default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[doc = "The maximum n-gram length. Default is 2."]
    #[serde(rename = "maxGram", default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
    #[doc = "Specifies which side of the input an n-gram should be generated from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<EdgeNGramTokenFilterSide>,
}
impl EdgeNGramTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            min_gram: None,
            max_gram: None,
            side: None,
        }
    }
}
#[doc = "Specifies which side of the input an n-gram should be generated from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EdgeNGramTokenFilterSide {
    #[serde(rename = "front")]
    Front,
    #[serde(rename = "back")]
    Back,
}
#[doc = "Generates n-grams of the given size(s) starting from the front or the back of an input token. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNGramTokenFilterV2 {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The minimum n-gram length. Default is 1. Maximum is 300. Must be less than the value of maxGram."]
    #[serde(rename = "minGram", default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[doc = "The maximum n-gram length. Default is 2. Maximum is 300."]
    #[serde(rename = "maxGram", default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
    #[doc = "Specifies which side of the input an n-gram should be generated from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<EdgeNGramTokenFilterSide>,
}
impl EdgeNGramTokenFilterV2 {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            min_gram: None,
            max_gram: None,
            side: None,
        }
    }
}
#[doc = "Tokenizes the input from an edge into n-grams of the given size(s). This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNGramTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The minimum n-gram length. Default is 1. Maximum is 300. Must be less than the value of maxGram."]
    #[serde(rename = "minGram", default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[doc = "The maximum n-gram length. Default is 2. Maximum is 300."]
    #[serde(rename = "maxGram", default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
    #[doc = "Character classes to keep in the tokens."]
    #[serde(
        rename = "tokenChars",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_chars: Vec<TokenCharacterKind>,
}
impl EdgeNGramTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            min_gram: None,
            max_gram: None,
            token_chars: Vec::new(),
        }
    }
}
#[doc = "Removes elisions. For example, \"l'avion\" (the plane) will be converted to \"avion\" (plane). This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElisionTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The set of articles to remove."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub articles: Vec<String>,
}
impl ElisionTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            articles: Vec::new(),
        }
    }
}
#[doc = "A string indicating what entity categories to return."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityCategory")]
pub enum EntityCategory {
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "quantity")]
    Quantity,
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Location => serializer.serialize_unit_variant("EntityCategory", 0u32, "location"),
            Self::Organization => serializer.serialize_unit_variant("EntityCategory", 1u32, "organization"),
            Self::Person => serializer.serialize_unit_variant("EntityCategory", 2u32, "person"),
            Self::Quantity => serializer.serialize_unit_variant("EntityCategory", 3u32, "quantity"),
            Self::Datetime => serializer.serialize_unit_variant("EntityCategory", 4u32, "datetime"),
            Self::Url => serializer.serialize_unit_variant("EntityCategory", 5u32, "url"),
            Self::Email => serializer.serialize_unit_variant("EntityCategory", 6u32, "email"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Using the Text Analytics API, extracts linked entities from text."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityLinkingSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "A value indicating which language code to use. Default is `en`."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<String>,
    #[doc = "A value between 0 and 1 that be used to only include entities whose confidence score is greater than the value specified. If not set (default), or if explicitly set to null, all entities will be included."]
    #[serde(rename = "minimumPrecision", default, skip_serializing_if = "Option::is_none")]
    pub minimum_precision: Option<f64>,
    #[doc = "The version of the model to use when calling the Text Analytics service. It will default to the latest available when not specified. We recommend you do not specify this value unless absolutely necessary."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
}
impl EntityLinkingSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            minimum_precision: None,
            model_version: None,
        }
    }
}
#[doc = "This skill is deprecated. Use the V3.EntityRecognitionSkill instead."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRecognitionSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "A list of entity categories that should be extracted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<EntityCategory>,
    #[doc = "Deprecated. The language codes supported for input text by EntityRecognitionSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<EntityRecognitionSkillLanguage>,
    #[doc = "Determines whether or not to include entities which are well known but don't conform to a pre-defined type. If this configuration is not set (default), set to null or set to false, entities which don't conform to one of the pre-defined types will not be surfaced."]
    #[serde(rename = "includeTypelessEntities", default, skip_serializing_if = "Option::is_none")]
    pub include_typeless_entities: Option<bool>,
    #[doc = "A value between 0 and 1 that be used to only include entities whose confidence score is greater than the value specified. If not set (default), or if explicitly set to null, all entities will be included."]
    #[serde(rename = "minimumPrecision", default, skip_serializing_if = "Option::is_none")]
    pub minimum_precision: Option<f64>,
}
impl EntityRecognitionSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            categories: Vec::new(),
            default_language_code: None,
            include_typeless_entities: None,
            minimum_precision: None,
        }
    }
}
#[doc = "Deprecated. The language codes supported for input text by EntityRecognitionSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityRecognitionSkillLanguage")]
pub enum EntityRecognitionSkillLanguage {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityRecognitionSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityRecognitionSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityRecognitionSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ar => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 0u32, "ar"),
            Self::Cs => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 1u32, "cs"),
            Self::ZhHans => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 2u32, "zh-Hans"),
            Self::ZhHant => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 3u32, "zh-Hant"),
            Self::Da => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 4u32, "da"),
            Self::Nl => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 5u32, "nl"),
            Self::En => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 6u32, "en"),
            Self::Fi => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 7u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 8u32, "fr"),
            Self::De => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 9u32, "de"),
            Self::El => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 10u32, "el"),
            Self::Hu => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 11u32, "hu"),
            Self::It => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 12u32, "it"),
            Self::Ja => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 13u32, "ja"),
            Self::Ko => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 14u32, "ko"),
            Self::No => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 15u32, "no"),
            Self::Pl => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 16u32, "pl"),
            Self::PtPt => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 17u32, "pt-PT"),
            Self::PtBr => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 18u32, "pt-BR"),
            Self::Ru => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 19u32, "ru"),
            Self::Es => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 20u32, "es"),
            Self::Sv => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 21u32, "sv"),
            Self::Tr => serializer.serialize_unit_variant("EntityRecognitionSkillLanguage", 22u32, "tr"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Using the Text Analytics API, extracts entities of different types from text."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRecognitionSkillV3 {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "A list of entity categories that should be extracted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<String>,
    #[doc = "A value indicating which language code to use. Default is `en`."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<String>,
    #[doc = "A value between 0 and 1 that be used to only include entities whose confidence score is greater than the value specified. If not set (default), or if explicitly set to null, all entities will be included."]
    #[serde(rename = "minimumPrecision", default, skip_serializing_if = "Option::is_none")]
    pub minimum_precision: Option<f64>,
    #[doc = "The version of the model to use when calling the Text Analytics API. It will default to the latest available when not specified. We recommend you do not specify this value unless absolutely necessary."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
}
impl EntityRecognitionSkillV3 {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            categories: Vec::new(),
            default_language_code: None,
            minimum_precision: None,
            model_version: None,
        }
    }
}
#[doc = "Specifies the environment in which the indexer should execute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExecutionEnvironment")]
pub enum ExecutionEnvironment {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "private")]
    Private,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExecutionEnvironment {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExecutionEnvironment {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExecutionEnvironment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("ExecutionEnvironment", 0u32, "standard"),
            Self::Private => serializer.serialize_unit_variant("ExecutionEnvironment", 1u32, "private"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ExecutionEnvironment {
    fn default() -> Self {
        Self::Standard
    }
}
#[doc = "Contains the parameters specific to exhaustive KNN algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExhaustiveKnnParameters {
    #[doc = "The similarity metric to use for vector comparisons."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<VectorSearchAlgorithmMetric>,
}
impl ExhaustiveKnnParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains configuration options specific to the exhaustive KNN algorithm used during querying, which will perform brute-force search across the entire vector index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExhaustiveKnnVectorSearchAlgorithmConfiguration {
    #[serde(flatten)]
    pub vector_search_algorithm_configuration: VectorSearchAlgorithmConfiguration,
    #[doc = "Contains the parameters specific to exhaustive KNN algorithm."]
    #[serde(rename = "exhaustiveKnnParameters", default, skip_serializing_if = "Option::is_none")]
    pub exhaustive_knn_parameters: Option<ExhaustiveKnnParameters>,
}
impl ExhaustiveKnnVectorSearchAlgorithmConfiguration {
    pub fn new(vector_search_algorithm_configuration: VectorSearchAlgorithmConfiguration) -> Self {
        Self {
            vector_search_algorithm_configuration,
            exhaustive_knn_parameters: None,
        }
    }
}
#[doc = "Defines a mapping between a field in a data source and a target field in an index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldMapping {
    #[doc = "The name of the field in the data source."]
    #[serde(rename = "sourceFieldName")]
    pub source_field_name: String,
    #[doc = "The name of the target field in the index. Same as the source field name by default."]
    #[serde(rename = "targetFieldName", default, skip_serializing_if = "Option::is_none")]
    pub target_field_name: Option<String>,
    #[doc = "Represents a function that transforms a value from a data source before indexing."]
    #[serde(rename = "mappingFunction", default, skip_serializing_if = "Option::is_none")]
    pub mapping_function: Option<FieldMappingFunction>,
}
impl FieldMapping {
    pub fn new(source_field_name: String) -> Self {
        Self {
            source_field_name,
            target_field_name: None,
            mapping_function: None,
        }
    }
}
#[doc = "Represents a function that transforms a value from a data source before indexing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldMappingFunction {
    #[doc = "The name of the field mapping function."]
    pub name: String,
    #[doc = "A dictionary of parameter name/value pairs to pass to the function. Each value must be of a primitive type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl FieldMappingFunction {
    pub fn new(name: String) -> Self {
        Self { name, parameters: None }
    }
}
#[doc = "Defines a function that boosts scores based on the value of a date-time field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FreshnessScoringFunction {
    #[serde(flatten)]
    pub scoring_function: ScoringFunction,
    #[doc = "Provides parameter values to a freshness scoring function."]
    pub freshness: FreshnessScoringParameters,
}
impl FreshnessScoringFunction {
    pub fn new(scoring_function: ScoringFunction, freshness: FreshnessScoringParameters) -> Self {
        Self {
            scoring_function,
            freshness,
        }
    }
}
#[doc = "Provides parameter values to a freshness scoring function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FreshnessScoringParameters {
    #[doc = "The expiration period after which boosting will stop for a particular document."]
    #[serde(rename = "boostingDuration")]
    pub boosting_duration: String,
}
impl FreshnessScoringParameters {
    pub fn new(boosting_duration: String) -> Self {
        Self { boosting_duration }
    }
}
#[doc = "Statistics for a given index. Statistics are collected periodically and are not guaranteed to always be up-to-date."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIndexStatisticsResult {
    #[doc = "The number of documents in the index."]
    #[serde(rename = "documentCount")]
    pub document_count: i64,
    #[doc = "The amount of storage in bytes consumed by the index."]
    #[serde(rename = "storageSize")]
    pub storage_size: i64,
    #[doc = "The amount of memory in bytes consumed by vectors in the index."]
    #[serde(rename = "vectorIndexSize")]
    pub vector_index_size: i64,
}
impl GetIndexStatisticsResult {
    pub fn new(document_count: i64, storage_size: i64, vector_index_size: i64) -> Self {
        Self {
            document_count,
            storage_size,
            vector_index_size,
        }
    }
}
#[doc = "Defines a data change detection policy that captures changes based on the value of a high water mark column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HighWaterMarkChangeDetectionPolicy {
    #[doc = "The name of the high water mark column."]
    #[serde(rename = "highWaterMarkColumnName")]
    pub high_water_mark_column_name: String,
}
impl HighWaterMarkChangeDetectionPolicy {
    pub fn new(high_water_mark_column_name: String) -> Self {
        Self {
            high_water_mark_column_name,
        }
    }
}
#[doc = "Contains the parameters specific to the HNSW algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HnswParameters {
    #[doc = "The number of bi-directional links created for every new element during construction. Increasing this parameter value may improve recall and reduce retrieval times for datasets with high intrinsic dimensionality at the expense of increased memory consumption and longer indexing time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m: Option<i32>,
    #[doc = "The size of the dynamic list containing the nearest neighbors, which is used during index time. Increasing this parameter may improve index quality, at the expense of increased indexing time. At a certain point, increasing this parameter leads to diminishing returns."]
    #[serde(rename = "efConstruction", default, skip_serializing_if = "Option::is_none")]
    pub ef_construction: Option<i32>,
    #[doc = "The size of the dynamic list containing the nearest neighbors, which is used during search time. Increasing this parameter may improve search results, at the expense of slower search. At a certain point, increasing this parameter leads to diminishing returns."]
    #[serde(rename = "efSearch", default, skip_serializing_if = "Option::is_none")]
    pub ef_search: Option<i32>,
    #[doc = "The similarity metric to use for vector comparisons."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<VectorSearchAlgorithmMetric>,
}
impl HnswParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains configuration options specific to the HNSW approximate nearest neighbors algorithm used during indexing and querying. The HNSW algorithm offers a tunable trade-off between search speed and accuracy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HnswVectorSearchAlgorithmConfiguration {
    #[serde(flatten)]
    pub vector_search_algorithm_configuration: VectorSearchAlgorithmConfiguration,
    #[doc = "Contains the parameters specific to the HNSW algorithm."]
    #[serde(rename = "hnswParameters", default, skip_serializing_if = "Option::is_none")]
    pub hnsw_parameters: Option<HnswParameters>,
}
impl HnswVectorSearchAlgorithmConfiguration {
    pub fn new(vector_search_algorithm_configuration: VectorSearchAlgorithmConfiguration) -> Self {
        Self {
            vector_search_algorithm_configuration,
            hnsw_parameters: None,
        }
    }
}
#[doc = "Determines how to process embedded images and image files in Azure blob storage.  Setting the \"imageAction\" configuration to any value other than \"none\" requires that a skillset also be attached to that indexer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageAction")]
pub enum ImageAction {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "generateNormalizedImages")]
    GenerateNormalizedImages,
    #[serde(rename = "generateNormalizedImagePerPage")]
    GenerateNormalizedImagePerPage,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ImageAction", 0u32, "none"),
            Self::GenerateNormalizedImages => serializer.serialize_unit_variant("ImageAction", 1u32, "generateNormalizedImages"),
            Self::GenerateNormalizedImagePerPage => {
                serializer.serialize_unit_variant("ImageAction", 2u32, "generateNormalizedImagePerPage")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ImageAction {
    fn default() -> Self {
        Self::None
    }
}
#[doc = "A skill that analyzes image files. It extracts a rich set of visual features based on the image content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageAnalysisSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The language codes supported for input by ImageAnalysisSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<ImageAnalysisSkillLanguage>,
    #[doc = "A list of visual features."]
    #[serde(
        rename = "visualFeatures",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub visual_features: Vec<VisualFeature>,
    #[doc = "A string indicating which domain-specific details to return."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ImageDetail>,
}
impl ImageAnalysisSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            visual_features: Vec::new(),
            details: Vec::new(),
        }
    }
}
#[doc = "The language codes supported for input by ImageAnalysisSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageAnalysisSkillLanguage")]
pub enum ImageAnalysisSkillLanguage {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "prs")]
    Prs,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sr-Cyrl")]
    SrCyrl,
    #[serde(rename = "sr-Latn")]
    SrLatn,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageAnalysisSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageAnalysisSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageAnalysisSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ar => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 0u32, "ar"),
            Self::Az => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 1u32, "az"),
            Self::Bg => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 2u32, "bg"),
            Self::Bs => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 3u32, "bs"),
            Self::Ca => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 4u32, "ca"),
            Self::Cs => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 5u32, "cs"),
            Self::Cy => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 6u32, "cy"),
            Self::Da => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 7u32, "da"),
            Self::De => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 8u32, "de"),
            Self::El => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 9u32, "el"),
            Self::En => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 10u32, "en"),
            Self::Es => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 11u32, "es"),
            Self::Et => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 12u32, "et"),
            Self::Eu => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 13u32, "eu"),
            Self::Fi => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 14u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 15u32, "fr"),
            Self::Ga => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 16u32, "ga"),
            Self::Gl => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 17u32, "gl"),
            Self::He => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 18u32, "he"),
            Self::Hi => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 19u32, "hi"),
            Self::Hr => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 20u32, "hr"),
            Self::Hu => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 21u32, "hu"),
            Self::Id => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 22u32, "id"),
            Self::It => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 23u32, "it"),
            Self::Ja => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 24u32, "ja"),
            Self::Kk => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 25u32, "kk"),
            Self::Ko => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 26u32, "ko"),
            Self::Lt => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 27u32, "lt"),
            Self::Lv => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 28u32, "lv"),
            Self::Mk => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 29u32, "mk"),
            Self::Ms => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 30u32, "ms"),
            Self::Nb => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 31u32, "nb"),
            Self::Nl => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 32u32, "nl"),
            Self::Pl => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 33u32, "pl"),
            Self::Prs => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 34u32, "prs"),
            Self::PtBr => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 35u32, "pt-BR"),
            Self::Pt => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 36u32, "pt"),
            Self::PtPt => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 37u32, "pt-PT"),
            Self::Ro => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 38u32, "ro"),
            Self::Ru => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 39u32, "ru"),
            Self::Sk => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 40u32, "sk"),
            Self::Sl => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 41u32, "sl"),
            Self::SrCyrl => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 42u32, "sr-Cyrl"),
            Self::SrLatn => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 43u32, "sr-Latn"),
            Self::Sv => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 44u32, "sv"),
            Self::Th => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 45u32, "th"),
            Self::Tr => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 46u32, "tr"),
            Self::Uk => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 47u32, "uk"),
            Self::Vi => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 48u32, "vi"),
            Self::Zh => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 49u32, "zh"),
            Self::ZhHans => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 50u32, "zh-Hans"),
            Self::ZhHant => serializer.serialize_unit_variant("ImageAnalysisSkillLanguage", 51u32, "zh-Hant"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A string indicating which domain-specific details to return."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageDetail")]
pub enum ImageDetail {
    #[serde(rename = "celebrities")]
    Celebrities,
    #[serde(rename = "landmarks")]
    Landmarks,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageDetail {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Celebrities => serializer.serialize_unit_variant("ImageDetail", 0u32, "celebrities"),
            Self::Landmarks => serializer.serialize_unit_variant("ImageDetail", 1u32, "landmarks"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines behavior of the index projections in relation to the rest of the indexer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IndexProjectionMode")]
pub enum IndexProjectionMode {
    #[serde(rename = "skipIndexingParentDocuments")]
    SkipIndexingParentDocuments,
    #[serde(rename = "includeIndexingParentDocuments")]
    IncludeIndexingParentDocuments,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IndexProjectionMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IndexProjectionMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IndexProjectionMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SkipIndexingParentDocuments => {
                serializer.serialize_unit_variant("IndexProjectionMode", 0u32, "skipIndexingParentDocuments")
            }
            Self::IncludeIndexingParentDocuments => {
                serializer.serialize_unit_variant("IndexProjectionMode", 1u32, "includeIndexingParentDocuments")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents all of the state that defines and dictates the indexer's current execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexerCurrentState {
    #[doc = "Represents the mode the indexer is executing in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<IndexingMode>,
    #[doc = "Change tracking state used when indexing starts on all documents in the datasource."]
    #[serde(rename = "allDocsInitialChangeTrackingState", default, skip_serializing_if = "Option::is_none")]
    pub all_docs_initial_change_tracking_state: Option<String>,
    #[doc = "Change tracking state value when indexing finishes on all documents in the datasource."]
    #[serde(rename = "allDocsFinalChangeTrackingState", default, skip_serializing_if = "Option::is_none")]
    pub all_docs_final_change_tracking_state: Option<String>,
    #[doc = "Change tracking state used when indexing starts on select, reset documents in the datasource."]
    #[serde(rename = "resetDocsInitialChangeTrackingState", default, skip_serializing_if = "Option::is_none")]
    pub reset_docs_initial_change_tracking_state: Option<String>,
    #[doc = "Change tracking state value when indexing finishes on select, reset documents in the datasource."]
    #[serde(rename = "resetDocsFinalChangeTrackingState", default, skip_serializing_if = "Option::is_none")]
    pub reset_docs_final_change_tracking_state: Option<String>,
    #[doc = "The list of document keys that have been reset. The document key is the document's unique identifier for the data in the search index. The indexer will prioritize selectively re-ingesting these keys."]
    #[serde(
        rename = "resetDocumentKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reset_document_keys: Vec<String>,
    #[doc = "The list of datasource document ids that have been reset. The datasource document id is the unique identifier for the data in the datasource. The indexer will prioritize selectively re-ingesting these ids."]
    #[serde(
        rename = "resetDatasourceDocumentIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reset_datasource_document_ids: Vec<String>,
}
impl IndexerCurrentState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the result of an individual indexer execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexerExecutionResult {
    #[doc = "Represents the status of an individual indexer execution."]
    pub status: IndexerExecutionStatus,
    #[doc = "Details the status of an individual indexer execution."]
    #[serde(rename = "statusDetail", default, skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<IndexerExecutionStatusDetail>,
    #[doc = "Represents all of the state that defines and dictates the indexer's current execution."]
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<IndexerCurrentState>,
    #[doc = "The error message indicating the top-level error, if any."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The start time of this indexer execution."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of this indexer execution, if the execution has already completed."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The item-level indexing errors."]
    pub errors: Vec<SearchIndexerError>,
    #[doc = "The item-level indexing warnings."]
    pub warnings: Vec<SearchIndexerWarning>,
    #[doc = "The number of items that were processed during this indexer execution. This includes both successfully processed items and items where indexing was attempted but failed."]
    #[serde(rename = "itemsProcessed")]
    pub items_processed: i32,
    #[doc = "The number of items that failed to be indexed during this indexer execution."]
    #[serde(rename = "itemsFailed")]
    pub items_failed: i32,
    #[doc = "Change tracking state with which an indexer execution started."]
    #[serde(rename = "initialTrackingState", default, skip_serializing_if = "Option::is_none")]
    pub initial_tracking_state: Option<String>,
    #[doc = "Change tracking state with which an indexer execution finished."]
    #[serde(rename = "finalTrackingState", default, skip_serializing_if = "Option::is_none")]
    pub final_tracking_state: Option<String>,
}
impl IndexerExecutionResult {
    pub fn new(
        status: IndexerExecutionStatus,
        errors: Vec<SearchIndexerError>,
        warnings: Vec<SearchIndexerWarning>,
        items_processed: i32,
        items_failed: i32,
    ) -> Self {
        Self {
            status,
            status_detail: None,
            current_state: None,
            error_message: None,
            start_time: None,
            end_time: None,
            errors,
            warnings,
            items_processed,
            items_failed,
            initial_tracking_state: None,
            final_tracking_state: None,
        }
    }
}
#[doc = "Represents the status of an individual indexer execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IndexerExecutionStatus {
    #[serde(rename = "transientFailure")]
    TransientFailure,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "reset")]
    Reset,
}
#[doc = "Details the status of an individual indexer execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IndexerExecutionStatusDetail")]
pub enum IndexerExecutionStatusDetail {
    #[serde(rename = "resetDocs")]
    ResetDocs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IndexerExecutionStatusDetail {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IndexerExecutionStatusDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IndexerExecutionStatusDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ResetDocs => serializer.serialize_unit_variant("IndexerExecutionStatusDetail", 0u32, "resetDocs"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the overall indexer status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IndexerStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "running")]
    Running,
}
#[doc = "Represents the mode the indexer is executing in."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IndexingMode")]
pub enum IndexingMode {
    #[serde(rename = "indexingAllDocs")]
    IndexingAllDocs,
    #[serde(rename = "indexingResetDocs")]
    IndexingResetDocs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IndexingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IndexingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IndexingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IndexingAllDocs => serializer.serialize_unit_variant("IndexingMode", 0u32, "indexingAllDocs"),
            Self::IndexingResetDocs => serializer.serialize_unit_variant("IndexingMode", 1u32, "indexingResetDocs"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents parameters for indexer execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexingParameters {
    #[doc = "The number of items that are read from the data source and indexed as a single batch in order to improve performance. The default depends on the data source type."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[doc = "The maximum number of items that can fail indexing for indexer execution to still be considered successful. -1 means no limit. Default is 0."]
    #[serde(rename = "maxFailedItems", default, skip_serializing_if = "Option::is_none")]
    pub max_failed_items: Option<i32>,
    #[doc = "The maximum number of items in a single batch that can fail indexing for the batch to still be considered successful. -1 means no limit. Default is 0."]
    #[serde(rename = "maxFailedItemsPerBatch", default, skip_serializing_if = "Option::is_none")]
    pub max_failed_items_per_batch: Option<i32>,
    #[doc = "A dictionary of indexer-specific configuration properties. Each name is the name of a specific property. Each value must be of a primitive type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<IndexingParametersConfiguration>,
}
impl IndexingParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dictionary of indexer-specific configuration properties. Each name is the name of a specific property. Each value must be of a primitive type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexingParametersConfiguration {
    #[doc = "Represents the parsing mode for indexing from an Azure blob data source."]
    #[serde(rename = "parsingMode", default, skip_serializing_if = "Option::is_none")]
    pub parsing_mode: Option<ParsingMode>,
    #[doc = "Comma-delimited list of filename extensions to ignore when processing from Azure blob storage.  For example, you could exclude \".png, .mp4\" to skip over those files during indexing."]
    #[serde(rename = "excludedFileNameExtensions", default, skip_serializing_if = "Option::is_none")]
    pub excluded_file_name_extensions: Option<String>,
    #[doc = "Comma-delimited list of filename extensions to select when processing from Azure blob storage.  For example, you could focus indexing on specific application files \".docx, .pptx, .msg\" to specifically include those file types."]
    #[serde(rename = "indexedFileNameExtensions", default, skip_serializing_if = "Option::is_none")]
    pub indexed_file_name_extensions: Option<String>,
    #[doc = "For Azure blobs, set to false if you want to continue indexing when an unsupported content type is encountered, and you don't know all the content types (file extensions) in advance."]
    #[serde(rename = "failOnUnsupportedContentType", default, skip_serializing_if = "Option::is_none")]
    pub fail_on_unsupported_content_type: Option<bool>,
    #[doc = "For Azure blobs, set to false if you want to continue indexing if a document fails indexing."]
    #[serde(rename = "failOnUnprocessableDocument", default, skip_serializing_if = "Option::is_none")]
    pub fail_on_unprocessable_document: Option<bool>,
    #[doc = "For Azure blobs, set this property to true to still index storage metadata for blob content that is too large to process. Oversized blobs are treated as errors by default. For limits on blob size, see https://docs.microsoft.com/azure/search/search-limits-quotas-capacity."]
    #[serde(
        rename = "indexStorageMetadataOnlyForOversizedDocuments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_storage_metadata_only_for_oversized_documents: Option<bool>,
    #[doc = "For CSV blobs, specifies a comma-delimited list of column headers, useful for mapping source fields to destination fields in an index."]
    #[serde(rename = "delimitedTextHeaders", default, skip_serializing_if = "Option::is_none")]
    pub delimited_text_headers: Option<String>,
    #[doc = "For CSV blobs, specifies the end-of-line single-character delimiter for CSV files where each line starts a new document (for example, \"|\")."]
    #[serde(rename = "delimitedTextDelimiter", default, skip_serializing_if = "Option::is_none")]
    pub delimited_text_delimiter: Option<String>,
    #[doc = "For CSV blobs, indicates that the first (non-blank) line of each blob contains headers."]
    #[serde(rename = "firstLineContainsHeaders", default, skip_serializing_if = "Option::is_none")]
    pub first_line_contains_headers: Option<bool>,
    #[doc = "For JSON arrays, given a structured or semi-structured document, you can specify a path to the array using this property."]
    #[serde(rename = "documentRoot", default, skip_serializing_if = "Option::is_none")]
    pub document_root: Option<String>,
    #[doc = "Specifies the data to extract from Azure blob storage and tells the indexer which data to extract from image content when \"imageAction\" is set to a value other than \"none\".  This applies to embedded image content in a .PDF or other application, or image files such as .jpg and .png, in Azure blobs."]
    #[serde(rename = "dataToExtract", default, skip_serializing_if = "Option::is_none")]
    pub data_to_extract: Option<DataToExtract>,
    #[doc = "Determines how to process embedded images and image files in Azure blob storage.  Setting the \"imageAction\" configuration to any value other than \"none\" requires that a skillset also be attached to that indexer."]
    #[serde(rename = "imageAction", default, skip_serializing_if = "Option::is_none")]
    pub image_action: Option<ImageAction>,
    #[doc = "If true, will create a path //document//file_data that is an object representing the original file data downloaded from your blob data source.  This allows you to pass the original file data to a custom skill for processing within the enrichment pipeline, or to the Document Extraction skill."]
    #[serde(rename = "allowSkillsetToReadFileData", default, skip_serializing_if = "Option::is_none")]
    pub allow_skillset_to_read_file_data: Option<bool>,
    #[doc = "Determines algorithm for text extraction from PDF files in Azure blob storage."]
    #[serde(rename = "pdfTextRotationAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub pdf_text_rotation_algorithm: Option<PdfTextRotationAlgorithm>,
    #[doc = "Specifies the environment in which the indexer should execute."]
    #[serde(rename = "executionEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub execution_environment: Option<ExecutionEnvironment>,
    #[doc = "Increases the timeout beyond the 5-minute default for Azure SQL database data sources, specified in the format \"hh:mm:ss\"."]
    #[serde(rename = "queryTimeout", default, skip_serializing_if = "Option::is_none")]
    pub query_timeout: Option<String>,
}
impl IndexingParametersConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a schedule for indexer execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexingSchedule {
    #[doc = "The interval of time between indexer executions."]
    pub interval: String,
    #[doc = "The time when an indexer should start running."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl IndexingSchedule {
    pub fn new(interval: String) -> Self {
        Self {
            interval,
            start_time: None,
        }
    }
}
#[doc = "Input field mapping for a skill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputFieldMappingEntry {
    #[doc = "The name of the input."]
    pub name: String,
    #[doc = "The source of the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The source context used for selecting recursive inputs."]
    #[serde(rename = "sourceContext", default, skip_serializing_if = "Option::is_none")]
    pub source_context: Option<String>,
    #[doc = "The recursive inputs used when creating a complex type."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<InputFieldMappingEntry>,
}
impl InputFieldMappingEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            source: None,
            source_context: None,
            inputs: Vec::new(),
        }
    }
}
#[doc = "A token filter that only keeps tokens with text contained in a specified list of words. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeepTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The list of words to keep."]
    #[serde(rename = "keepWords")]
    pub keep_words: Vec<String>,
    #[doc = "A value indicating whether to lower case all words first. Default is false."]
    #[serde(rename = "keepWordsCase", default, skip_serializing_if = "Option::is_none")]
    pub keep_words_case: Option<bool>,
}
impl KeepTokenFilter {
    pub fn new(token_filter: TokenFilter, keep_words: Vec<String>) -> Self {
        Self {
            token_filter,
            keep_words,
            keep_words_case: None,
        }
    }
}
#[doc = "A skill that uses text analytics for key phrase extraction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyPhraseExtractionSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The language codes supported for input text by KeyPhraseExtractionSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<KeyPhraseExtractionSkillLanguage>,
    #[doc = "A number indicating how many key phrases to return. If absent, all identified key phrases will be returned."]
    #[serde(rename = "maxKeyPhraseCount", default, skip_serializing_if = "Option::is_none")]
    pub max_key_phrase_count: Option<i32>,
    #[doc = "The version of the model to use when calling the Text Analytics service. It will default to the latest available when not specified. We recommend you do not specify this value unless absolutely necessary."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
}
impl KeyPhraseExtractionSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            max_key_phrase_count: None,
            model_version: None,
        }
    }
}
#[doc = "The language codes supported for input text by KeyPhraseExtractionSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyPhraseExtractionSkillLanguage")]
pub enum KeyPhraseExtractionSkillLanguage {
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "sv")]
    Sv,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyPhraseExtractionSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyPhraseExtractionSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyPhraseExtractionSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Da => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 0u32, "da"),
            Self::Nl => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 1u32, "nl"),
            Self::En => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 2u32, "en"),
            Self::Fi => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 3u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 4u32, "fr"),
            Self::De => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 5u32, "de"),
            Self::It => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 6u32, "it"),
            Self::Ja => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 7u32, "ja"),
            Self::Ko => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 8u32, "ko"),
            Self::No => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 9u32, "no"),
            Self::Pl => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 10u32, "pl"),
            Self::PtPt => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 11u32, "pt-PT"),
            Self::PtBr => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 12u32, "pt-BR"),
            Self::Ru => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 13u32, "ru"),
            Self::Es => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 14u32, "es"),
            Self::Sv => serializer.serialize_unit_variant("KeyPhraseExtractionSkillLanguage", 15u32, "sv"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Marks terms as keywords. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeywordMarkerTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A list of words to mark as keywords."]
    pub keywords: Vec<String>,
    #[doc = "A value indicating whether to ignore case. If true, all words are converted to lower case first. Default is false."]
    #[serde(rename = "ignoreCase", default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
}
impl KeywordMarkerTokenFilter {
    pub fn new(token_filter: TokenFilter, keywords: Vec<String>) -> Self {
        Self {
            token_filter,
            keywords,
            ignore_case: None,
        }
    }
}
#[doc = "Emits the entire input as a single token. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeywordTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The read buffer size in bytes. Default is 256."]
    #[serde(rename = "bufferSize", default, skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
}
impl KeywordTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            buffer_size: None,
        }
    }
}
#[doc = "Emits the entire input as a single token. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeywordTokenizerV2 {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Default is 256. Tokens longer than the maximum length are split. The maximum token length that can be used is 300 characters."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
}
impl KeywordTokenizerV2 {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
        }
    }
}
#[doc = "A skill that detects the language of input text and reports a single language code for every document submitted on the request. The language code is paired with a score indicating the confidence of the analysis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LanguageDetectionSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "A country code to use as a hint to the language detection model if it cannot disambiguate the language."]
    #[serde(rename = "defaultCountryHint", default, skip_serializing_if = "Option::is_none")]
    pub default_country_hint: Option<String>,
    #[doc = "The version of the model to use when calling the Text Analytics service. It will default to the latest available when not specified. We recommend you do not specify this value unless absolutely necessary."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
}
impl LanguageDetectionSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_country_hint: None,
            model_version: None,
        }
    }
}
#[doc = "Removes words that are too long or too short. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LengthTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The minimum length in characters. Default is 0. Maximum is 300. Must be less than the value of max."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[doc = "The maximum length in characters. Default and maximum is 300."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}
impl LengthTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            min: None,
            max: None,
        }
    }
}
#[doc = "Base type for analyzers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LexicalAnalyzer {
    #[doc = "The name of the analyzer. It must only contain letters, digits, spaces, dashes or underscores, can only start and end with alphanumeric characters, and is limited to 128 characters."]
    pub name: String,
}
impl LexicalAnalyzer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "A URI fragment specifying the type of analyzer. "]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum LexicalAnalyzerUnion {
    #[serde(rename = "#Microsoft.Azure.Search.CustomAnalyzer")]
    MicrosoftAzureSearchCustomAnalyzer(CustomAnalyzer),
    #[serde(rename = "#Microsoft.Azure.Search.StandardAnalyzer")]
    MicrosoftAzureSearchStandardAnalyzer(LuceneStandardAnalyzer),
    #[serde(rename = "#Microsoft.Azure.Search.PatternAnalyzer")]
    MicrosoftAzureSearchPatternAnalyzer(PatternAnalyzer),
    #[serde(rename = "#Microsoft.Azure.Search.StopAnalyzer")]
    MicrosoftAzureSearchStopAnalyzer(StopAnalyzer),
}
#[doc = "Defines the names of all text analyzers supported by the search engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LexicalAnalyzerName")]
pub enum LexicalAnalyzerName {
    #[serde(rename = "ar.microsoft")]
    ArMicrosoft,
    #[serde(rename = "ar.lucene")]
    ArLucene,
    #[serde(rename = "hy.lucene")]
    HyLucene,
    #[serde(rename = "bn.microsoft")]
    BnMicrosoft,
    #[serde(rename = "eu.lucene")]
    EuLucene,
    #[serde(rename = "bg.microsoft")]
    BgMicrosoft,
    #[serde(rename = "bg.lucene")]
    BgLucene,
    #[serde(rename = "ca.microsoft")]
    CaMicrosoft,
    #[serde(rename = "ca.lucene")]
    CaLucene,
    #[serde(rename = "zh-Hans.microsoft")]
    ZhHansMicrosoft,
    #[serde(rename = "zh-Hans.lucene")]
    ZhHansLucene,
    #[serde(rename = "zh-Hant.microsoft")]
    ZhHantMicrosoft,
    #[serde(rename = "zh-Hant.lucene")]
    ZhHantLucene,
    #[serde(rename = "hr.microsoft")]
    HrMicrosoft,
    #[serde(rename = "cs.microsoft")]
    CsMicrosoft,
    #[serde(rename = "cs.lucene")]
    CsLucene,
    #[serde(rename = "da.microsoft")]
    DaMicrosoft,
    #[serde(rename = "da.lucene")]
    DaLucene,
    #[serde(rename = "nl.microsoft")]
    NlMicrosoft,
    #[serde(rename = "nl.lucene")]
    NlLucene,
    #[serde(rename = "en.microsoft")]
    EnMicrosoft,
    #[serde(rename = "en.lucene")]
    EnLucene,
    #[serde(rename = "et.microsoft")]
    EtMicrosoft,
    #[serde(rename = "fi.microsoft")]
    FiMicrosoft,
    #[serde(rename = "fi.lucene")]
    FiLucene,
    #[serde(rename = "fr.microsoft")]
    FrMicrosoft,
    #[serde(rename = "fr.lucene")]
    FrLucene,
    #[serde(rename = "gl.lucene")]
    GlLucene,
    #[serde(rename = "de.microsoft")]
    DeMicrosoft,
    #[serde(rename = "de.lucene")]
    DeLucene,
    #[serde(rename = "el.microsoft")]
    ElMicrosoft,
    #[serde(rename = "el.lucene")]
    ElLucene,
    #[serde(rename = "gu.microsoft")]
    GuMicrosoft,
    #[serde(rename = "he.microsoft")]
    HeMicrosoft,
    #[serde(rename = "hi.microsoft")]
    HiMicrosoft,
    #[serde(rename = "hi.lucene")]
    HiLucene,
    #[serde(rename = "hu.microsoft")]
    HuMicrosoft,
    #[serde(rename = "hu.lucene")]
    HuLucene,
    #[serde(rename = "is.microsoft")]
    IsMicrosoft,
    #[serde(rename = "id.microsoft")]
    IdMicrosoft,
    #[serde(rename = "id.lucene")]
    IdLucene,
    #[serde(rename = "ga.lucene")]
    GaLucene,
    #[serde(rename = "it.microsoft")]
    ItMicrosoft,
    #[serde(rename = "it.lucene")]
    ItLucene,
    #[serde(rename = "ja.microsoft")]
    JaMicrosoft,
    #[serde(rename = "ja.lucene")]
    JaLucene,
    #[serde(rename = "kn.microsoft")]
    KnMicrosoft,
    #[serde(rename = "ko.microsoft")]
    KoMicrosoft,
    #[serde(rename = "ko.lucene")]
    KoLucene,
    #[serde(rename = "lv.microsoft")]
    LvMicrosoft,
    #[serde(rename = "lv.lucene")]
    LvLucene,
    #[serde(rename = "lt.microsoft")]
    LtMicrosoft,
    #[serde(rename = "ml.microsoft")]
    MlMicrosoft,
    #[serde(rename = "ms.microsoft")]
    MsMicrosoft,
    #[serde(rename = "mr.microsoft")]
    MrMicrosoft,
    #[serde(rename = "nb.microsoft")]
    NbMicrosoft,
    #[serde(rename = "no.lucene")]
    NoLucene,
    #[serde(rename = "fa.lucene")]
    FaLucene,
    #[serde(rename = "pl.microsoft")]
    PlMicrosoft,
    #[serde(rename = "pl.lucene")]
    PlLucene,
    #[serde(rename = "pt-BR.microsoft")]
    PtBrMicrosoft,
    #[serde(rename = "pt-BR.lucene")]
    PtBrLucene,
    #[serde(rename = "pt-PT.microsoft")]
    PtPtMicrosoft,
    #[serde(rename = "pt-PT.lucene")]
    PtPtLucene,
    #[serde(rename = "pa.microsoft")]
    PaMicrosoft,
    #[serde(rename = "ro.microsoft")]
    RoMicrosoft,
    #[serde(rename = "ro.lucene")]
    RoLucene,
    #[serde(rename = "ru.microsoft")]
    RuMicrosoft,
    #[serde(rename = "ru.lucene")]
    RuLucene,
    #[serde(rename = "sr-cyrillic.microsoft")]
    SrCyrillicMicrosoft,
    #[serde(rename = "sr-latin.microsoft")]
    SrLatinMicrosoft,
    #[serde(rename = "sk.microsoft")]
    SkMicrosoft,
    #[serde(rename = "sl.microsoft")]
    SlMicrosoft,
    #[serde(rename = "es.microsoft")]
    EsMicrosoft,
    #[serde(rename = "es.lucene")]
    EsLucene,
    #[serde(rename = "sv.microsoft")]
    SvMicrosoft,
    #[serde(rename = "sv.lucene")]
    SvLucene,
    #[serde(rename = "ta.microsoft")]
    TaMicrosoft,
    #[serde(rename = "te.microsoft")]
    TeMicrosoft,
    #[serde(rename = "th.microsoft")]
    ThMicrosoft,
    #[serde(rename = "th.lucene")]
    ThLucene,
    #[serde(rename = "tr.microsoft")]
    TrMicrosoft,
    #[serde(rename = "tr.lucene")]
    TrLucene,
    #[serde(rename = "uk.microsoft")]
    UkMicrosoft,
    #[serde(rename = "ur.microsoft")]
    UrMicrosoft,
    #[serde(rename = "vi.microsoft")]
    ViMicrosoft,
    #[serde(rename = "standard.lucene")]
    StandardLucene,
    #[serde(rename = "standardasciifolding.lucene")]
    StandardasciifoldingLucene,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "pattern")]
    Pattern,
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "whitespace")]
    Whitespace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LexicalAnalyzerName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LexicalAnalyzerName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LexicalAnalyzerName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ArMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 0u32, "ar.microsoft"),
            Self::ArLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 1u32, "ar.lucene"),
            Self::HyLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 2u32, "hy.lucene"),
            Self::BnMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 3u32, "bn.microsoft"),
            Self::EuLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 4u32, "eu.lucene"),
            Self::BgMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 5u32, "bg.microsoft"),
            Self::BgLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 6u32, "bg.lucene"),
            Self::CaMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 7u32, "ca.microsoft"),
            Self::CaLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 8u32, "ca.lucene"),
            Self::ZhHansMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 9u32, "zh-Hans.microsoft"),
            Self::ZhHansLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 10u32, "zh-Hans.lucene"),
            Self::ZhHantMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 11u32, "zh-Hant.microsoft"),
            Self::ZhHantLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 12u32, "zh-Hant.lucene"),
            Self::HrMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 13u32, "hr.microsoft"),
            Self::CsMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 14u32, "cs.microsoft"),
            Self::CsLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 15u32, "cs.lucene"),
            Self::DaMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 16u32, "da.microsoft"),
            Self::DaLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 17u32, "da.lucene"),
            Self::NlMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 18u32, "nl.microsoft"),
            Self::NlLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 19u32, "nl.lucene"),
            Self::EnMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 20u32, "en.microsoft"),
            Self::EnLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 21u32, "en.lucene"),
            Self::EtMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 22u32, "et.microsoft"),
            Self::FiMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 23u32, "fi.microsoft"),
            Self::FiLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 24u32, "fi.lucene"),
            Self::FrMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 25u32, "fr.microsoft"),
            Self::FrLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 26u32, "fr.lucene"),
            Self::GlLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 27u32, "gl.lucene"),
            Self::DeMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 28u32, "de.microsoft"),
            Self::DeLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 29u32, "de.lucene"),
            Self::ElMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 30u32, "el.microsoft"),
            Self::ElLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 31u32, "el.lucene"),
            Self::GuMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 32u32, "gu.microsoft"),
            Self::HeMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 33u32, "he.microsoft"),
            Self::HiMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 34u32, "hi.microsoft"),
            Self::HiLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 35u32, "hi.lucene"),
            Self::HuMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 36u32, "hu.microsoft"),
            Self::HuLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 37u32, "hu.lucene"),
            Self::IsMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 38u32, "is.microsoft"),
            Self::IdMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 39u32, "id.microsoft"),
            Self::IdLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 40u32, "id.lucene"),
            Self::GaLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 41u32, "ga.lucene"),
            Self::ItMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 42u32, "it.microsoft"),
            Self::ItLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 43u32, "it.lucene"),
            Self::JaMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 44u32, "ja.microsoft"),
            Self::JaLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 45u32, "ja.lucene"),
            Self::KnMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 46u32, "kn.microsoft"),
            Self::KoMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 47u32, "ko.microsoft"),
            Self::KoLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 48u32, "ko.lucene"),
            Self::LvMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 49u32, "lv.microsoft"),
            Self::LvLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 50u32, "lv.lucene"),
            Self::LtMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 51u32, "lt.microsoft"),
            Self::MlMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 52u32, "ml.microsoft"),
            Self::MsMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 53u32, "ms.microsoft"),
            Self::MrMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 54u32, "mr.microsoft"),
            Self::NbMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 55u32, "nb.microsoft"),
            Self::NoLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 56u32, "no.lucene"),
            Self::FaLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 57u32, "fa.lucene"),
            Self::PlMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 58u32, "pl.microsoft"),
            Self::PlLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 59u32, "pl.lucene"),
            Self::PtBrMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 60u32, "pt-BR.microsoft"),
            Self::PtBrLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 61u32, "pt-BR.lucene"),
            Self::PtPtMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 62u32, "pt-PT.microsoft"),
            Self::PtPtLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 63u32, "pt-PT.lucene"),
            Self::PaMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 64u32, "pa.microsoft"),
            Self::RoMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 65u32, "ro.microsoft"),
            Self::RoLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 66u32, "ro.lucene"),
            Self::RuMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 67u32, "ru.microsoft"),
            Self::RuLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 68u32, "ru.lucene"),
            Self::SrCyrillicMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 69u32, "sr-cyrillic.microsoft"),
            Self::SrLatinMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 70u32, "sr-latin.microsoft"),
            Self::SkMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 71u32, "sk.microsoft"),
            Self::SlMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 72u32, "sl.microsoft"),
            Self::EsMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 73u32, "es.microsoft"),
            Self::EsLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 74u32, "es.lucene"),
            Self::SvMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 75u32, "sv.microsoft"),
            Self::SvLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 76u32, "sv.lucene"),
            Self::TaMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 77u32, "ta.microsoft"),
            Self::TeMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 78u32, "te.microsoft"),
            Self::ThMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 79u32, "th.microsoft"),
            Self::ThLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 80u32, "th.lucene"),
            Self::TrMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 81u32, "tr.microsoft"),
            Self::TrLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 82u32, "tr.lucene"),
            Self::UkMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 83u32, "uk.microsoft"),
            Self::UrMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 84u32, "ur.microsoft"),
            Self::ViMicrosoft => serializer.serialize_unit_variant("LexicalAnalyzerName", 85u32, "vi.microsoft"),
            Self::StandardLucene => serializer.serialize_unit_variant("LexicalAnalyzerName", 86u32, "standard.lucene"),
            Self::StandardasciifoldingLucene => {
                serializer.serialize_unit_variant("LexicalAnalyzerName", 87u32, "standardasciifolding.lucene")
            }
            Self::Keyword => serializer.serialize_unit_variant("LexicalAnalyzerName", 88u32, "keyword"),
            Self::Pattern => serializer.serialize_unit_variant("LexicalAnalyzerName", 89u32, "pattern"),
            Self::Simple => serializer.serialize_unit_variant("LexicalAnalyzerName", 90u32, "simple"),
            Self::Stop => serializer.serialize_unit_variant("LexicalAnalyzerName", 91u32, "stop"),
            Self::Whitespace => serializer.serialize_unit_variant("LexicalAnalyzerName", 92u32, "whitespace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base type for normalizers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LexicalNormalizer {
    #[doc = "The name of the normalizer. It must only contain letters, digits, spaces, dashes or underscores, can only start and end with alphanumeric characters, and is limited to 128 characters. It cannot end in '.microsoft' nor '.lucene', nor be named 'asciifolding', 'standard', 'lowercase', 'uppercase', or 'elision'."]
    pub name: String,
}
impl LexicalNormalizer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "A URI fragment specifying the type of normalizer."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum LexicalNormalizerUnion {
    #[serde(rename = "#Microsoft.Azure.Search.CustomNormalizer")]
    MicrosoftAzureSearchCustomNormalizer(CustomNormalizer),
}
#[doc = "Defines the names of all text normalizers supported by the search engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LexicalNormalizerName")]
pub enum LexicalNormalizerName {
    #[serde(rename = "asciifolding")]
    Asciifolding,
    #[serde(rename = "elision")]
    Elision,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LexicalNormalizerName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LexicalNormalizerName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LexicalNormalizerName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Asciifolding => serializer.serialize_unit_variant("LexicalNormalizerName", 0u32, "asciifolding"),
            Self::Elision => serializer.serialize_unit_variant("LexicalNormalizerName", 1u32, "elision"),
            Self::Lowercase => serializer.serialize_unit_variant("LexicalNormalizerName", 2u32, "lowercase"),
            Self::Standard => serializer.serialize_unit_variant("LexicalNormalizerName", 3u32, "standard"),
            Self::Uppercase => serializer.serialize_unit_variant("LexicalNormalizerName", 4u32, "uppercase"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Base type for tokenizers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LexicalTokenizer {
    #[doc = "The name of the tokenizer. It must only contain letters, digits, spaces, dashes or underscores, can only start and end with alphanumeric characters, and is limited to 128 characters."]
    pub name: String,
}
impl LexicalTokenizer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "A URI fragment specifying the type of tokenizer."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum LexicalTokenizerUnion {
    #[serde(rename = "#Microsoft.Azure.Search.ClassicTokenizer")]
    MicrosoftAzureSearchClassicTokenizer(ClassicTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.EdgeNGramTokenizer")]
    MicrosoftAzureSearchEdgeNGramTokenizer(EdgeNGramTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.KeywordTokenizer")]
    MicrosoftAzureSearchKeywordTokenizer(KeywordTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.KeywordTokenizerV2")]
    MicrosoftAzureSearchKeywordTokenizerV2(KeywordTokenizerV2),
    #[serde(rename = "#Microsoft.Azure.Search.StandardTokenizer")]
    MicrosoftAzureSearchStandardTokenizer(LuceneStandardTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.StandardTokenizerV2")]
    MicrosoftAzureSearchStandardTokenizerV2(LuceneStandardTokenizerV2),
    #[serde(rename = "#Microsoft.Azure.Search.MicrosoftLanguageStemmingTokenizer")]
    MicrosoftAzureSearchMicrosoftLanguageStemmingTokenizer(MicrosoftLanguageStemmingTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.MicrosoftLanguageTokenizer")]
    MicrosoftAzureSearchMicrosoftLanguageTokenizer(MicrosoftLanguageTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.NGramTokenizer")]
    MicrosoftAzureSearchNGramTokenizer(NGramTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.PathHierarchyTokenizerV2")]
    MicrosoftAzureSearchPathHierarchyTokenizerV2(PathHierarchyTokenizerV2),
    #[serde(rename = "#Microsoft.Azure.Search.PatternTokenizer")]
    MicrosoftAzureSearchPatternTokenizer(PatternTokenizer),
    #[serde(rename = "#Microsoft.Azure.Search.UaxUrlEmailTokenizer")]
    MicrosoftAzureSearchUaxUrlEmailTokenizer(UaxUrlEmailTokenizer),
}
#[doc = "Defines the names of all tokenizers supported by the search engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LexicalTokenizerName")]
pub enum LexicalTokenizerName {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "edgeNGram")]
    EdgeNGram,
    #[serde(rename = "keyword_v2")]
    KeywordV2,
    #[serde(rename = "letter")]
    Letter,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "microsoft_language_tokenizer")]
    MicrosoftLanguageTokenizer,
    #[serde(rename = "microsoft_language_stemming_tokenizer")]
    MicrosoftLanguageStemmingTokenizer,
    #[serde(rename = "nGram")]
    NGram,
    #[serde(rename = "path_hierarchy_v2")]
    PathHierarchyV2,
    #[serde(rename = "pattern")]
    Pattern,
    #[serde(rename = "standard_v2")]
    StandardV2,
    #[serde(rename = "uax_url_email")]
    UaxUrlEmail,
    #[serde(rename = "whitespace")]
    Whitespace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LexicalTokenizerName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LexicalTokenizerName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LexicalTokenizerName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Classic => serializer.serialize_unit_variant("LexicalTokenizerName", 0u32, "classic"),
            Self::EdgeNGram => serializer.serialize_unit_variant("LexicalTokenizerName", 1u32, "edgeNGram"),
            Self::KeywordV2 => serializer.serialize_unit_variant("LexicalTokenizerName", 2u32, "keyword_v2"),
            Self::Letter => serializer.serialize_unit_variant("LexicalTokenizerName", 3u32, "letter"),
            Self::Lowercase => serializer.serialize_unit_variant("LexicalTokenizerName", 4u32, "lowercase"),
            Self::MicrosoftLanguageTokenizer => {
                serializer.serialize_unit_variant("LexicalTokenizerName", 5u32, "microsoft_language_tokenizer")
            }
            Self::MicrosoftLanguageStemmingTokenizer => {
                serializer.serialize_unit_variant("LexicalTokenizerName", 6u32, "microsoft_language_stemming_tokenizer")
            }
            Self::NGram => serializer.serialize_unit_variant("LexicalTokenizerName", 7u32, "nGram"),
            Self::PathHierarchyV2 => serializer.serialize_unit_variant("LexicalTokenizerName", 8u32, "path_hierarchy_v2"),
            Self::Pattern => serializer.serialize_unit_variant("LexicalTokenizerName", 9u32, "pattern"),
            Self::StandardV2 => serializer.serialize_unit_variant("LexicalTokenizerName", 10u32, "standard_v2"),
            Self::UaxUrlEmail => serializer.serialize_unit_variant("LexicalTokenizerName", 11u32, "uax_url_email"),
            Self::Whitespace => serializer.serialize_unit_variant("LexicalTokenizerName", 12u32, "whitespace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Limits the number of tokens while indexing. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The maximum number of tokens to produce. Default is 1."]
    #[serde(rename = "maxTokenCount", default, skip_serializing_if = "Option::is_none")]
    pub max_token_count: Option<i32>,
    #[doc = "A value indicating whether all tokens from the input must be consumed even if maxTokenCount is reached. Default is false."]
    #[serde(rename = "consumeAllTokens", default, skip_serializing_if = "Option::is_none")]
    pub consume_all_tokens: Option<bool>,
}
impl LimitTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            max_token_count: None,
            consume_all_tokens: None,
        }
    }
}
#[doc = "Response from a List Aliases request. If successful, it includes the associated index mappings for all aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAliasesResult {
    #[doc = "The aliases in the Search service."]
    pub value: Vec<SearchAlias>,
}
impl azure_core::Continuable for ListAliasesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListAliasesResult {
    pub fn new(value: Vec<SearchAlias>) -> Self {
        Self { value }
    }
}
#[doc = "Response from a List Datasources request. If successful, it includes the full definitions of all datasources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListDataSourcesResult {
    #[doc = "The datasources in the Search service."]
    pub value: Vec<SearchIndexerDataSource>,
}
impl ListDataSourcesResult {
    pub fn new(value: Vec<SearchIndexerDataSource>) -> Self {
        Self { value }
    }
}
#[doc = "Response from a List Indexers request. If successful, it includes the full definitions of all indexers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListIndexersResult {
    #[doc = "The indexers in the Search service."]
    pub value: Vec<SearchIndexer>,
}
impl ListIndexersResult {
    pub fn new(value: Vec<SearchIndexer>) -> Self {
        Self { value }
    }
}
#[doc = "Response from a List Indexes request. If successful, it includes the full definitions of all indexes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListIndexesResult {
    #[doc = "The indexes in the Search service."]
    pub value: Vec<SearchIndex>,
}
impl azure_core::Continuable for ListIndexesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListIndexesResult {
    pub fn new(value: Vec<SearchIndex>) -> Self {
        Self { value }
    }
}
#[doc = "Response from a list skillset request. If successful, it includes the full definitions of all skillsets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListSkillsetsResult {
    #[doc = "The skillsets defined in the Search service."]
    pub value: Vec<SearchIndexerSkillset>,
}
impl ListSkillsetsResult {
    pub fn new(value: Vec<SearchIndexerSkillset>) -> Self {
        Self { value }
    }
}
#[doc = "Response from a List SynonymMaps request. If successful, it includes the full definitions of all synonym maps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListSynonymMapsResult {
    #[doc = "The synonym maps in the Search service."]
    pub value: Vec<SynonymMap>,
}
impl ListSynonymMapsResult {
    pub fn new(value: Vec<SynonymMap>) -> Self {
        Self { value }
    }
}
#[doc = "Standard Apache Lucene analyzer; Composed of the standard tokenizer, lowercase filter and stop filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LuceneStandardAnalyzer {
    #[serde(flatten)]
    pub lexical_analyzer: LexicalAnalyzer,
    #[doc = "The maximum token length. Default is 255. Tokens longer than the maximum length are split. The maximum token length that can be used is 300 characters."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    #[doc = "A list of stopwords."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stopwords: Vec<String>,
}
impl LuceneStandardAnalyzer {
    pub fn new(lexical_analyzer: LexicalAnalyzer) -> Self {
        Self {
            lexical_analyzer,
            max_token_length: None,
            stopwords: Vec::new(),
        }
    }
}
#[doc = "Breaks text following the Unicode Text Segmentation rules. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LuceneStandardTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Default is 255. Tokens longer than the maximum length are split."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
}
impl LuceneStandardTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
        }
    }
}
#[doc = "Breaks text following the Unicode Text Segmentation rules. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LuceneStandardTokenizerV2 {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Default is 255. Tokens longer than the maximum length are split. The maximum token length that can be used is 300 characters."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
}
impl LuceneStandardTokenizerV2 {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
        }
    }
}
#[doc = "Defines a function that boosts scores based on the magnitude of a numeric field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MagnitudeScoringFunction {
    #[serde(flatten)]
    pub scoring_function: ScoringFunction,
    #[doc = "Provides parameter values to a magnitude scoring function."]
    pub magnitude: MagnitudeScoringParameters,
}
impl MagnitudeScoringFunction {
    pub fn new(scoring_function: ScoringFunction, magnitude: MagnitudeScoringParameters) -> Self {
        Self {
            scoring_function,
            magnitude,
        }
    }
}
#[doc = "Provides parameter values to a magnitude scoring function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MagnitudeScoringParameters {
    #[doc = "The field value at which boosting starts."]
    #[serde(rename = "boostingRangeStart")]
    pub boosting_range_start: f64,
    #[doc = "The field value at which boosting ends."]
    #[serde(rename = "boostingRangeEnd")]
    pub boosting_range_end: f64,
    #[doc = "A value indicating whether to apply a constant boost for field values beyond the range end value; default is false."]
    #[serde(rename = "constantBoostBeyondRange", default, skip_serializing_if = "Option::is_none")]
    pub constant_boost_beyond_range: Option<bool>,
}
impl MagnitudeScoringParameters {
    pub fn new(boosting_range_start: f64, boosting_range_end: f64) -> Self {
        Self {
            boosting_range_start,
            boosting_range_end,
            constant_boost_beyond_range: None,
        }
    }
}
#[doc = "A character filter that applies mappings defined with the mappings option. Matching is greedy (longest pattern matching at a given point wins). Replacement is allowed to be the empty string. This character filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MappingCharFilter {
    #[serde(flatten)]
    pub char_filter: CharFilter,
    #[doc = "A list of mappings of the following format: \"a=>b\" (all occurrences of the character \"a\" will be replaced with character \"b\")."]
    pub mappings: Vec<String>,
}
impl MappingCharFilter {
    pub fn new(char_filter: CharFilter, mappings: Vec<String>) -> Self {
        Self { char_filter, mappings }
    }
}
#[doc = "A skill for merging two or more strings into a single unified string, with an optional user-defined delimiter separating each component part."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MergeSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The tag indicates the start of the merged text. By default, the tag is an empty space."]
    #[serde(rename = "insertPreTag", default, skip_serializing_if = "Option::is_none")]
    pub insert_pre_tag: Option<String>,
    #[doc = "The tag indicates the end of the merged text. By default, the tag is an empty space."]
    #[serde(rename = "insertPostTag", default, skip_serializing_if = "Option::is_none")]
    pub insert_post_tag: Option<String>,
}
impl MergeSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            insert_pre_tag: None,
            insert_post_tag: None,
        }
    }
}
#[doc = "Divides text using language-specific rules and reduces words to their base forms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftLanguageStemmingTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Tokens longer than the maximum length are split. Maximum token length that can be used is 300 characters. Tokens longer than 300 characters are first split into tokens of length 300 and then each of those tokens is split based on the max token length set. Default is 255."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    #[doc = "A value indicating how the tokenizer is used. Set to true if used as the search tokenizer, set to false if used as the indexing tokenizer. Default is false."]
    #[serde(rename = "isSearchTokenizer", default, skip_serializing_if = "Option::is_none")]
    pub is_search_tokenizer: Option<bool>,
    #[doc = "Lists the languages supported by the Microsoft language stemming tokenizer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<MicrosoftStemmingTokenizerLanguage>,
}
impl MicrosoftLanguageStemmingTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
            is_search_tokenizer: None,
            language: None,
        }
    }
}
#[doc = "Divides text using language-specific rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftLanguageTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Tokens longer than the maximum length are split. Maximum token length that can be used is 300 characters. Tokens longer than 300 characters are first split into tokens of length 300 and then each of those tokens is split based on the max token length set. Default is 255."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    #[doc = "A value indicating how the tokenizer is used. Set to true if used as the search tokenizer, set to false if used as the indexing tokenizer. Default is false."]
    #[serde(rename = "isSearchTokenizer", default, skip_serializing_if = "Option::is_none")]
    pub is_search_tokenizer: Option<bool>,
    #[doc = "Lists the languages supported by the Microsoft language tokenizer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<MicrosoftTokenizerLanguage>,
}
impl MicrosoftLanguageTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
            is_search_tokenizer: None,
            language: None,
        }
    }
}
#[doc = "Lists the languages supported by the Microsoft language stemming tokenizer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MicrosoftStemmingTokenizerLanguage {
    #[serde(rename = "arabic")]
    Arabic,
    #[serde(rename = "bangla")]
    Bangla,
    #[serde(rename = "bulgarian")]
    Bulgarian,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "croatian")]
    Croatian,
    #[serde(rename = "czech")]
    Czech,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "estonian")]
    Estonian,
    #[serde(rename = "finnish")]
    Finnish,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "greek")]
    Greek,
    #[serde(rename = "gujarati")]
    Gujarati,
    #[serde(rename = "hebrew")]
    Hebrew,
    #[serde(rename = "hindi")]
    Hindi,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "icelandic")]
    Icelandic,
    #[serde(rename = "indonesian")]
    Indonesian,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "kannada")]
    Kannada,
    #[serde(rename = "latvian")]
    Latvian,
    #[serde(rename = "lithuanian")]
    Lithuanian,
    #[serde(rename = "malay")]
    Malay,
    #[serde(rename = "malayalam")]
    Malayalam,
    #[serde(rename = "marathi")]
    Marathi,
    #[serde(rename = "norwegianBokmaal")]
    NorwegianBokmaal,
    #[serde(rename = "polish")]
    Polish,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "portugueseBrazilian")]
    PortugueseBrazilian,
    #[serde(rename = "punjabi")]
    Punjabi,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "serbianCyrillic")]
    SerbianCyrillic,
    #[serde(rename = "serbianLatin")]
    SerbianLatin,
    #[serde(rename = "slovak")]
    Slovak,
    #[serde(rename = "slovenian")]
    Slovenian,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "tamil")]
    Tamil,
    #[serde(rename = "telugu")]
    Telugu,
    #[serde(rename = "turkish")]
    Turkish,
    #[serde(rename = "ukrainian")]
    Ukrainian,
    #[serde(rename = "urdu")]
    Urdu,
}
#[doc = "Lists the languages supported by the Microsoft language tokenizer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MicrosoftTokenizerLanguage {
    #[serde(rename = "bangla")]
    Bangla,
    #[serde(rename = "bulgarian")]
    Bulgarian,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "chineseSimplified")]
    ChineseSimplified,
    #[serde(rename = "chineseTraditional")]
    ChineseTraditional,
    #[serde(rename = "croatian")]
    Croatian,
    #[serde(rename = "czech")]
    Czech,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "greek")]
    Greek,
    #[serde(rename = "gujarati")]
    Gujarati,
    #[serde(rename = "hindi")]
    Hindi,
    #[serde(rename = "icelandic")]
    Icelandic,
    #[serde(rename = "indonesian")]
    Indonesian,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "japanese")]
    Japanese,
    #[serde(rename = "kannada")]
    Kannada,
    #[serde(rename = "korean")]
    Korean,
    #[serde(rename = "malay")]
    Malay,
    #[serde(rename = "malayalam")]
    Malayalam,
    #[serde(rename = "marathi")]
    Marathi,
    #[serde(rename = "norwegianBokmaal")]
    NorwegianBokmaal,
    #[serde(rename = "polish")]
    Polish,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "portugueseBrazilian")]
    PortugueseBrazilian,
    #[serde(rename = "punjabi")]
    Punjabi,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "serbianCyrillic")]
    SerbianCyrillic,
    #[serde(rename = "serbianLatin")]
    SerbianLatin,
    #[serde(rename = "slovenian")]
    Slovenian,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "tamil")]
    Tamil,
    #[serde(rename = "telugu")]
    Telugu,
    #[serde(rename = "thai")]
    Thai,
    #[serde(rename = "ukrainian")]
    Ukrainian,
    #[serde(rename = "urdu")]
    Urdu,
    #[serde(rename = "vietnamese")]
    Vietnamese,
}
#[doc = "Generates n-grams of the given size(s). This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NGramTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The minimum n-gram length. Default is 1. Must be less than the value of maxGram."]
    #[serde(rename = "minGram", default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[doc = "The maximum n-gram length. Default is 2."]
    #[serde(rename = "maxGram", default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
}
impl NGramTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            min_gram: None,
            max_gram: None,
        }
    }
}
#[doc = "Generates n-grams of the given size(s). This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NGramTokenFilterV2 {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The minimum n-gram length. Default is 1. Maximum is 300. Must be less than the value of maxGram."]
    #[serde(rename = "minGram", default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[doc = "The maximum n-gram length. Default is 2. Maximum is 300."]
    #[serde(rename = "maxGram", default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
}
impl NGramTokenFilterV2 {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            min_gram: None,
            max_gram: None,
        }
    }
}
#[doc = "Tokenizes the input into n-grams of the given size(s). This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NGramTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The minimum n-gram length. Default is 1. Maximum is 300. Must be less than the value of maxGram."]
    #[serde(rename = "minGram", default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[doc = "The maximum n-gram length. Default is 2. Maximum is 300."]
    #[serde(rename = "maxGram", default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
    #[doc = "Character classes to keep in the tokens."]
    #[serde(
        rename = "tokenChars",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_chars: Vec<TokenCharacterKind>,
}
impl NGramTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            min_gram: None,
            max_gram: None,
            token_chars: Vec::new(),
        }
    }
}
#[doc = "Defines a data deletion detection policy utilizing Azure Blob Storage's native soft delete feature for deletion detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NativeBlobSoftDeleteDeletionDetectionPolicy {}
impl NativeBlobSoftDeleteDeletionDetectionPolicy {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "A skill that extracts text from image files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OcrSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The language codes supported for input by OcrSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<OcrSkillLanguage>,
    #[doc = "A value indicating to turn orientation detection on or not. Default is false."]
    #[serde(rename = "detectOrientation", default, skip_serializing_if = "Option::is_none")]
    pub detect_orientation: Option<bool>,
    #[doc = "Defines the sequence of characters to use between the lines of text recognized by the OCR skill. The default value is \"space\"."]
    #[serde(rename = "lineEnding", default, skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<OcrSkillLineEnding>,
}
impl OcrSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            detect_orientation: None,
            line_ending: None,
        }
    }
}
#[doc = "The language codes supported for input by OcrSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OcrSkillLanguage")]
pub enum OcrSkillLanguage {
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "anp")]
    Anp,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "ast")]
    Ast,
    #[serde(rename = "awa")]
    Awa,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "bfy")]
    Bfy,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "be-cyrl")]
    BeCyrl,
    #[serde(rename = "be-latn")]
    BeLatn,
    #[serde(rename = "bho")]
    Bho,
    #[serde(rename = "bi")]
    Bi,
    #[serde(rename = "brx")]
    Brx,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "bra")]
    Bra,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bns")]
    Bns,
    #[serde(rename = "bua")]
    Bua,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "ceb")]
    Ceb,
    #[serde(rename = "rab")]
    Rab,
    #[serde(rename = "ch")]
    Ch,
    #[serde(rename = "hne")]
    Hne,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(rename = "kw")]
    Kw,
    #[serde(rename = "co")]
    Co,
    #[serde(rename = "crh")]
    Crh,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "prs")]
    Prs,
    #[serde(rename = "dhi")]
    Dhi,
    #[serde(rename = "doi")]
    Doi,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "myv")]
    Myv,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fj")]
    Fj,
    #[serde(rename = "fil")]
    Fil,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fur")]
    Fur,
    #[serde(rename = "gag")]
    Gag,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "gil")]
    Gil,
    #[serde(rename = "gon")]
    Gon,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "kl")]
    Kl,
    #[serde(rename = "gvr")]
    Gvr,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hlb")]
    Hlb,
    #[serde(rename = "hni")]
    Hni,
    #[serde(rename = "bgc")]
    Bgc,
    #[serde(rename = "haw")]
    Haw,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "mww")]
    Mww,
    #[serde(rename = "hoc")]
    Hoc,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "smn")]
    Smn,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "ia")]
    Ia,
    #[serde(rename = "iu")]
    Iu,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    Jns,
    #[serde(rename = "jv")]
    Jv,
    #[serde(rename = "kea")]
    Kea,
    #[serde(rename = "kac")]
    Kac,
    #[serde(rename = "xnr")]
    Xnr,
    #[serde(rename = "krc")]
    Krc,
    #[serde(rename = "kaa-cyrl")]
    KaaCyrl,
    #[serde(rename = "kaa")]
    Kaa,
    #[serde(rename = "csb")]
    Csb,
    #[serde(rename = "kk-cyrl")]
    KkCyrl,
    #[serde(rename = "kk-latn")]
    KkLatn,
    #[serde(rename = "klr")]
    Klr,
    #[serde(rename = "kha")]
    Kha,
    #[serde(rename = "quc")]
    Quc,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "kfq")]
    Kfq,
    #[serde(rename = "kpy")]
    Kpy,
    #[serde(rename = "kos")]
    Kos,
    #[serde(rename = "kum")]
    Kum,
    #[serde(rename = "ku-arab")]
    KuArab,
    #[serde(rename = "ku-latn")]
    KuLatn,
    #[serde(rename = "kru")]
    Kru,
    #[serde(rename = "ky")]
    Ky,
    #[serde(rename = "lkt")]
    Lkt,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "dsb")]
    Dsb,
    #[serde(rename = "smj")]
    Smj,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "bfz")]
    Bfz,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "kmj")]
    Kmj,
    #[serde(rename = "gv")]
    Gv,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "cnr-cyrl")]
    CnrCyrl,
    #[serde(rename = "cnr-latn")]
    CnrLatn,
    #[serde(rename = "nap")]
    Nap,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "niu")]
    Niu,
    #[serde(rename = "nog")]
    Nog,
    #[serde(rename = "sme")]
    Sme,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "oc")]
    Oc,
    #[serde(rename = "os")]
    Os,
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "ksh")]
    Ksh,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "rm")]
    Rm,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sck")]
    Sck,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "sa")]
    Sa,
    #[serde(rename = "sat")]
    Sat,
    #[serde(rename = "sco")]
    Sco,
    #[serde(rename = "gd")]
    Gd,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "sr-Cyrl")]
    SrCyrl,
    #[serde(rename = "sr-Latn")]
    SrLatn,
    #[serde(rename = "xsr")]
    Xsr,
    #[serde(rename = "srx")]
    Srx,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "so")]
    So,
    #[serde(rename = "sma")]
    Sma,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "tt")]
    Tt,
    #[serde(rename = "tet")]
    Tet,
    #[serde(rename = "thf")]
    Thf,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "tk")]
    Tk,
    #[serde(rename = "tyv")]
    Tyv,
    #[serde(rename = "hsb")]
    Hsb,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "ug")]
    Ug,
    #[serde(rename = "uz-arab")]
    UzArab,
    #[serde(rename = "uz-cyrl")]
    UzCyrl,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "vo")]
    Vo,
    #[serde(rename = "wae")]
    Wae,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "fy")]
    Fy,
    #[serde(rename = "yua")]
    Yua,
    #[serde(rename = "za")]
    Za,
    #[serde(rename = "zu")]
    Zu,
    #[serde(rename = "unk")]
    Unk,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OcrSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OcrSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OcrSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Af => serializer.serialize_unit_variant("OcrSkillLanguage", 0u32, "af"),
            Self::Sq => serializer.serialize_unit_variant("OcrSkillLanguage", 1u32, "sq"),
            Self::Anp => serializer.serialize_unit_variant("OcrSkillLanguage", 2u32, "anp"),
            Self::Ar => serializer.serialize_unit_variant("OcrSkillLanguage", 3u32, "ar"),
            Self::Ast => serializer.serialize_unit_variant("OcrSkillLanguage", 4u32, "ast"),
            Self::Awa => serializer.serialize_unit_variant("OcrSkillLanguage", 5u32, "awa"),
            Self::Az => serializer.serialize_unit_variant("OcrSkillLanguage", 6u32, "az"),
            Self::Bfy => serializer.serialize_unit_variant("OcrSkillLanguage", 7u32, "bfy"),
            Self::Eu => serializer.serialize_unit_variant("OcrSkillLanguage", 8u32, "eu"),
            Self::Be => serializer.serialize_unit_variant("OcrSkillLanguage", 9u32, "be"),
            Self::BeCyrl => serializer.serialize_unit_variant("OcrSkillLanguage", 10u32, "be-cyrl"),
            Self::BeLatn => serializer.serialize_unit_variant("OcrSkillLanguage", 11u32, "be-latn"),
            Self::Bho => serializer.serialize_unit_variant("OcrSkillLanguage", 12u32, "bho"),
            Self::Bi => serializer.serialize_unit_variant("OcrSkillLanguage", 13u32, "bi"),
            Self::Brx => serializer.serialize_unit_variant("OcrSkillLanguage", 14u32, "brx"),
            Self::Bs => serializer.serialize_unit_variant("OcrSkillLanguage", 15u32, "bs"),
            Self::Bra => serializer.serialize_unit_variant("OcrSkillLanguage", 16u32, "bra"),
            Self::Br => serializer.serialize_unit_variant("OcrSkillLanguage", 17u32, "br"),
            Self::Bg => serializer.serialize_unit_variant("OcrSkillLanguage", 18u32, "bg"),
            Self::Bns => serializer.serialize_unit_variant("OcrSkillLanguage", 19u32, "bns"),
            Self::Bua => serializer.serialize_unit_variant("OcrSkillLanguage", 20u32, "bua"),
            Self::Ca => serializer.serialize_unit_variant("OcrSkillLanguage", 21u32, "ca"),
            Self::Ceb => serializer.serialize_unit_variant("OcrSkillLanguage", 22u32, "ceb"),
            Self::Rab => serializer.serialize_unit_variant("OcrSkillLanguage", 23u32, "rab"),
            Self::Ch => serializer.serialize_unit_variant("OcrSkillLanguage", 24u32, "ch"),
            Self::Hne => serializer.serialize_unit_variant("OcrSkillLanguage", 25u32, "hne"),
            Self::ZhHans => serializer.serialize_unit_variant("OcrSkillLanguage", 26u32, "zh-Hans"),
            Self::ZhHant => serializer.serialize_unit_variant("OcrSkillLanguage", 27u32, "zh-Hant"),
            Self::Kw => serializer.serialize_unit_variant("OcrSkillLanguage", 28u32, "kw"),
            Self::Co => serializer.serialize_unit_variant("OcrSkillLanguage", 29u32, "co"),
            Self::Crh => serializer.serialize_unit_variant("OcrSkillLanguage", 30u32, "crh"),
            Self::Hr => serializer.serialize_unit_variant("OcrSkillLanguage", 31u32, "hr"),
            Self::Cs => serializer.serialize_unit_variant("OcrSkillLanguage", 32u32, "cs"),
            Self::Da => serializer.serialize_unit_variant("OcrSkillLanguage", 33u32, "da"),
            Self::Prs => serializer.serialize_unit_variant("OcrSkillLanguage", 34u32, "prs"),
            Self::Dhi => serializer.serialize_unit_variant("OcrSkillLanguage", 35u32, "dhi"),
            Self::Doi => serializer.serialize_unit_variant("OcrSkillLanguage", 36u32, "doi"),
            Self::Nl => serializer.serialize_unit_variant("OcrSkillLanguage", 37u32, "nl"),
            Self::En => serializer.serialize_unit_variant("OcrSkillLanguage", 38u32, "en"),
            Self::Myv => serializer.serialize_unit_variant("OcrSkillLanguage", 39u32, "myv"),
            Self::Et => serializer.serialize_unit_variant("OcrSkillLanguage", 40u32, "et"),
            Self::Fo => serializer.serialize_unit_variant("OcrSkillLanguage", 41u32, "fo"),
            Self::Fj => serializer.serialize_unit_variant("OcrSkillLanguage", 42u32, "fj"),
            Self::Fil => serializer.serialize_unit_variant("OcrSkillLanguage", 43u32, "fil"),
            Self::Fi => serializer.serialize_unit_variant("OcrSkillLanguage", 44u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("OcrSkillLanguage", 45u32, "fr"),
            Self::Fur => serializer.serialize_unit_variant("OcrSkillLanguage", 46u32, "fur"),
            Self::Gag => serializer.serialize_unit_variant("OcrSkillLanguage", 47u32, "gag"),
            Self::Gl => serializer.serialize_unit_variant("OcrSkillLanguage", 48u32, "gl"),
            Self::De => serializer.serialize_unit_variant("OcrSkillLanguage", 49u32, "de"),
            Self::Gil => serializer.serialize_unit_variant("OcrSkillLanguage", 50u32, "gil"),
            Self::Gon => serializer.serialize_unit_variant("OcrSkillLanguage", 51u32, "gon"),
            Self::El => serializer.serialize_unit_variant("OcrSkillLanguage", 52u32, "el"),
            Self::Kl => serializer.serialize_unit_variant("OcrSkillLanguage", 53u32, "kl"),
            Self::Gvr => serializer.serialize_unit_variant("OcrSkillLanguage", 54u32, "gvr"),
            Self::Ht => serializer.serialize_unit_variant("OcrSkillLanguage", 55u32, "ht"),
            Self::Hlb => serializer.serialize_unit_variant("OcrSkillLanguage", 56u32, "hlb"),
            Self::Hni => serializer.serialize_unit_variant("OcrSkillLanguage", 57u32, "hni"),
            Self::Bgc => serializer.serialize_unit_variant("OcrSkillLanguage", 58u32, "bgc"),
            Self::Haw => serializer.serialize_unit_variant("OcrSkillLanguage", 59u32, "haw"),
            Self::Hi => serializer.serialize_unit_variant("OcrSkillLanguage", 60u32, "hi"),
            Self::Mww => serializer.serialize_unit_variant("OcrSkillLanguage", 61u32, "mww"),
            Self::Hoc => serializer.serialize_unit_variant("OcrSkillLanguage", 62u32, "hoc"),
            Self::Hu => serializer.serialize_unit_variant("OcrSkillLanguage", 63u32, "hu"),
            Self::Is => serializer.serialize_unit_variant("OcrSkillLanguage", 64u32, "is"),
            Self::Smn => serializer.serialize_unit_variant("OcrSkillLanguage", 65u32, "smn"),
            Self::Id => serializer.serialize_unit_variant("OcrSkillLanguage", 66u32, "id"),
            Self::Ia => serializer.serialize_unit_variant("OcrSkillLanguage", 67u32, "ia"),
            Self::Iu => serializer.serialize_unit_variant("OcrSkillLanguage", 68u32, "iu"),
            Self::Ga => serializer.serialize_unit_variant("OcrSkillLanguage", 69u32, "ga"),
            Self::It => serializer.serialize_unit_variant("OcrSkillLanguage", 70u32, "it"),
            Self::Ja => serializer.serialize_unit_variant("OcrSkillLanguage", 71u32, "ja"),
            Self::Jns => serializer.serialize_unit_variant("OcrSkillLanguage", 72u32, "Jns"),
            Self::Jv => serializer.serialize_unit_variant("OcrSkillLanguage", 73u32, "jv"),
            Self::Kea => serializer.serialize_unit_variant("OcrSkillLanguage", 74u32, "kea"),
            Self::Kac => serializer.serialize_unit_variant("OcrSkillLanguage", 75u32, "kac"),
            Self::Xnr => serializer.serialize_unit_variant("OcrSkillLanguage", 76u32, "xnr"),
            Self::Krc => serializer.serialize_unit_variant("OcrSkillLanguage", 77u32, "krc"),
            Self::KaaCyrl => serializer.serialize_unit_variant("OcrSkillLanguage", 78u32, "kaa-cyrl"),
            Self::Kaa => serializer.serialize_unit_variant("OcrSkillLanguage", 79u32, "kaa"),
            Self::Csb => serializer.serialize_unit_variant("OcrSkillLanguage", 80u32, "csb"),
            Self::KkCyrl => serializer.serialize_unit_variant("OcrSkillLanguage", 81u32, "kk-cyrl"),
            Self::KkLatn => serializer.serialize_unit_variant("OcrSkillLanguage", 82u32, "kk-latn"),
            Self::Klr => serializer.serialize_unit_variant("OcrSkillLanguage", 83u32, "klr"),
            Self::Kha => serializer.serialize_unit_variant("OcrSkillLanguage", 84u32, "kha"),
            Self::Quc => serializer.serialize_unit_variant("OcrSkillLanguage", 85u32, "quc"),
            Self::Ko => serializer.serialize_unit_variant("OcrSkillLanguage", 86u32, "ko"),
            Self::Kfq => serializer.serialize_unit_variant("OcrSkillLanguage", 87u32, "kfq"),
            Self::Kpy => serializer.serialize_unit_variant("OcrSkillLanguage", 88u32, "kpy"),
            Self::Kos => serializer.serialize_unit_variant("OcrSkillLanguage", 89u32, "kos"),
            Self::Kum => serializer.serialize_unit_variant("OcrSkillLanguage", 90u32, "kum"),
            Self::KuArab => serializer.serialize_unit_variant("OcrSkillLanguage", 91u32, "ku-arab"),
            Self::KuLatn => serializer.serialize_unit_variant("OcrSkillLanguage", 92u32, "ku-latn"),
            Self::Kru => serializer.serialize_unit_variant("OcrSkillLanguage", 93u32, "kru"),
            Self::Ky => serializer.serialize_unit_variant("OcrSkillLanguage", 94u32, "ky"),
            Self::Lkt => serializer.serialize_unit_variant("OcrSkillLanguage", 95u32, "lkt"),
            Self::La => serializer.serialize_unit_variant("OcrSkillLanguage", 96u32, "la"),
            Self::Lt => serializer.serialize_unit_variant("OcrSkillLanguage", 97u32, "lt"),
            Self::Dsb => serializer.serialize_unit_variant("OcrSkillLanguage", 98u32, "dsb"),
            Self::Smj => serializer.serialize_unit_variant("OcrSkillLanguage", 99u32, "smj"),
            Self::Lb => serializer.serialize_unit_variant("OcrSkillLanguage", 100u32, "lb"),
            Self::Bfz => serializer.serialize_unit_variant("OcrSkillLanguage", 101u32, "bfz"),
            Self::Ms => serializer.serialize_unit_variant("OcrSkillLanguage", 102u32, "ms"),
            Self::Mt => serializer.serialize_unit_variant("OcrSkillLanguage", 103u32, "mt"),
            Self::Kmj => serializer.serialize_unit_variant("OcrSkillLanguage", 104u32, "kmj"),
            Self::Gv => serializer.serialize_unit_variant("OcrSkillLanguage", 105u32, "gv"),
            Self::Mi => serializer.serialize_unit_variant("OcrSkillLanguage", 106u32, "mi"),
            Self::Mr => serializer.serialize_unit_variant("OcrSkillLanguage", 107u32, "mr"),
            Self::Mn => serializer.serialize_unit_variant("OcrSkillLanguage", 108u32, "mn"),
            Self::CnrCyrl => serializer.serialize_unit_variant("OcrSkillLanguage", 109u32, "cnr-cyrl"),
            Self::CnrLatn => serializer.serialize_unit_variant("OcrSkillLanguage", 110u32, "cnr-latn"),
            Self::Nap => serializer.serialize_unit_variant("OcrSkillLanguage", 111u32, "nap"),
            Self::Ne => serializer.serialize_unit_variant("OcrSkillLanguage", 112u32, "ne"),
            Self::Niu => serializer.serialize_unit_variant("OcrSkillLanguage", 113u32, "niu"),
            Self::Nog => serializer.serialize_unit_variant("OcrSkillLanguage", 114u32, "nog"),
            Self::Sme => serializer.serialize_unit_variant("OcrSkillLanguage", 115u32, "sme"),
            Self::Nb => serializer.serialize_unit_variant("OcrSkillLanguage", 116u32, "nb"),
            Self::No => serializer.serialize_unit_variant("OcrSkillLanguage", 117u32, "no"),
            Self::Oc => serializer.serialize_unit_variant("OcrSkillLanguage", 118u32, "oc"),
            Self::Os => serializer.serialize_unit_variant("OcrSkillLanguage", 119u32, "os"),
            Self::Ps => serializer.serialize_unit_variant("OcrSkillLanguage", 120u32, "ps"),
            Self::Fa => serializer.serialize_unit_variant("OcrSkillLanguage", 121u32, "fa"),
            Self::Pl => serializer.serialize_unit_variant("OcrSkillLanguage", 122u32, "pl"),
            Self::Pt => serializer.serialize_unit_variant("OcrSkillLanguage", 123u32, "pt"),
            Self::Pa => serializer.serialize_unit_variant("OcrSkillLanguage", 124u32, "pa"),
            Self::Ksh => serializer.serialize_unit_variant("OcrSkillLanguage", 125u32, "ksh"),
            Self::Ro => serializer.serialize_unit_variant("OcrSkillLanguage", 126u32, "ro"),
            Self::Rm => serializer.serialize_unit_variant("OcrSkillLanguage", 127u32, "rm"),
            Self::Ru => serializer.serialize_unit_variant("OcrSkillLanguage", 128u32, "ru"),
            Self::Sck => serializer.serialize_unit_variant("OcrSkillLanguage", 129u32, "sck"),
            Self::Sm => serializer.serialize_unit_variant("OcrSkillLanguage", 130u32, "sm"),
            Self::Sa => serializer.serialize_unit_variant("OcrSkillLanguage", 131u32, "sa"),
            Self::Sat => serializer.serialize_unit_variant("OcrSkillLanguage", 132u32, "sat"),
            Self::Sco => serializer.serialize_unit_variant("OcrSkillLanguage", 133u32, "sco"),
            Self::Gd => serializer.serialize_unit_variant("OcrSkillLanguage", 134u32, "gd"),
            Self::Sr => serializer.serialize_unit_variant("OcrSkillLanguage", 135u32, "sr"),
            Self::SrCyrl => serializer.serialize_unit_variant("OcrSkillLanguage", 136u32, "sr-Cyrl"),
            Self::SrLatn => serializer.serialize_unit_variant("OcrSkillLanguage", 137u32, "sr-Latn"),
            Self::Xsr => serializer.serialize_unit_variant("OcrSkillLanguage", 138u32, "xsr"),
            Self::Srx => serializer.serialize_unit_variant("OcrSkillLanguage", 139u32, "srx"),
            Self::Sms => serializer.serialize_unit_variant("OcrSkillLanguage", 140u32, "sms"),
            Self::Sk => serializer.serialize_unit_variant("OcrSkillLanguage", 141u32, "sk"),
            Self::Sl => serializer.serialize_unit_variant("OcrSkillLanguage", 142u32, "sl"),
            Self::So => serializer.serialize_unit_variant("OcrSkillLanguage", 143u32, "so"),
            Self::Sma => serializer.serialize_unit_variant("OcrSkillLanguage", 144u32, "sma"),
            Self::Es => serializer.serialize_unit_variant("OcrSkillLanguage", 145u32, "es"),
            Self::Sw => serializer.serialize_unit_variant("OcrSkillLanguage", 146u32, "sw"),
            Self::Sv => serializer.serialize_unit_variant("OcrSkillLanguage", 147u32, "sv"),
            Self::Tg => serializer.serialize_unit_variant("OcrSkillLanguage", 148u32, "tg"),
            Self::Tt => serializer.serialize_unit_variant("OcrSkillLanguage", 149u32, "tt"),
            Self::Tet => serializer.serialize_unit_variant("OcrSkillLanguage", 150u32, "tet"),
            Self::Thf => serializer.serialize_unit_variant("OcrSkillLanguage", 151u32, "thf"),
            Self::To => serializer.serialize_unit_variant("OcrSkillLanguage", 152u32, "to"),
            Self::Tr => serializer.serialize_unit_variant("OcrSkillLanguage", 153u32, "tr"),
            Self::Tk => serializer.serialize_unit_variant("OcrSkillLanguage", 154u32, "tk"),
            Self::Tyv => serializer.serialize_unit_variant("OcrSkillLanguage", 155u32, "tyv"),
            Self::Hsb => serializer.serialize_unit_variant("OcrSkillLanguage", 156u32, "hsb"),
            Self::Ur => serializer.serialize_unit_variant("OcrSkillLanguage", 157u32, "ur"),
            Self::Ug => serializer.serialize_unit_variant("OcrSkillLanguage", 158u32, "ug"),
            Self::UzArab => serializer.serialize_unit_variant("OcrSkillLanguage", 159u32, "uz-arab"),
            Self::UzCyrl => serializer.serialize_unit_variant("OcrSkillLanguage", 160u32, "uz-cyrl"),
            Self::Uz => serializer.serialize_unit_variant("OcrSkillLanguage", 161u32, "uz"),
            Self::Vo => serializer.serialize_unit_variant("OcrSkillLanguage", 162u32, "vo"),
            Self::Wae => serializer.serialize_unit_variant("OcrSkillLanguage", 163u32, "wae"),
            Self::Cy => serializer.serialize_unit_variant("OcrSkillLanguage", 164u32, "cy"),
            Self::Fy => serializer.serialize_unit_variant("OcrSkillLanguage", 165u32, "fy"),
            Self::Yua => serializer.serialize_unit_variant("OcrSkillLanguage", 166u32, "yua"),
            Self::Za => serializer.serialize_unit_variant("OcrSkillLanguage", 167u32, "za"),
            Self::Zu => serializer.serialize_unit_variant("OcrSkillLanguage", 168u32, "zu"),
            Self::Unk => serializer.serialize_unit_variant("OcrSkillLanguage", 169u32, "unk"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines the sequence of characters to use between the lines of text recognized by the OCR skill. The default value is \"space\"."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OcrSkillLineEnding")]
pub enum OcrSkillLineEnding {
    #[serde(rename = "space")]
    Space,
    #[serde(rename = "carriageReturn")]
    CarriageReturn,
    #[serde(rename = "lineFeed")]
    LineFeed,
    #[serde(rename = "carriageReturnLineFeed")]
    CarriageReturnLineFeed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OcrSkillLineEnding {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OcrSkillLineEnding {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OcrSkillLineEnding {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Space => serializer.serialize_unit_variant("OcrSkillLineEnding", 0u32, "space"),
            Self::CarriageReturn => serializer.serialize_unit_variant("OcrSkillLineEnding", 1u32, "carriageReturn"),
            Self::LineFeed => serializer.serialize_unit_variant("OcrSkillLineEnding", 2u32, "lineFeed"),
            Self::CarriageReturnLineFeed => serializer.serialize_unit_variant("OcrSkillLineEnding", 3u32, "carriageReturnLineFeed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Output field mapping for a skill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFieldMappingEntry {
    #[doc = "The name of the output defined by the skill."]
    pub name: String,
    #[doc = "The target name of the output. It is optional and default to name."]
    #[serde(rename = "targetName", default, skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
}
impl OutputFieldMappingEntry {
    pub fn new(name: String) -> Self {
        Self { name, target_name: None }
    }
}
#[doc = "Using the Text Analytics API, extracts personal information from an input text and gives you the option of masking it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PiiDetectionSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "A value indicating which language code to use. Default is `en`."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<String>,
    #[doc = "A value between 0 and 1 that be used to only include entities whose confidence score is greater than the value specified. If not set (default), or if explicitly set to null, all entities will be included."]
    #[serde(rename = "minimumPrecision", default, skip_serializing_if = "Option::is_none")]
    pub minimum_precision: Option<f64>,
    #[doc = "A string indicating what maskingMode to use to mask the personal information detected in the input text."]
    #[serde(rename = "maskingMode", default, skip_serializing_if = "Option::is_none")]
    pub masking_mode: Option<PiiDetectionSkillMaskingMode>,
    #[doc = "The character used to mask the text if the maskingMode parameter is set to replace. Default is '*'."]
    #[serde(rename = "maskingCharacter", default, skip_serializing_if = "Option::is_none")]
    pub masking_character: Option<String>,
    #[doc = "The version of the model to use when calling the Text Analytics service. It will default to the latest available when not specified. We recommend you do not specify this value unless absolutely necessary."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[doc = "A list of PII entity categories that should be extracted and masked."]
    #[serde(
        rename = "piiCategories",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pii_categories: Vec<String>,
    #[doc = "If specified, will set the PII domain to include only a subset of the entity categories. Possible values include: 'phi', 'none'. Default is 'none'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
impl PiiDetectionSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            minimum_precision: None,
            masking_mode: None,
            masking_character: None,
            model_version: None,
            pii_categories: Vec::new(),
            domain: None,
        }
    }
}
#[doc = "A string indicating what maskingMode to use to mask the personal information detected in the input text."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PiiDetectionSkillMaskingMode")]
pub enum PiiDetectionSkillMaskingMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "replace")]
    Replace,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PiiDetectionSkillMaskingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PiiDetectionSkillMaskingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PiiDetectionSkillMaskingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("PiiDetectionSkillMaskingMode", 0u32, "none"),
            Self::Replace => serializer.serialize_unit_variant("PiiDetectionSkillMaskingMode", 1u32, "replace"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the parsing mode for indexing from an Azure blob data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ParsingMode")]
pub enum ParsingMode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "delimitedText")]
    DelimitedText,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "jsonArray")]
    JsonArray,
    #[serde(rename = "jsonLines")]
    JsonLines,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ParsingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ParsingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ParsingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("ParsingMode", 0u32, "default"),
            Self::Text => serializer.serialize_unit_variant("ParsingMode", 1u32, "text"),
            Self::DelimitedText => serializer.serialize_unit_variant("ParsingMode", 2u32, "delimitedText"),
            Self::Json => serializer.serialize_unit_variant("ParsingMode", 3u32, "json"),
            Self::JsonArray => serializer.serialize_unit_variant("ParsingMode", 4u32, "jsonArray"),
            Self::JsonLines => serializer.serialize_unit_variant("ParsingMode", 5u32, "jsonLines"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ParsingMode {
    fn default() -> Self {
        Self::Default
    }
}
#[doc = "Tokenizer for path-like hierarchies. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PathHierarchyTokenizerV2 {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The delimiter character to use. Default is \"/\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[doc = "A value that, if set, replaces the delimiter character. Default is \"/\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    #[doc = "The maximum token length. Default and maximum is 300."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    #[doc = "A value indicating whether to generate tokens in reverse order. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[doc = "The number of initial tokens to skip. Default is 0."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
}
impl PathHierarchyTokenizerV2 {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            delimiter: None,
            replacement: None,
            max_token_length: None,
            reverse: None,
            skip: None,
        }
    }
}
#[doc = "Flexibly separates text into terms via a regular expression pattern. This analyzer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternAnalyzer {
    #[serde(flatten)]
    pub lexical_analyzer: LexicalAnalyzer,
    #[doc = "A value indicating whether terms should be lower-cased. Default is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowercase: Option<bool>,
    #[doc = "A regular expression pattern to match token separators. Default is an expression that matches one or more non-word characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Defines flags that can be combined to control how regular expressions are used in the pattern analyzer and pattern tokenizer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<RegexFlags>,
    #[doc = "A list of stopwords."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stopwords: Vec<String>,
}
impl PatternAnalyzer {
    pub fn new(lexical_analyzer: LexicalAnalyzer) -> Self {
        Self {
            lexical_analyzer,
            lowercase: None,
            pattern: None,
            flags: None,
            stopwords: Vec::new(),
        }
    }
}
#[doc = "Uses Java regexes to emit multiple tokens - one for each capture group in one or more patterns. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternCaptureTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A list of patterns to match against each token."]
    pub patterns: Vec<String>,
    #[doc = "A value indicating whether to return the original token even if one of the patterns matches. Default is true."]
    #[serde(rename = "preserveOriginal", default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<bool>,
}
impl PatternCaptureTokenFilter {
    pub fn new(token_filter: TokenFilter, patterns: Vec<String>) -> Self {
        Self {
            token_filter,
            patterns,
            preserve_original: None,
        }
    }
}
#[doc = "A character filter that replaces characters in the input string. It uses a regular expression to identify character sequences to preserve and a replacement pattern to identify characters to replace. For example, given the input text \"aa bb aa bb\", pattern \"(aa)\\s+(bb)\", and replacement \"$1#$2\", the result would be \"aa#bb aa#bb\". This character filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternReplaceCharFilter {
    #[serde(flatten)]
    pub char_filter: CharFilter,
    #[doc = "A regular expression pattern."]
    pub pattern: String,
    #[doc = "The replacement text."]
    pub replacement: String,
}
impl PatternReplaceCharFilter {
    pub fn new(char_filter: CharFilter, pattern: String, replacement: String) -> Self {
        Self {
            char_filter,
            pattern,
            replacement,
        }
    }
}
#[doc = "A character filter that replaces characters in the input string. It uses a regular expression to identify character sequences to preserve and a replacement pattern to identify characters to replace. For example, given the input text \"aa bb aa bb\", pattern \"(aa)\\s+(bb)\", and replacement \"$1#$2\", the result would be \"aa#bb aa#bb\". This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternReplaceTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A regular expression pattern."]
    pub pattern: String,
    #[doc = "The replacement text."]
    pub replacement: String,
}
impl PatternReplaceTokenFilter {
    pub fn new(token_filter: TokenFilter, pattern: String, replacement: String) -> Self {
        Self {
            token_filter,
            pattern,
            replacement,
        }
    }
}
#[doc = "Tokenizer that uses regex pattern matching to construct distinct tokens. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "A regular expression pattern to match token separators. Default is an expression that matches one or more non-word characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Defines flags that can be combined to control how regular expressions are used in the pattern analyzer and pattern tokenizer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<RegexFlags>,
    #[doc = "The zero-based ordinal of the matching group in the regular expression pattern to extract into tokens. Use -1 if you want to use the entire pattern to split the input into tokens, irrespective of matching groups. Default is -1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
}
impl PatternTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            pattern: None,
            flags: None,
            group: None,
        }
    }
}
#[doc = "Determines algorithm for text extraction from PDF files in Azure blob storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PdfTextRotationAlgorithm")]
pub enum PdfTextRotationAlgorithm {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "detectAngles")]
    DetectAngles,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PdfTextRotationAlgorithm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PdfTextRotationAlgorithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PdfTextRotationAlgorithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("PdfTextRotationAlgorithm", 0u32, "none"),
            Self::DetectAngles => serializer.serialize_unit_variant("PdfTextRotationAlgorithm", 1u32, "detectAngles"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for PdfTextRotationAlgorithm {
    fn default() -> Self {
        Self::None
    }
}
#[doc = "Identifies the type of phonetic encoder to use with a PhoneticTokenFilter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PhoneticEncoder {
    #[serde(rename = "metaphone")]
    Metaphone,
    #[serde(rename = "doubleMetaphone")]
    DoubleMetaphone,
    #[serde(rename = "soundex")]
    Soundex,
    #[serde(rename = "refinedSoundex")]
    RefinedSoundex,
    #[serde(rename = "caverphone1")]
    Caverphone1,
    #[serde(rename = "caverphone2")]
    Caverphone2,
    #[serde(rename = "cologne")]
    Cologne,
    #[serde(rename = "nysiis")]
    Nysiis,
    #[serde(rename = "koelnerPhonetik")]
    KoelnerPhonetik,
    #[serde(rename = "haasePhonetik")]
    HaasePhonetik,
    #[serde(rename = "beiderMorse")]
    BeiderMorse,
}
#[doc = "Create tokens for phonetic matches. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhoneticTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "Identifies the type of phonetic encoder to use with a PhoneticTokenFilter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoder: Option<PhoneticEncoder>,
    #[doc = "A value indicating whether encoded tokens should replace original tokens. If false, encoded tokens are added as synonyms. Default is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}
impl PhoneticTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            encoder: None,
            replace: None,
        }
    }
}
#[doc = "Describes the title, content, and keywords fields to be used for semantic ranking, captions, highlights, and answers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrioritizedFields {
    #[doc = "A field that is used as part of the semantic configuration."]
    #[serde(rename = "titleField", default, skip_serializing_if = "Option::is_none")]
    pub title_field: Option<SemanticField>,
    #[doc = "Defines the content fields to be used for semantic ranking, captions, highlights, and answers. For the best result, the selected fields should contain text in natural language form. The order of the fields in the array represents their priority. Fields with lower priority may get truncated if the content is long."]
    #[serde(
        rename = "prioritizedContentFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prioritized_content_fields: Vec<SemanticField>,
    #[doc = "Defines the keyword fields to be used for semantic ranking, captions, highlights, and answers. For the best result, the selected fields should contain a list of keywords. The order of the fields in the array represents their priority. Fields with lower priority may get truncated if the content is long."]
    #[serde(
        rename = "prioritizedKeywordsFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prioritized_keywords_fields: Vec<SemanticField>,
}
impl PrioritizedFields {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines flags that can be combined to control how regular expressions are used in the pattern analyzer and pattern tokenizer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegexFlags")]
pub enum RegexFlags {
    #[serde(rename = "CANON_EQ")]
    CanonEq,
    #[serde(rename = "CASE_INSENSITIVE")]
    CaseInsensitive,
    #[serde(rename = "COMMENTS")]
    Comments,
    #[serde(rename = "DOTALL")]
    Dotall,
    #[serde(rename = "LITERAL")]
    Literal,
    #[serde(rename = "MULTILINE")]
    Multiline,
    #[serde(rename = "UNICODE_CASE")]
    UnicodeCase,
    #[serde(rename = "UNIX_LINES")]
    UnixLines,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegexFlags {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegexFlags {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegexFlags {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CanonEq => serializer.serialize_unit_variant("RegexFlags", 0u32, "CANON_EQ"),
            Self::CaseInsensitive => serializer.serialize_unit_variant("RegexFlags", 1u32, "CASE_INSENSITIVE"),
            Self::Comments => serializer.serialize_unit_variant("RegexFlags", 2u32, "COMMENTS"),
            Self::Dotall => serializer.serialize_unit_variant("RegexFlags", 3u32, "DOTALL"),
            Self::Literal => serializer.serialize_unit_variant("RegexFlags", 4u32, "LITERAL"),
            Self::Multiline => serializer.serialize_unit_variant("RegexFlags", 5u32, "MULTILINE"),
            Self::UnicodeCase => serializer.serialize_unit_variant("RegexFlags", 6u32, "UNICODE_CASE"),
            Self::UnixLines => serializer.serialize_unit_variant("RegexFlags", 7u32, "UNIX_LINES"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a resource's usage and quota."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCounter {
    #[doc = "The resource usage amount."]
    pub usage: i64,
    #[doc = "The resource amount quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
}
impl ResourceCounter {
    pub fn new(usage: i64) -> Self {
        Self { usage, quota: None }
    }
}
#[doc = "Base type for functions that can modify document scores during ranking."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoringFunction {
    #[doc = "The name of the field used as input to the scoring function."]
    #[serde(rename = "fieldName")]
    pub field_name: String,
    #[doc = "A multiplier for the raw score. Must be a positive number not equal to 1.0."]
    pub boost: f64,
    #[doc = "Defines the function used to interpolate score boosting across a range of documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<ScoringFunctionInterpolation>,
}
impl ScoringFunction {
    pub fn new(field_name: String, boost: f64) -> Self {
        Self {
            field_name,
            boost,
            interpolation: None,
        }
    }
}
#[doc = "Indicates the type of function to use. Valid values include magnitude, freshness, distance, and tag. The function type must be lower case."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScoringFunctionUnion {
    #[serde(rename = "distance")]
    Distance(DistanceScoringFunction),
    #[serde(rename = "freshness")]
    Freshness(FreshnessScoringFunction),
    #[serde(rename = "magnitude")]
    Magnitude(MagnitudeScoringFunction),
    #[serde(rename = "tag")]
    Tag(TagScoringFunction),
}
#[doc = "Defines the aggregation function used to combine the results of all the scoring functions in a scoring profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScoringFunctionAggregation {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "minimum")]
    Minimum,
    #[serde(rename = "maximum")]
    Maximum,
    #[serde(rename = "firstMatching")]
    FirstMatching,
}
#[doc = "Defines the function used to interpolate score boosting across a range of documents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScoringFunctionInterpolation {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "quadratic")]
    Quadratic,
    #[serde(rename = "logarithmic")]
    Logarithmic,
}
#[doc = "Defines parameters for a search index that influence scoring in search queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoringProfile {
    #[doc = "The name of the scoring profile."]
    pub name: String,
    #[doc = "Defines weights on index fields for which matches should boost scoring in search queries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextWeights>,
    #[doc = "The collection of functions that influence the scoring of documents."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub functions: Vec<ScoringFunctionUnion>,
    #[doc = "Defines the aggregation function used to combine the results of all the scoring functions in a scoring profile."]
    #[serde(rename = "functionAggregation", default, skip_serializing_if = "Option::is_none")]
    pub function_aggregation: Option<ScoringFunctionAggregation>,
}
impl ScoringProfile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            text: None,
            functions: Vec::new(),
            function_aggregation: None,
        }
    }
}
#[doc = "Represents an index alias, which describes a mapping from the alias name to an index. The alias name can be used in place of the index name for supported operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchAlias {
    #[doc = "The name of the alias."]
    pub name: String,
    #[doc = "The name of the index this alias maps to. Only one index name may be specified."]
    pub indexes: Vec<String>,
    #[doc = "The ETag of the alias."]
    #[serde(rename = "@odata.etag", default, skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
}
impl SearchAlias {
    pub fn new(name: String, indexes: Vec<String>) -> Self {
        Self {
            name,
            indexes,
            odata_etag: None,
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
impl azure_core::Continuable for SearchError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
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
#[doc = "Represents a field in an index definition, which describes the name, data type, and search behavior of a field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchField {
    #[doc = "The name of the field, which must be unique within the fields collection of the index or parent field."]
    pub name: String,
    #[doc = "Defines the data type of a field in a search index."]
    #[serde(rename = "type")]
    pub type_: SearchFieldDataType,
    #[doc = "A value indicating whether the field uniquely identifies documents in the index. Exactly one top-level field in each index must be chosen as the key field and it must be of type Edm.String. Key fields can be used to look up documents directly and update or delete specific documents. Default is false for simple fields and null for complex fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<bool>,
    #[doc = "A value indicating whether the field can be returned in a search result. You can disable this option if you want to use a field (for example, margin) as a filter, sorting, or scoring mechanism but do not want the field to be visible to the end user. This property must be true for key fields, and it must be null for complex fields. This property can be changed on existing fields. Enabling this property does not cause any increase in index storage requirements. Default is true for simple fields and null for complex fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrievable: Option<bool>,
    #[doc = "A value indicating whether the field is full-text searchable. This means it will undergo analysis such as word-breaking during indexing. If you set a searchable field to a value like \"sunny day\", internally it will be split into the individual tokens \"sunny\" and \"day\". This enables full-text searches for these terms. Fields of type Edm.String or Collection(Edm.String) are searchable by default. This property must be false for simple fields of other non-string data types, and it must be null for complex fields. Note: searchable fields consume extra space in your index to accommodate additional tokenized versions of the field value for full-text searches. If you want to save space in your index and you don't need a field to be included in searches, set searchable to false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    #[doc = "A value indicating whether to enable the field to be referenced in $filter queries. filterable differs from searchable in how strings are handled. Fields of type Edm.String or Collection(Edm.String) that are filterable do not undergo word-breaking, so comparisons are for exact matches only. For example, if you set such a field f to \"sunny day\", $filter=f eq 'sunny' will find no matches, but $filter=f eq 'sunny day' will. This property must be null for complex fields. Default is true for simple fields and null for complex fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filterable: Option<bool>,
    #[doc = "A value indicating whether to enable the field to be referenced in $orderby expressions. By default, the search engine sorts results by score, but in many experiences users will want to sort by fields in the documents. A simple field can be sortable only if it is single-valued (it has a single value in the scope of the parent document). Simple collection fields cannot be sortable, since they are multi-valued. Simple sub-fields of complex collections are also multi-valued, and therefore cannot be sortable. This is true whether it's an immediate parent field, or an ancestor field, that's the complex collection. Complex fields cannot be sortable and the sortable property must be null for such fields. The default for sortable is true for single-valued simple fields, false for multi-valued simple fields, and null for complex fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
    #[doc = "A value indicating whether to enable the field to be referenced in facet queries. Typically used in a presentation of search results that includes hit count by category (for example, search for digital cameras and see hits by brand, by megapixels, by price, and so on). This property must be null for complex fields. Fields of type Edm.GeographyPoint or Collection(Edm.GeographyPoint) cannot be facetable. Default is true for all other simple fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facetable: Option<bool>,
    #[doc = "Defines the names of all text analyzers supported by the search engine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<LexicalAnalyzerName>,
    #[doc = "Defines the names of all text analyzers supported by the search engine."]
    #[serde(rename = "searchAnalyzer", default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<LexicalAnalyzerName>,
    #[doc = "Defines the names of all text analyzers supported by the search engine."]
    #[serde(rename = "indexAnalyzer", default, skip_serializing_if = "Option::is_none")]
    pub index_analyzer: Option<LexicalAnalyzerName>,
    #[doc = "Defines the names of all text normalizers supported by the search engine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<LexicalNormalizerName>,
    #[doc = "The dimensionality of the vector field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
    #[doc = "The name of the vector search profile that specifies the algorithm and vectorizer to use when searching the vector field."]
    #[serde(rename = "vectorSearchProfile", default, skip_serializing_if = "Option::is_none")]
    pub vector_search_profile: Option<String>,
    #[doc = "A list of the names of synonym maps to associate with this field. This option can be used only with searchable fields. Currently only one synonym map per field is supported. Assigning a synonym map to a field ensures that query terms targeting that field are expanded at query-time using the rules in the synonym map. This attribute can be changed on existing fields. Must be null or an empty collection for complex fields."]
    #[serde(
        rename = "synonymMaps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub synonym_maps: Vec<String>,
    #[doc = "A list of sub-fields if this is a field of type Edm.ComplexType or Collection(Edm.ComplexType). Must be null or empty for simple fields."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<SearchField>,
}
impl SearchField {
    pub fn new(name: String, type_: SearchFieldDataType) -> Self {
        Self {
            name,
            type_,
            key: None,
            retrievable: None,
            searchable: None,
            filterable: None,
            sortable: None,
            facetable: None,
            analyzer: None,
            search_analyzer: None,
            index_analyzer: None,
            normalizer: None,
            dimensions: None,
            vector_search_profile: None,
            synonym_maps: Vec::new(),
            fields: Vec::new(),
        }
    }
}
#[doc = "Defines the data type of a field in a search index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SearchFieldDataType")]
pub enum SearchFieldDataType {
    #[serde(rename = "Edm.String")]
    EdmString,
    #[serde(rename = "Edm.Int32")]
    EdmInt32,
    #[serde(rename = "Edm.Int64")]
    EdmInt64,
    #[serde(rename = "Edm.Double")]
    EdmDouble,
    #[serde(rename = "Edm.Boolean")]
    EdmBoolean,
    #[serde(rename = "Edm.DateTimeOffset")]
    EdmDateTimeOffset,
    #[serde(rename = "Edm.GeographyPoint")]
    EdmGeographyPoint,
    #[serde(rename = "Edm.ComplexType")]
    EdmComplexType,
    #[serde(rename = "Edm.Single")]
    EdmSingle,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SearchFieldDataType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SearchFieldDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SearchFieldDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdmString => serializer.serialize_unit_variant("SearchFieldDataType", 0u32, "Edm.String"),
            Self::EdmInt32 => serializer.serialize_unit_variant("SearchFieldDataType", 1u32, "Edm.Int32"),
            Self::EdmInt64 => serializer.serialize_unit_variant("SearchFieldDataType", 2u32, "Edm.Int64"),
            Self::EdmDouble => serializer.serialize_unit_variant("SearchFieldDataType", 3u32, "Edm.Double"),
            Self::EdmBoolean => serializer.serialize_unit_variant("SearchFieldDataType", 4u32, "Edm.Boolean"),
            Self::EdmDateTimeOffset => serializer.serialize_unit_variant("SearchFieldDataType", 5u32, "Edm.DateTimeOffset"),
            Self::EdmGeographyPoint => serializer.serialize_unit_variant("SearchFieldDataType", 6u32, "Edm.GeographyPoint"),
            Self::EdmComplexType => serializer.serialize_unit_variant("SearchFieldDataType", 7u32, "Edm.ComplexType"),
            Self::EdmSingle => serializer.serialize_unit_variant("SearchFieldDataType", 8u32, "Edm.Single"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a search index definition, which describes the fields and search behavior of an index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndex {
    #[doc = "The name of the index."]
    pub name: String,
    #[doc = "The fields of the index."]
    pub fields: Vec<SearchField>,
    #[doc = "The scoring profiles for the index."]
    #[serde(
        rename = "scoringProfiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scoring_profiles: Vec<ScoringProfile>,
    #[doc = "The name of the scoring profile to use if none is specified in the query. If this property is not set and no scoring profile is specified in the query, then default scoring (tf-idf) will be used."]
    #[serde(rename = "defaultScoringProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_scoring_profile: Option<String>,
    #[doc = "Defines options to control Cross-Origin Resource Sharing (CORS) for an index."]
    #[serde(rename = "corsOptions", default, skip_serializing_if = "Option::is_none")]
    pub cors_options: Option<CorsOptions>,
    #[doc = "The suggesters for the index."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suggesters: Vec<Suggester>,
    #[doc = "The analyzers for the index."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub analyzers: Vec<LexicalAnalyzerUnion>,
    #[doc = "The tokenizers for the index."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tokenizers: Vec<LexicalTokenizerUnion>,
    #[doc = "The token filters for the index."]
    #[serde(
        rename = "tokenFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_filters: Vec<TokenFilterUnion>,
    #[doc = "The character filters for the index."]
    #[serde(
        rename = "charFilters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub char_filters: Vec<CharFilterUnion>,
    #[doc = "The normalizers for the index."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub normalizers: Vec<LexicalNormalizerUnion>,
    #[doc = "A customer-managed encryption key in Azure Key Vault. Keys that you create and manage can be used to encrypt or decrypt data-at-rest, such as indexes and synonym maps."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<SearchResourceEncryptionKey>,
    #[doc = "Base type for similarity algorithms. Similarity algorithms are used to calculate scores that tie queries to documents. The higher the score, the more relevant the document is to that specific query. Those scores are used to rank the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<SimilarityUnion>,
    #[doc = "Defines parameters for a search index that influence semantic capabilities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic: Option<SemanticSettings>,
    #[doc = "Contains configuration options related to vector search."]
    #[serde(rename = "vectorSearch", default, skip_serializing_if = "Option::is_none")]
    pub vector_search: Option<VectorSearch>,
    #[doc = "The ETag of the index."]
    #[serde(rename = "@odata.etag", default, skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
}
impl SearchIndex {
    pub fn new(name: String, fields: Vec<SearchField>) -> Self {
        Self {
            name,
            fields,
            scoring_profiles: Vec::new(),
            default_scoring_profile: None,
            cors_options: None,
            suggesters: Vec::new(),
            analyzers: Vec::new(),
            tokenizers: Vec::new(),
            token_filters: Vec::new(),
            char_filters: Vec::new(),
            normalizers: Vec::new(),
            encryption_key: None,
            similarity: None,
            semantic: None,
            vector_search: None,
            odata_etag: None,
        }
    }
}
#[doc = "Represents an indexer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexer {
    #[doc = "The name of the indexer."]
    pub name: String,
    #[doc = "The description of the indexer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the datasource from which this indexer reads data."]
    #[serde(rename = "dataSourceName")]
    pub data_source_name: String,
    #[doc = "The name of the skillset executing with this indexer."]
    #[serde(rename = "skillsetName", default, skip_serializing_if = "Option::is_none")]
    pub skillset_name: Option<String>,
    #[doc = "The name of the index to which this indexer writes data."]
    #[serde(rename = "targetIndexName")]
    pub target_index_name: String,
    #[doc = "Represents a schedule for indexer execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<IndexingSchedule>,
    #[doc = "Represents parameters for indexer execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IndexingParameters>,
    #[doc = "Defines mappings between fields in the data source and corresponding target fields in the index."]
    #[serde(
        rename = "fieldMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_mappings: Vec<FieldMapping>,
    #[doc = "Output field mappings are applied after enrichment and immediately before indexing."]
    #[serde(
        rename = "outputFieldMappings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output_field_mappings: Vec<FieldMapping>,
    #[doc = "A value indicating whether the indexer is disabled. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[doc = "The ETag of the indexer."]
    #[serde(rename = "@odata.etag", default, skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
    #[doc = "A customer-managed encryption key in Azure Key Vault. Keys that you create and manage can be used to encrypt or decrypt data-at-rest, such as indexes and synonym maps."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<SearchResourceEncryptionKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<SearchIndexerCache>,
}
impl SearchIndexer {
    pub fn new(name: String, data_source_name: String, target_index_name: String) -> Self {
        Self {
            name,
            description: None,
            data_source_name,
            skillset_name: None,
            target_index_name,
            schedule: None,
            parameters: None,
            field_mappings: Vec::new(),
            output_field_mappings: Vec::new(),
            disabled: None,
            odata_etag: None,
            encryption_key: None,
            cache: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchIndexerCache {
    #[doc = "The connection string to the storage account where the cache data will be persisted."]
    #[serde(rename = "storageConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub storage_connection_string: Option<String>,
    #[doc = "Specifies whether incremental reprocessing is enabled."]
    #[serde(rename = "enableReprocessing", default, skip_serializing_if = "Option::is_none")]
    pub enable_reprocessing: Option<bool>,
    #[doc = "Abstract base type for data identities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SearchIndexerDataIdentityUnion>,
}
impl SearchIndexerCache {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents information about the entity (such as Azure SQL table or CosmosDB collection) that will be indexed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerDataContainer {
    #[doc = "The name of the table or view (for Azure SQL data source) or collection (for CosmosDB data source) that will be indexed."]
    pub name: String,
    #[doc = "A query that is applied to this data container. The syntax and meaning of this parameter is datasource-specific. Not supported by Azure SQL datasources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl SearchIndexerDataContainer {
    pub fn new(name: String) -> Self {
        Self { name, query: None }
    }
}
#[doc = "A URI fragment specifying the type of identity."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum SearchIndexerDataIdentityUnion {
    #[serde(rename = "#Microsoft.Azure.Search.DataNoneIdentity")]
    MicrosoftAzureSearchDataNoneIdentity(SearchIndexerDataNoneIdentity),
    #[serde(rename = "#Microsoft.Azure.Search.DataUserAssignedIdentity")]
    MicrosoftAzureSearchDataUserAssignedIdentity(SearchIndexerDataUserAssignedIdentity),
}
#[doc = "Clears the identity property of a datasource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerDataNoneIdentity {}
impl SearchIndexerDataNoneIdentity {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Represents a datasource definition, which can be used to configure an indexer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerDataSource {
    #[doc = "The name of the datasource."]
    pub name: String,
    #[doc = "The description of the datasource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Defines the type of a datasource."]
    #[serde(rename = "type")]
    pub type_: SearchIndexerDataSourceType,
    #[doc = "Represents credentials that can be used to connect to a datasource."]
    pub credentials: DataSourceCredentials,
    #[doc = "Represents information about the entity (such as Azure SQL table or CosmosDB collection) that will be indexed."]
    pub container: SearchIndexerDataContainer,
    #[doc = "Abstract base type for data identities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SearchIndexerDataIdentityUnion>,
    #[doc = "Base type for data change detection policies."]
    #[serde(rename = "dataChangeDetectionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub data_change_detection_policy: Option<DataChangeDetectionPolicyUnion>,
    #[doc = "Base type for data deletion detection policies."]
    #[serde(rename = "dataDeletionDetectionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub data_deletion_detection_policy: Option<DataDeletionDetectionPolicyUnion>,
    #[doc = "The ETag of the data source."]
    #[serde(rename = "@odata.etag", default, skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
    #[doc = "A customer-managed encryption key in Azure Key Vault. Keys that you create and manage can be used to encrypt or decrypt data-at-rest, such as indexes and synonym maps."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<SearchResourceEncryptionKey>,
}
impl SearchIndexerDataSource {
    pub fn new(
        name: String,
        type_: SearchIndexerDataSourceType,
        credentials: DataSourceCredentials,
        container: SearchIndexerDataContainer,
    ) -> Self {
        Self {
            name,
            description: None,
            type_,
            credentials,
            container,
            identity: None,
            data_change_detection_policy: None,
            data_deletion_detection_policy: None,
            odata_etag: None,
            encryption_key: None,
        }
    }
}
#[doc = "Defines the type of a datasource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SearchIndexerDataSourceType")]
pub enum SearchIndexerDataSourceType {
    #[serde(rename = "azuresql")]
    Azuresql,
    #[serde(rename = "cosmosdb")]
    Cosmosdb,
    #[serde(rename = "azureblob")]
    Azureblob,
    #[serde(rename = "azuretable")]
    Azuretable,
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "adlsgen2")]
    Adlsgen2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SearchIndexerDataSourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SearchIndexerDataSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SearchIndexerDataSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Azuresql => serializer.serialize_unit_variant("SearchIndexerDataSourceType", 0u32, "azuresql"),
            Self::Cosmosdb => serializer.serialize_unit_variant("SearchIndexerDataSourceType", 1u32, "cosmosdb"),
            Self::Azureblob => serializer.serialize_unit_variant("SearchIndexerDataSourceType", 2u32, "azureblob"),
            Self::Azuretable => serializer.serialize_unit_variant("SearchIndexerDataSourceType", 3u32, "azuretable"),
            Self::Mysql => serializer.serialize_unit_variant("SearchIndexerDataSourceType", 4u32, "mysql"),
            Self::Adlsgen2 => serializer.serialize_unit_variant("SearchIndexerDataSourceType", 5u32, "adlsgen2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the identity for a datasource to use."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerDataUserAssignedIdentity {
    #[doc = "The fully qualified Azure resource Id of a user assigned managed identity typically in the form \"/subscriptions/12345678-1234-1234-1234-1234567890ab/resourceGroups/rg/providers/Microsoft.ManagedIdentity/userAssignedIdentities/myId\" that should have been assigned to the search service."]
    #[serde(rename = "userAssignedIdentity")]
    pub user_assigned_identity: String,
}
impl SearchIndexerDataUserAssignedIdentity {
    pub fn new(user_assigned_identity: String) -> Self {
        Self { user_assigned_identity }
    }
}
#[doc = "Represents an item- or document-level indexing error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerError {
    #[doc = "The key of the item for which indexing failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The message describing the error that occurred while processing the item."]
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[doc = "The status code indicating why the indexing operation failed. Possible values include: 400 for a malformed input document, 404 for document not found, 409 for a version conflict, 422 when the index is temporarily unavailable, or 503 for when the service is too busy."]
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    #[doc = "The name of the source at which the error originated. For example, this could refer to a particular skill in the attached skillset. This may not be always available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Additional, verbose details about the error to assist in debugging the indexer. This may not be always available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "A link to a troubleshooting guide for these classes of errors. This may not be always available."]
    #[serde(rename = "documentationLink", default, skip_serializing_if = "Option::is_none")]
    pub documentation_link: Option<String>,
}
impl SearchIndexerError {
    pub fn new(error_message: String, status_code: i32) -> Self {
        Self {
            key: None,
            error_message,
            status_code,
            name: None,
            details: None,
            documentation_link: None,
        }
    }
}
#[doc = "Description for what data to store in the designated search index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerIndexProjectionSelector {
    #[doc = "Name of the search index to project to. Must have a key field with the 'keyword' analyzer set."]
    #[serde(rename = "targetIndexName")]
    pub target_index_name: String,
    #[doc = "Name of the field in the search index to map the parent document's key value to. Must be a string field that is filterable and not the key field."]
    #[serde(rename = "parentKeyFieldName")]
    pub parent_key_field_name: String,
    #[doc = "Source context for the projections. Represents the cardinality at which the document will be split into multiple sub documents."]
    #[serde(rename = "sourceContext")]
    pub source_context: String,
    #[doc = "Mappings for the projection, or which source should be mapped to which field in the target index."]
    pub mappings: Vec<InputFieldMappingEntry>,
}
impl SearchIndexerIndexProjectionSelector {
    pub fn new(
        target_index_name: String,
        parent_key_field_name: String,
        source_context: String,
        mappings: Vec<InputFieldMappingEntry>,
    ) -> Self {
        Self {
            target_index_name,
            parent_key_field_name,
            source_context,
            mappings,
        }
    }
}
#[doc = "Definition of additional projections to secondary search indexes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerIndexProjections {
    #[doc = "A list of projections to be performed to secondary search indexes."]
    pub selectors: Vec<SearchIndexerIndexProjectionSelector>,
    #[doc = "A dictionary of index projection-specific configuration properties. Each name is the name of a specific property. Each value must be of a primitive type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SearchIndexerIndexProjectionsParameters>,
}
impl SearchIndexerIndexProjections {
    pub fn new(selectors: Vec<SearchIndexerIndexProjectionSelector>) -> Self {
        Self {
            selectors,
            parameters: None,
        }
    }
}
#[doc = "A dictionary of index projection-specific configuration properties. Each name is the name of a specific property. Each value must be of a primitive type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchIndexerIndexProjectionsParameters {
    #[doc = "Defines behavior of the index projections in relation to the rest of the indexer."]
    #[serde(rename = "projectionMode", default, skip_serializing_if = "Option::is_none")]
    pub projection_mode: Option<IndexProjectionMode>,
}
impl SearchIndexerIndexProjectionsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of additional projections to azure blob, table, or files, of enriched data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerKnowledgeStore {
    #[doc = "The connection string to the storage account projections will be stored in."]
    #[serde(rename = "storageConnectionString")]
    pub storage_connection_string: String,
    #[doc = "A list of additional projections to perform during indexing."]
    pub projections: Vec<SearchIndexerKnowledgeStoreProjection>,
    #[doc = "Abstract base type for data identities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SearchIndexerDataIdentityUnion>,
    #[doc = "A dictionary of knowledge store-specific configuration properties. Each name is the name of a specific property. Each value must be of a primitive type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SearchIndexerKnowledgeStoreParameters>,
}
impl SearchIndexerKnowledgeStore {
    pub fn new(storage_connection_string: String, projections: Vec<SearchIndexerKnowledgeStoreProjection>) -> Self {
        Self {
            storage_connection_string,
            projections,
            identity: None,
            parameters: None,
        }
    }
}
#[doc = "Abstract class to share properties between concrete selectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerKnowledgeStoreBlobProjectionSelector {
    #[serde(flatten)]
    pub search_indexer_knowledge_store_projection_selector: SearchIndexerKnowledgeStoreProjectionSelector,
    #[doc = "Blob container to store projections in."]
    #[serde(rename = "storageContainer")]
    pub storage_container: String,
}
impl SearchIndexerKnowledgeStoreBlobProjectionSelector {
    pub fn new(storage_container: String) -> Self {
        Self {
            search_indexer_knowledge_store_projection_selector: SearchIndexerKnowledgeStoreProjectionSelector::default(),
            storage_container,
        }
    }
}
#[doc = "Projection definition for what data to store in Azure Files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerKnowledgeStoreFileProjectionSelector {
    #[serde(flatten)]
    pub search_indexer_knowledge_store_blob_projection_selector: SearchIndexerKnowledgeStoreBlobProjectionSelector,
}
impl SearchIndexerKnowledgeStoreFileProjectionSelector {
    pub fn new(search_indexer_knowledge_store_blob_projection_selector: SearchIndexerKnowledgeStoreBlobProjectionSelector) -> Self {
        Self {
            search_indexer_knowledge_store_blob_projection_selector,
        }
    }
}
#[doc = "Projection definition for what data to store in Azure Blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerKnowledgeStoreObjectProjectionSelector {
    #[serde(flatten)]
    pub search_indexer_knowledge_store_blob_projection_selector: SearchIndexerKnowledgeStoreBlobProjectionSelector,
}
impl SearchIndexerKnowledgeStoreObjectProjectionSelector {
    pub fn new(search_indexer_knowledge_store_blob_projection_selector: SearchIndexerKnowledgeStoreBlobProjectionSelector) -> Self {
        Self {
            search_indexer_knowledge_store_blob_projection_selector,
        }
    }
}
#[doc = "A dictionary of knowledge store-specific configuration properties. Each name is the name of a specific property. Each value must be of a primitive type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchIndexerKnowledgeStoreParameters {
    #[doc = "Whether or not projections should synthesize a generated key name if one isn't already present."]
    #[serde(rename = "synthesizeGeneratedKeyName", default, skip_serializing_if = "Option::is_none")]
    pub synthesize_generated_key_name: Option<bool>,
}
impl SearchIndexerKnowledgeStoreParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container object for various projection selectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchIndexerKnowledgeStoreProjection {
    #[doc = "Projections to Azure Table storage."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tables: Vec<SearchIndexerKnowledgeStoreTableProjectionSelector>,
    #[doc = "Projections to Azure Blob storage."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub objects: Vec<SearchIndexerKnowledgeStoreObjectProjectionSelector>,
    #[doc = "Projections to Azure File storage."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<SearchIndexerKnowledgeStoreFileProjectionSelector>,
}
impl SearchIndexerKnowledgeStoreProjection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Abstract class to share properties between concrete selectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchIndexerKnowledgeStoreProjectionSelector {
    #[doc = "Name of reference key to different projection."]
    #[serde(rename = "referenceKeyName", default, skip_serializing_if = "Option::is_none")]
    pub reference_key_name: Option<String>,
    #[doc = "Name of generated key to store projection under."]
    #[serde(rename = "generatedKeyName", default, skip_serializing_if = "Option::is_none")]
    pub generated_key_name: Option<String>,
    #[doc = "Source data to project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Source context for complex projections."]
    #[serde(rename = "sourceContext", default, skip_serializing_if = "Option::is_none")]
    pub source_context: Option<String>,
    #[doc = "Nested inputs for complex projections."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<InputFieldMappingEntry>,
}
impl SearchIndexerKnowledgeStoreProjectionSelector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description for what data to store in Azure Tables."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerKnowledgeStoreTableProjectionSelector {
    #[serde(flatten)]
    pub search_indexer_knowledge_store_projection_selector: SearchIndexerKnowledgeStoreProjectionSelector,
    #[doc = "Name of the Azure table to store projected data in."]
    #[serde(rename = "tableName")]
    pub table_name: String,
}
impl SearchIndexerKnowledgeStoreTableProjectionSelector {
    pub fn new(table_name: String) -> Self {
        Self {
            search_indexer_knowledge_store_projection_selector: SearchIndexerKnowledgeStoreProjectionSelector::default(),
            table_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchIndexerLimits {
    #[doc = "The maximum duration that the indexer is permitted to run for one execution."]
    #[serde(rename = "maxRunTime", default, skip_serializing_if = "Option::is_none")]
    pub max_run_time: Option<String>,
    #[doc = "The maximum size of a document, in bytes, which will be considered valid for indexing."]
    #[serde(rename = "maxDocumentExtractionSize", default, skip_serializing_if = "Option::is_none")]
    pub max_document_extraction_size: Option<f64>,
    #[doc = "The maximum number of characters that will be extracted from a document picked up for indexing."]
    #[serde(rename = "maxDocumentContentCharactersToExtract", default, skip_serializing_if = "Option::is_none")]
    pub max_document_content_characters_to_extract: Option<f64>,
}
impl SearchIndexerLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base type for skills."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerSkill {
    #[doc = "The name of the skill which uniquely identifies it within the skillset. A skill with no name defined will be given a default name of its 1-based index in the skills array, prefixed with the character '#'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The description of the skill which describes the inputs, outputs, and usage of the skill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Represents the level at which operations take place, such as the document root or document content (for example, /document or /document/content). The default is /document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Inputs of the skills could be a column in the source data set, or the output of an upstream skill."]
    pub inputs: Vec<InputFieldMappingEntry>,
    #[doc = "The output of a skill is either a field in a search index, or a value that can be consumed as an input by another skill."]
    pub outputs: Vec<OutputFieldMappingEntry>,
}
impl SearchIndexerSkill {
    pub fn new(inputs: Vec<InputFieldMappingEntry>, outputs: Vec<OutputFieldMappingEntry>) -> Self {
        Self {
            name: None,
            description: None,
            context: None,
            inputs,
            outputs,
        }
    }
}
#[doc = "A URI fragment specifying the type of skill."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum SearchIndexerSkillUnion {
    #[serde(rename = "#Microsoft.Skills.Custom.AmlSkill")]
    MicrosoftSkillsCustomAmlSkill(AmlSkill),
    #[serde(rename = "#Microsoft.Skills.Text.AzureOpenAIEmbeddingSkill")]
    MicrosoftSkillsTextAzureOpenAiEmbeddingSkill(AzureOpenAiEmbeddingSkill),
    #[serde(rename = "#Microsoft.Skills.Util.ConditionalSkill")]
    MicrosoftSkillsUtilConditionalSkill(ConditionalSkill),
    #[serde(rename = "#Microsoft.Skills.Text.CustomEntityLookupSkill")]
    MicrosoftSkillsTextCustomEntityLookupSkill(CustomEntityLookupSkill),
    #[serde(rename = "#Microsoft.Skills.Util.DocumentExtractionSkill")]
    MicrosoftSkillsUtilDocumentExtractionSkill(DocumentExtractionSkill),
    #[serde(rename = "#Microsoft.Skills.Text.V3.EntityLinkingSkill")]
    MicrosoftSkillsTextV3EntityLinkingSkill(EntityLinkingSkill),
    #[serde(rename = "#Microsoft.Skills.Text.EntityRecognitionSkill")]
    MicrosoftSkillsTextEntityRecognitionSkill(EntityRecognitionSkill),
    #[serde(rename = "#Microsoft.Skills.Text.V3.EntityRecognitionSkill")]
    MicrosoftSkillsTextV3EntityRecognitionSkill(EntityRecognitionSkillV3),
    #[serde(rename = "#Microsoft.Skills.Vision.ImageAnalysisSkill")]
    MicrosoftSkillsVisionImageAnalysisSkill(ImageAnalysisSkill),
    #[serde(rename = "#Microsoft.Skills.Text.KeyPhraseExtractionSkill")]
    MicrosoftSkillsTextKeyPhraseExtractionSkill(KeyPhraseExtractionSkill),
    #[serde(rename = "#Microsoft.Skills.Text.LanguageDetectionSkill")]
    MicrosoftSkillsTextLanguageDetectionSkill(LanguageDetectionSkill),
    #[serde(rename = "#Microsoft.Skills.Text.MergeSkill")]
    MicrosoftSkillsTextMergeSkill(MergeSkill),
    #[serde(rename = "#Microsoft.Skills.Vision.OcrSkill")]
    MicrosoftSkillsVisionOcrSkill(OcrSkill),
    #[serde(rename = "#Microsoft.Skills.Text.PIIDetectionSkill")]
    MicrosoftSkillsTextPiiDetectionSkill(PiiDetectionSkill),
    #[serde(rename = "#Microsoft.Skills.Text.SentimentSkill")]
    MicrosoftSkillsTextSentimentSkill(SentimentSkill),
    #[serde(rename = "#Microsoft.Skills.Text.V3.SentimentSkill")]
    MicrosoftSkillsTextV3SentimentSkill(SentimentSkillV3),
    #[serde(rename = "#Microsoft.Skills.Util.ShaperSkill")]
    MicrosoftSkillsUtilShaperSkill(ShaperSkill),
    #[serde(rename = "#Microsoft.Skills.Text.SplitSkill")]
    MicrosoftSkillsTextSplitSkill(SplitSkill),
    #[serde(rename = "#Microsoft.Skills.Text.TranslationSkill")]
    MicrosoftSkillsTextTranslationSkill(TextTranslationSkill),
    #[serde(rename = "#Microsoft.Skills.Custom.WebApiSkill")]
    MicrosoftSkillsCustomWebApiSkill(WebApiSkill),
}
#[doc = "A list of skills."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerSkillset {
    #[doc = "The name of the skillset."]
    pub name: String,
    #[doc = "The description of the skillset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A list of skills in the skillset."]
    pub skills: Vec<SearchIndexerSkillUnion>,
    #[doc = "Base type for describing any Azure AI service resource attached to a skillset."]
    #[serde(rename = "cognitiveServices", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_services: Option<CognitiveServicesAccountUnion>,
    #[doc = "Definition of additional projections to azure blob, table, or files, of enriched data."]
    #[serde(rename = "knowledgeStore", default, skip_serializing_if = "Option::is_none")]
    pub knowledge_store: Option<SearchIndexerKnowledgeStore>,
    #[doc = "Definition of additional projections to secondary search indexes."]
    #[serde(rename = "indexProjections", default, skip_serializing_if = "Option::is_none")]
    pub index_projections: Option<SearchIndexerIndexProjections>,
    #[doc = "The ETag of the skillset."]
    #[serde(rename = "@odata.etag", default, skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
    #[doc = "A customer-managed encryption key in Azure Key Vault. Keys that you create and manage can be used to encrypt or decrypt data-at-rest, such as indexes and synonym maps."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<SearchResourceEncryptionKey>,
}
impl SearchIndexerSkillset {
    pub fn new(name: String, skills: Vec<SearchIndexerSkillUnion>) -> Self {
        Self {
            name,
            description: None,
            skills,
            cognitive_services: None,
            knowledge_store: None,
            index_projections: None,
            odata_etag: None,
            encryption_key: None,
        }
    }
}
#[doc = "Represents the current status and execution history of an indexer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerStatus {
    #[doc = "Represents the overall indexer status."]
    pub status: IndexerStatus,
    #[doc = "Represents the result of an individual indexer execution."]
    #[serde(rename = "lastResult", default, skip_serializing_if = "Option::is_none")]
    pub last_result: Option<IndexerExecutionResult>,
    #[doc = "History of the recent indexer executions, sorted in reverse chronological order."]
    #[serde(rename = "executionHistory")]
    pub execution_history: Vec<IndexerExecutionResult>,
    pub limits: SearchIndexerLimits,
}
impl SearchIndexerStatus {
    pub fn new(status: IndexerStatus, execution_history: Vec<IndexerExecutionResult>, limits: SearchIndexerLimits) -> Self {
        Self {
            status,
            last_result: None,
            execution_history,
            limits,
        }
    }
}
#[doc = "Represents an item-level warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchIndexerWarning {
    #[doc = "The key of the item which generated a warning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The message describing the warning that occurred while processing the item."]
    pub message: String,
    #[doc = "The name of the source at which the warning originated. For example, this could refer to a particular skill in the attached skillset. This may not be always available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Additional, verbose details about the warning to assist in debugging the indexer. This may not be always available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "A link to a troubleshooting guide for these classes of warnings. This may not be always available."]
    #[serde(rename = "documentationLink", default, skip_serializing_if = "Option::is_none")]
    pub documentation_link: Option<String>,
}
impl SearchIndexerWarning {
    pub fn new(message: String) -> Self {
        Self {
            key: None,
            message,
            name: None,
            details: None,
            documentation_link: None,
        }
    }
}
#[doc = "A customer-managed encryption key in Azure Key Vault. Keys that you create and manage can be used to encrypt or decrypt data-at-rest, such as indexes and synonym maps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResourceEncryptionKey {
    #[doc = "The name of your Azure Key Vault key to be used to encrypt your data at rest."]
    #[serde(rename = "keyVaultKeyName")]
    pub key_vault_key_name: String,
    #[doc = "The version of your Azure Key Vault key to be used to encrypt your data at rest."]
    #[serde(rename = "keyVaultKeyVersion")]
    pub key_vault_key_version: String,
    #[doc = "The URI of your Azure Key Vault, also referred to as DNS name, that contains the key to be used to encrypt your data at rest. An example URI might be `https://my-keyvault-name.vault.azure.net`."]
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
    #[doc = "Credentials of a registered application created for your search service, used for authenticated access to the encryption keys stored in Azure Key Vault."]
    #[serde(rename = "accessCredentials", default, skip_serializing_if = "Option::is_none")]
    pub access_credentials: Option<AzureActiveDirectoryApplicationCredentials>,
    #[doc = "Abstract base type for data identities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SearchIndexerDataIdentityUnion>,
}
impl SearchResourceEncryptionKey {
    pub fn new(key_vault_key_name: String, key_vault_key_version: String, key_vault_uri: String) -> Self {
        Self {
            key_vault_key_name,
            key_vault_key_version,
            key_vault_uri,
            access_credentials: None,
            identity: None,
        }
    }
}
#[doc = "Defines a specific configuration to be used in the context of semantic capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemanticConfiguration {
    #[doc = "The name of the semantic configuration."]
    pub name: String,
    #[doc = "Describes the title, content, and keywords fields to be used for semantic ranking, captions, highlights, and answers."]
    #[serde(rename = "prioritizedFields")]
    pub prioritized_fields: PrioritizedFields,
}
impl SemanticConfiguration {
    pub fn new(name: String, prioritized_fields: PrioritizedFields) -> Self {
        Self { name, prioritized_fields }
    }
}
#[doc = "A field that is used as part of the semantic configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemanticField {
    #[doc = ""]
    #[serde(rename = "fieldName")]
    pub field_name: String,
}
impl SemanticField {
    pub fn new(field_name: String) -> Self {
        Self { field_name }
    }
}
#[doc = "Defines parameters for a search index that influence semantic capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SemanticSettings {
    #[doc = "Allows you to set the name of a default semantic configuration in your index, making it optional to pass it on as a query parameter every time."]
    #[serde(rename = "defaultConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub default_configuration: Option<String>,
    #[doc = "The semantic configurations for the index."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<SemanticConfiguration>,
}
impl SemanticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This skill is deprecated. Use the V3.SentimentSkill instead."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentimentSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "Deprecated. The language codes supported for input text by SentimentSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<SentimentSkillLanguage>,
}
impl SentimentSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
        }
    }
}
#[doc = "Deprecated. The language codes supported for input text by SentimentSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SentimentSkillLanguage")]
pub enum SentimentSkillLanguage {
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SentimentSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SentimentSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SentimentSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Da => serializer.serialize_unit_variant("SentimentSkillLanguage", 0u32, "da"),
            Self::Nl => serializer.serialize_unit_variant("SentimentSkillLanguage", 1u32, "nl"),
            Self::En => serializer.serialize_unit_variant("SentimentSkillLanguage", 2u32, "en"),
            Self::Fi => serializer.serialize_unit_variant("SentimentSkillLanguage", 3u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("SentimentSkillLanguage", 4u32, "fr"),
            Self::De => serializer.serialize_unit_variant("SentimentSkillLanguage", 5u32, "de"),
            Self::El => serializer.serialize_unit_variant("SentimentSkillLanguage", 6u32, "el"),
            Self::It => serializer.serialize_unit_variant("SentimentSkillLanguage", 7u32, "it"),
            Self::No => serializer.serialize_unit_variant("SentimentSkillLanguage", 8u32, "no"),
            Self::Pl => serializer.serialize_unit_variant("SentimentSkillLanguage", 9u32, "pl"),
            Self::PtPt => serializer.serialize_unit_variant("SentimentSkillLanguage", 10u32, "pt-PT"),
            Self::Ru => serializer.serialize_unit_variant("SentimentSkillLanguage", 11u32, "ru"),
            Self::Es => serializer.serialize_unit_variant("SentimentSkillLanguage", 12u32, "es"),
            Self::Sv => serializer.serialize_unit_variant("SentimentSkillLanguage", 13u32, "sv"),
            Self::Tr => serializer.serialize_unit_variant("SentimentSkillLanguage", 14u32, "tr"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Using the Text Analytics API, evaluates unstructured text and for each record, provides sentiment labels (such as \"negative\", \"neutral\" and \"positive\") based on the highest confidence score found by the service at a sentence and document-level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentimentSkillV3 {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "A value indicating which language code to use. Default is `en`."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<String>,
    #[doc = "If set to true, the skill output will include information from Text Analytics for opinion mining, namely targets (nouns or verbs) and their associated assessment (adjective) in the text. Default is false."]
    #[serde(rename = "includeOpinionMining", default, skip_serializing_if = "Option::is_none")]
    pub include_opinion_mining: Option<bool>,
    #[doc = "The version of the model to use when calling the Text Analytics service. It will default to the latest available when not specified. We recommend you do not specify this value unless absolutely necessary."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
}
impl SentimentSkillV3 {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            include_opinion_mining: None,
            model_version: None,
        }
    }
}
#[doc = "Represents service-level resource counters and quotas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCounters {
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "aliasesCount")]
    pub aliases_count: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "documentCount")]
    pub document_count: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "indexesCount")]
    pub indexes_count: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "indexersCount")]
    pub indexers_count: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "dataSourcesCount")]
    pub data_sources_count: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "storageSize")]
    pub storage_size: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "synonymMaps")]
    pub synonym_maps: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "skillsetCount")]
    pub skillset_count: ResourceCounter,
    #[doc = "Represents a resource's usage and quota."]
    #[serde(rename = "vectorIndexSize")]
    pub vector_index_size: ResourceCounter,
}
impl ServiceCounters {
    pub fn new(
        aliases_count: ResourceCounter,
        document_count: ResourceCounter,
        indexes_count: ResourceCounter,
        indexers_count: ResourceCounter,
        data_sources_count: ResourceCounter,
        storage_size: ResourceCounter,
        synonym_maps: ResourceCounter,
        skillset_count: ResourceCounter,
        vector_index_size: ResourceCounter,
    ) -> Self {
        Self {
            aliases_count,
            document_count,
            indexes_count,
            indexers_count,
            data_sources_count,
            storage_size,
            synonym_maps,
            skillset_count,
            vector_index_size,
        }
    }
}
#[doc = "Represents various service level limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceLimits {
    #[doc = "The maximum allowed fields per index."]
    #[serde(rename = "maxFieldsPerIndex", default, skip_serializing_if = "Option::is_none")]
    pub max_fields_per_index: Option<i32>,
    #[doc = "The maximum depth which you can nest sub-fields in an index, including the top-level complex field. For example, a/b/c has a nesting depth of 3."]
    #[serde(rename = "maxFieldNestingDepthPerIndex", default, skip_serializing_if = "Option::is_none")]
    pub max_field_nesting_depth_per_index: Option<i32>,
    #[doc = "The maximum number of fields of type Collection(Edm.ComplexType) allowed in an index."]
    #[serde(rename = "maxComplexCollectionFieldsPerIndex", default, skip_serializing_if = "Option::is_none")]
    pub max_complex_collection_fields_per_index: Option<i32>,
    #[doc = "The maximum number of objects in complex collections allowed per document."]
    #[serde(
        rename = "maxComplexObjectsInCollectionsPerDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_complex_objects_in_collections_per_document: Option<i32>,
}
impl ServiceLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response from a get service statistics request. If successful, it includes service level counters and limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStatistics {
    #[doc = "Represents service-level resource counters and quotas."]
    pub counters: ServiceCounters,
    #[doc = "Represents various service level limits."]
    pub limits: ServiceLimits,
}
impl ServiceStatistics {
    pub fn new(counters: ServiceCounters, limits: ServiceLimits) -> Self {
        Self { counters, limits }
    }
}
#[doc = "A skill for reshaping the outputs. It creates a complex type to support composite fields (also known as multipart fields)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShaperSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
}
impl ShaperSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self { search_indexer_skill }
    }
}
#[doc = "Creates combinations of tokens as a single token. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShingleTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The maximum shingle size. Default and minimum value is 2."]
    #[serde(rename = "maxShingleSize", default, skip_serializing_if = "Option::is_none")]
    pub max_shingle_size: Option<i32>,
    #[doc = "The minimum shingle size. Default and minimum value is 2. Must be less than the value of maxShingleSize."]
    #[serde(rename = "minShingleSize", default, skip_serializing_if = "Option::is_none")]
    pub min_shingle_size: Option<i32>,
    #[doc = "A value indicating whether the output stream will contain the input tokens (unigrams) as well as shingles. Default is true."]
    #[serde(rename = "outputUnigrams", default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams: Option<bool>,
    #[doc = "A value indicating whether to output unigrams for those times when no shingles are available. This property takes precedence when outputUnigrams is set to false. Default is false."]
    #[serde(rename = "outputUnigramsIfNoShingles", default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams_if_no_shingles: Option<bool>,
    #[doc = "The string to use when joining adjacent tokens to form a shingle. Default is a single space (\" \")."]
    #[serde(rename = "tokenSeparator", default, skip_serializing_if = "Option::is_none")]
    pub token_separator: Option<String>,
    #[doc = "The string to insert for each position at which there is no token. Default is an underscore (\"_\")."]
    #[serde(rename = "filterToken", default, skip_serializing_if = "Option::is_none")]
    pub filter_token: Option<String>,
}
impl ShingleTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            max_shingle_size: None,
            min_shingle_size: None,
            output_unigrams: None,
            output_unigrams_if_no_shingles: None,
            token_separator: None,
            filter_token: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum SimilarityUnion {
    #[serde(rename = "#Microsoft.Azure.Search.BM25Similarity")]
    MicrosoftAzureSearchBm25Similarity(Bm25Similarity),
    #[serde(rename = "#Microsoft.Azure.Search.ClassicSimilarity")]
    MicrosoftAzureSearchClassicSimilarity(ClassicSimilarity),
}
#[doc = "A filter that stems words using a Snowball-generated stemmer. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnowballTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The language to use for a Snowball token filter."]
    pub language: SnowballTokenFilterLanguage,
}
impl SnowballTokenFilter {
    pub fn new(token_filter: TokenFilter, language: SnowballTokenFilterLanguage) -> Self {
        Self { token_filter, language }
    }
}
#[doc = "The language to use for a Snowball token filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SnowballTokenFilterLanguage {
    #[serde(rename = "armenian")]
    Armenian,
    #[serde(rename = "basque")]
    Basque,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "finnish")]
    Finnish,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "german2")]
    German2,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "kp")]
    Kp,
    #[serde(rename = "lovins")]
    Lovins,
    #[serde(rename = "norwegian")]
    Norwegian,
    #[serde(rename = "porter")]
    Porter,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "turkish")]
    Turkish,
}
#[doc = "Defines a data deletion detection policy that implements a soft-deletion strategy. It determines whether an item should be deleted based on the value of a designated 'soft delete' column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftDeleteColumnDeletionDetectionPolicy {
    #[doc = "The name of the column to use for soft-deletion detection."]
    #[serde(rename = "softDeleteColumnName", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_column_name: Option<String>,
    #[doc = "The marker value that identifies an item as deleted."]
    #[serde(rename = "softDeleteMarkerValue", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_marker_value: Option<String>,
}
impl SoftDeleteColumnDeletionDetectionPolicy {
    pub fn new() -> Self {
        Self {
            soft_delete_column_name: None,
            soft_delete_marker_value: None,
        }
    }
}
#[doc = "A skill to split a string into chunks of text."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The language codes supported for input text by SplitSkill."]
    #[serde(rename = "defaultLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<SplitSkillLanguage>,
    #[doc = "A value indicating which split mode to perform."]
    #[serde(rename = "textSplitMode", default, skip_serializing_if = "Option::is_none")]
    pub text_split_mode: Option<TextSplitMode>,
    #[doc = "The desired maximum page length. Default is 10000."]
    #[serde(rename = "maximumPageLength", default, skip_serializing_if = "Option::is_none")]
    pub maximum_page_length: Option<i32>,
    #[doc = "Only applicable when textSplitMode is set to 'pages'. If specified, n+1th chunk will start with this number of characters/tokens from the end of the nth chunk."]
    #[serde(rename = "pageOverlapLength", default, skip_serializing_if = "Option::is_none")]
    pub page_overlap_length: Option<i32>,
    #[doc = "Only applicable when textSplitMode is set to 'pages'. If specified, the SplitSkill will discontinue splitting after processing the first 'maximumPagesToTake' pages, in order to improve performance when only a few initial pages are needed from each document."]
    #[serde(rename = "maximumPagesToTake", default, skip_serializing_if = "Option::is_none")]
    pub maximum_pages_to_take: Option<i32>,
}
impl SplitSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill) -> Self {
        Self {
            search_indexer_skill,
            default_language_code: None,
            text_split_mode: None,
            maximum_page_length: None,
            page_overlap_length: None,
            maximum_pages_to_take: None,
        }
    }
}
#[doc = "The language codes supported for input text by SplitSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SplitSkillLanguage")]
pub enum SplitSkillLanguage {
    #[serde(rename = "am")]
    Am,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "zh")]
    Zh,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SplitSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SplitSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SplitSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Am => serializer.serialize_unit_variant("SplitSkillLanguage", 0u32, "am"),
            Self::Bs => serializer.serialize_unit_variant("SplitSkillLanguage", 1u32, "bs"),
            Self::Cs => serializer.serialize_unit_variant("SplitSkillLanguage", 2u32, "cs"),
            Self::Da => serializer.serialize_unit_variant("SplitSkillLanguage", 3u32, "da"),
            Self::De => serializer.serialize_unit_variant("SplitSkillLanguage", 4u32, "de"),
            Self::En => serializer.serialize_unit_variant("SplitSkillLanguage", 5u32, "en"),
            Self::Es => serializer.serialize_unit_variant("SplitSkillLanguage", 6u32, "es"),
            Self::Et => serializer.serialize_unit_variant("SplitSkillLanguage", 7u32, "et"),
            Self::Fi => serializer.serialize_unit_variant("SplitSkillLanguage", 8u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("SplitSkillLanguage", 9u32, "fr"),
            Self::He => serializer.serialize_unit_variant("SplitSkillLanguage", 10u32, "he"),
            Self::Hi => serializer.serialize_unit_variant("SplitSkillLanguage", 11u32, "hi"),
            Self::Hr => serializer.serialize_unit_variant("SplitSkillLanguage", 12u32, "hr"),
            Self::Hu => serializer.serialize_unit_variant("SplitSkillLanguage", 13u32, "hu"),
            Self::Id => serializer.serialize_unit_variant("SplitSkillLanguage", 14u32, "id"),
            Self::Is => serializer.serialize_unit_variant("SplitSkillLanguage", 15u32, "is"),
            Self::It => serializer.serialize_unit_variant("SplitSkillLanguage", 16u32, "it"),
            Self::Ja => serializer.serialize_unit_variant("SplitSkillLanguage", 17u32, "ja"),
            Self::Ko => serializer.serialize_unit_variant("SplitSkillLanguage", 18u32, "ko"),
            Self::Lv => serializer.serialize_unit_variant("SplitSkillLanguage", 19u32, "lv"),
            Self::Nb => serializer.serialize_unit_variant("SplitSkillLanguage", 20u32, "nb"),
            Self::Nl => serializer.serialize_unit_variant("SplitSkillLanguage", 21u32, "nl"),
            Self::Pl => serializer.serialize_unit_variant("SplitSkillLanguage", 22u32, "pl"),
            Self::Pt => serializer.serialize_unit_variant("SplitSkillLanguage", 23u32, "pt"),
            Self::PtBr => serializer.serialize_unit_variant("SplitSkillLanguage", 24u32, "pt-br"),
            Self::Ru => serializer.serialize_unit_variant("SplitSkillLanguage", 25u32, "ru"),
            Self::Sk => serializer.serialize_unit_variant("SplitSkillLanguage", 26u32, "sk"),
            Self::Sl => serializer.serialize_unit_variant("SplitSkillLanguage", 27u32, "sl"),
            Self::Sr => serializer.serialize_unit_variant("SplitSkillLanguage", 28u32, "sr"),
            Self::Sv => serializer.serialize_unit_variant("SplitSkillLanguage", 29u32, "sv"),
            Self::Tr => serializer.serialize_unit_variant("SplitSkillLanguage", 30u32, "tr"),
            Self::Ur => serializer.serialize_unit_variant("SplitSkillLanguage", 31u32, "ur"),
            Self::Zh => serializer.serialize_unit_variant("SplitSkillLanguage", 32u32, "zh"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines a data change detection policy that captures changes using the Integrated Change Tracking feature of Azure SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlIntegratedChangeTrackingPolicy {}
impl SqlIntegratedChangeTrackingPolicy {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Provides the ability to override other stemming filters with custom dictionary-based stemming. Any dictionary-stemmed terms will be marked as keywords so that they will not be stemmed with stemmers down the chain. Must be placed before any stemming filters. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StemmerOverrideTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A list of stemming rules in the following format: \"word => stem\", for example: \"ran => run\"."]
    pub rules: Vec<String>,
}
impl StemmerOverrideTokenFilter {
    pub fn new(token_filter: TokenFilter, rules: Vec<String>) -> Self {
        Self { token_filter, rules }
    }
}
#[doc = "Language specific stemming filter. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StemmerTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The language to use for a stemmer token filter."]
    pub language: StemmerTokenFilterLanguage,
}
impl StemmerTokenFilter {
    pub fn new(token_filter: TokenFilter, language: StemmerTokenFilterLanguage) -> Self {
        Self { token_filter, language }
    }
}
#[doc = "The language to use for a stemmer token filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StemmerTokenFilterLanguage {
    #[serde(rename = "arabic")]
    Arabic,
    #[serde(rename = "armenian")]
    Armenian,
    #[serde(rename = "basque")]
    Basque,
    #[serde(rename = "brazilian")]
    Brazilian,
    #[serde(rename = "bulgarian")]
    Bulgarian,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "czech")]
    Czech,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "dutchKp")]
    DutchKp,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "lightEnglish")]
    LightEnglish,
    #[serde(rename = "minimalEnglish")]
    MinimalEnglish,
    #[serde(rename = "possessiveEnglish")]
    PossessiveEnglish,
    #[serde(rename = "porter2")]
    Porter2,
    #[serde(rename = "lovins")]
    Lovins,
    #[serde(rename = "finnish")]
    Finnish,
    #[serde(rename = "lightFinnish")]
    LightFinnish,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "lightFrench")]
    LightFrench,
    #[serde(rename = "minimalFrench")]
    MinimalFrench,
    #[serde(rename = "galician")]
    Galician,
    #[serde(rename = "minimalGalician")]
    MinimalGalician,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "german2")]
    German2,
    #[serde(rename = "lightGerman")]
    LightGerman,
    #[serde(rename = "minimalGerman")]
    MinimalGerman,
    #[serde(rename = "greek")]
    Greek,
    #[serde(rename = "hindi")]
    Hindi,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "lightHungarian")]
    LightHungarian,
    #[serde(rename = "indonesian")]
    Indonesian,
    #[serde(rename = "irish")]
    Irish,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "lightItalian")]
    LightItalian,
    #[serde(rename = "sorani")]
    Sorani,
    #[serde(rename = "latvian")]
    Latvian,
    #[serde(rename = "norwegian")]
    Norwegian,
    #[serde(rename = "lightNorwegian")]
    LightNorwegian,
    #[serde(rename = "minimalNorwegian")]
    MinimalNorwegian,
    #[serde(rename = "lightNynorsk")]
    LightNynorsk,
    #[serde(rename = "minimalNynorsk")]
    MinimalNynorsk,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "lightPortuguese")]
    LightPortuguese,
    #[serde(rename = "minimalPortuguese")]
    MinimalPortuguese,
    #[serde(rename = "portugueseRslp")]
    PortugueseRslp,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "lightRussian")]
    LightRussian,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "lightSpanish")]
    LightSpanish,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "lightSwedish")]
    LightSwedish,
    #[serde(rename = "turkish")]
    Turkish,
}
#[doc = "Divides text at non-letters; Applies the lowercase and stopword token filters. This analyzer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StopAnalyzer {
    #[serde(flatten)]
    pub lexical_analyzer: LexicalAnalyzer,
    #[doc = "A list of stopwords."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stopwords: Vec<String>,
}
impl StopAnalyzer {
    pub fn new(lexical_analyzer: LexicalAnalyzer) -> Self {
        Self {
            lexical_analyzer,
            stopwords: Vec::new(),
        }
    }
}
#[doc = "Identifies a predefined list of language-specific stopwords."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StopwordsList {
    #[serde(rename = "arabic")]
    Arabic,
    #[serde(rename = "armenian")]
    Armenian,
    #[serde(rename = "basque")]
    Basque,
    #[serde(rename = "brazilian")]
    Brazilian,
    #[serde(rename = "bulgarian")]
    Bulgarian,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "czech")]
    Czech,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "finnish")]
    Finnish,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "galician")]
    Galician,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "greek")]
    Greek,
    #[serde(rename = "hindi")]
    Hindi,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "indonesian")]
    Indonesian,
    #[serde(rename = "irish")]
    Irish,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "latvian")]
    Latvian,
    #[serde(rename = "norwegian")]
    Norwegian,
    #[serde(rename = "persian")]
    Persian,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "sorani")]
    Sorani,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "thai")]
    Thai,
    #[serde(rename = "turkish")]
    Turkish,
}
#[doc = "Removes stop words from a token stream. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StopwordsTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The list of stopwords. This property and the stopwords list property cannot both be set."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stopwords: Vec<String>,
    #[doc = "Identifies a predefined list of language-specific stopwords."]
    #[serde(rename = "stopwordsList", default, skip_serializing_if = "Option::is_none")]
    pub stopwords_list: Option<StopwordsList>,
    #[doc = "A value indicating whether to ignore case. If true, all words are converted to lower case first. Default is false."]
    #[serde(rename = "ignoreCase", default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[doc = "A value indicating whether to ignore the last search term if it's a stop word. Default is true."]
    #[serde(rename = "removeTrailing", default, skip_serializing_if = "Option::is_none")]
    pub remove_trailing: Option<bool>,
}
impl StopwordsTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            stopwords: Vec::new(),
            stopwords_list: None,
            ignore_case: None,
            remove_trailing: None,
        }
    }
}
#[doc = "Defines how the Suggest API should apply to a group of fields in the index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Suggester {
    #[doc = "The name of the suggester."]
    pub name: String,
    #[doc = "A value indicating the capabilities of the suggester."]
    #[serde(rename = "searchMode")]
    pub search_mode: suggester::SearchMode,
    #[doc = "The list of field names to which the suggester applies. Each field must be searchable."]
    #[serde(rename = "sourceFields")]
    pub source_fields: Vec<String>,
}
impl Suggester {
    pub fn new(name: String, search_mode: suggester::SearchMode, source_fields: Vec<String>) -> Self {
        Self {
            name,
            search_mode,
            source_fields,
        }
    }
}
pub mod suggester {
    use super::*;
    #[doc = "A value indicating the capabilities of the suggester."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SearchMode {
        #[serde(rename = "analyzingInfixMatching")]
        AnalyzingInfixMatching,
    }
}
#[doc = "Represents a synonym map definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynonymMap {
    #[doc = "The name of the synonym map."]
    pub name: String,
    #[doc = "The format of the synonym map. Only the 'solr' format is currently supported."]
    pub format: synonym_map::Format,
    #[doc = "A series of synonym rules in the specified synonym map format. The rules must be separated by newlines."]
    pub synonyms: String,
    #[doc = "A customer-managed encryption key in Azure Key Vault. Keys that you create and manage can be used to encrypt or decrypt data-at-rest, such as indexes and synonym maps."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<SearchResourceEncryptionKey>,
    #[doc = "The ETag of the synonym map."]
    #[serde(rename = "@odata.etag", default, skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
}
impl SynonymMap {
    pub fn new(name: String, format: synonym_map::Format, synonyms: String) -> Self {
        Self {
            name,
            format,
            synonyms,
            encryption_key: None,
            odata_etag: None,
        }
    }
}
pub mod synonym_map {
    use super::*;
    #[doc = "The format of the synonym map. Only the 'solr' format is currently supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        #[serde(rename = "solr")]
        Solr,
    }
}
#[doc = "Matches single or multi-word synonyms in a token stream. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynonymTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A list of synonyms in following one of two formats: 1. incredible, unbelievable, fabulous => amazing - all terms on the left side of => symbol will be replaced with all terms on its right side; 2. incredible, unbelievable, fabulous, amazing - comma separated list of equivalent words. Set the expand option to change how this list is interpreted."]
    pub synonyms: Vec<String>,
    #[doc = "A value indicating whether to case-fold input for matching. Default is false."]
    #[serde(rename = "ignoreCase", default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[doc = "A value indicating whether all words in the list of synonyms (if => notation is not used) will map to one another. If true, all words in the list of synonyms (if => notation is not used) will map to one another. The following list: incredible, unbelievable, fabulous, amazing is equivalent to: incredible, unbelievable, fabulous, amazing => incredible, unbelievable, fabulous, amazing. If false, the following list: incredible, unbelievable, fabulous, amazing will be equivalent to: incredible, unbelievable, fabulous, amazing => incredible. Default is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
}
impl SynonymTokenFilter {
    pub fn new(token_filter: TokenFilter, synonyms: Vec<String>) -> Self {
        Self {
            token_filter,
            synonyms,
            ignore_case: None,
            expand: None,
        }
    }
}
#[doc = "Defines a function that boosts scores of documents with string values matching a given list of tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagScoringFunction {
    #[serde(flatten)]
    pub scoring_function: ScoringFunction,
    #[doc = "Provides parameter values to a tag scoring function."]
    pub tag: TagScoringParameters,
}
impl TagScoringFunction {
    pub fn new(scoring_function: ScoringFunction, tag: TagScoringParameters) -> Self {
        Self { scoring_function, tag }
    }
}
#[doc = "Provides parameter values to a tag scoring function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagScoringParameters {
    #[doc = "The name of the parameter passed in search queries to specify the list of tags to compare against the target field."]
    #[serde(rename = "tagsParameter")]
    pub tags_parameter: String,
}
impl TagScoringParameters {
    pub fn new(tags_parameter: String) -> Self {
        Self { tags_parameter }
    }
}
#[doc = "A value indicating which split mode to perform."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TextSplitMode")]
pub enum TextSplitMode {
    #[serde(rename = "pages")]
    Pages,
    #[serde(rename = "sentences")]
    Sentences,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TextSplitMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TextSplitMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TextSplitMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pages => serializer.serialize_unit_variant("TextSplitMode", 0u32, "pages"),
            Self::Sentences => serializer.serialize_unit_variant("TextSplitMode", 1u32, "sentences"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A skill to translate text from one language to another."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextTranslationSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The language codes supported for input text by TextTranslationSkill."]
    #[serde(rename = "defaultToLanguageCode")]
    pub default_to_language_code: TextTranslationSkillLanguage,
    #[doc = "The language codes supported for input text by TextTranslationSkill."]
    #[serde(rename = "defaultFromLanguageCode", default, skip_serializing_if = "Option::is_none")]
    pub default_from_language_code: Option<TextTranslationSkillLanguage>,
    #[doc = "The language codes supported for input text by TextTranslationSkill."]
    #[serde(rename = "suggestedFrom", default, skip_serializing_if = "Option::is_none")]
    pub suggested_from: Option<TextTranslationSkillLanguage>,
}
impl TextTranslationSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill, default_to_language_code: TextTranslationSkillLanguage) -> Self {
        Self {
            search_indexer_skill,
            default_to_language_code,
            default_from_language_code: None,
            suggested_from: None,
        }
    }
}
#[doc = "The language codes supported for input text by TextTranslationSkill."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TextTranslationSkillLanguage")]
pub enum TextTranslationSkillLanguage {
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "yue")]
    Yue,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fj")]
    Fj,
    #[serde(rename = "fil")]
    Fil,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "mww")]
    Mww,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "tlh")]
    Tlh,
    #[serde(rename = "tlh-Latn")]
    TlhLatn,
    #[serde(rename = "tlh-Piqd")]
    TlhPiqd,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "otq")]
    Otq,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "sr-Cyrl")]
    SrCyrl,
    #[serde(rename = "sr-Latn")]
    SrLatn,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "ty")]
    Ty,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "yua")]
    Yua,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "pa")]
    Pa,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TextTranslationSkillLanguage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TextTranslationSkillLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TextTranslationSkillLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Af => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 0u32, "af"),
            Self::Ar => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 1u32, "ar"),
            Self::Bn => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 2u32, "bn"),
            Self::Bs => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 3u32, "bs"),
            Self::Bg => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 4u32, "bg"),
            Self::Yue => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 5u32, "yue"),
            Self::Ca => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 6u32, "ca"),
            Self::ZhHans => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 7u32, "zh-Hans"),
            Self::ZhHant => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 8u32, "zh-Hant"),
            Self::Hr => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 9u32, "hr"),
            Self::Cs => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 10u32, "cs"),
            Self::Da => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 11u32, "da"),
            Self::Nl => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 12u32, "nl"),
            Self::En => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 13u32, "en"),
            Self::Et => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 14u32, "et"),
            Self::Fj => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 15u32, "fj"),
            Self::Fil => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 16u32, "fil"),
            Self::Fi => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 17u32, "fi"),
            Self::Fr => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 18u32, "fr"),
            Self::De => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 19u32, "de"),
            Self::El => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 20u32, "el"),
            Self::Ht => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 21u32, "ht"),
            Self::He => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 22u32, "he"),
            Self::Hi => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 23u32, "hi"),
            Self::Mww => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 24u32, "mww"),
            Self::Hu => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 25u32, "hu"),
            Self::Is => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 26u32, "is"),
            Self::Id => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 27u32, "id"),
            Self::It => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 28u32, "it"),
            Self::Ja => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 29u32, "ja"),
            Self::Sw => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 30u32, "sw"),
            Self::Tlh => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 31u32, "tlh"),
            Self::TlhLatn => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 32u32, "tlh-Latn"),
            Self::TlhPiqd => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 33u32, "tlh-Piqd"),
            Self::Ko => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 34u32, "ko"),
            Self::Lv => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 35u32, "lv"),
            Self::Lt => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 36u32, "lt"),
            Self::Mg => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 37u32, "mg"),
            Self::Ms => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 38u32, "ms"),
            Self::Mt => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 39u32, "mt"),
            Self::Nb => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 40u32, "nb"),
            Self::Fa => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 41u32, "fa"),
            Self::Pl => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 42u32, "pl"),
            Self::Pt => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 43u32, "pt"),
            Self::PtBr => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 44u32, "pt-br"),
            Self::PtPt => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 45u32, "pt-PT"),
            Self::Otq => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 46u32, "otq"),
            Self::Ro => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 47u32, "ro"),
            Self::Ru => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 48u32, "ru"),
            Self::Sm => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 49u32, "sm"),
            Self::SrCyrl => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 50u32, "sr-Cyrl"),
            Self::SrLatn => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 51u32, "sr-Latn"),
            Self::Sk => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 52u32, "sk"),
            Self::Sl => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 53u32, "sl"),
            Self::Es => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 54u32, "es"),
            Self::Sv => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 55u32, "sv"),
            Self::Ty => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 56u32, "ty"),
            Self::Ta => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 57u32, "ta"),
            Self::Te => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 58u32, "te"),
            Self::Th => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 59u32, "th"),
            Self::To => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 60u32, "to"),
            Self::Tr => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 61u32, "tr"),
            Self::Uk => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 62u32, "uk"),
            Self::Ur => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 63u32, "ur"),
            Self::Vi => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 64u32, "vi"),
            Self::Cy => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 65u32, "cy"),
            Self::Yua => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 66u32, "yua"),
            Self::Ga => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 67u32, "ga"),
            Self::Kn => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 68u32, "kn"),
            Self::Mi => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 69u32, "mi"),
            Self::Ml => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 70u32, "ml"),
            Self::Pa => serializer.serialize_unit_variant("TextTranslationSkillLanguage", 71u32, "pa"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines weights on index fields for which matches should boost scoring in search queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextWeights {
    #[doc = "The dictionary of per-field weights to boost document scoring. The keys are field names and the values are the weights for each field."]
    pub weights: serde_json::Value,
}
impl TextWeights {
    pub fn new(weights: serde_json::Value) -> Self {
        Self { weights }
    }
}
#[doc = "Represents classes of characters on which a token filter can operate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TokenCharacterKind {
    #[serde(rename = "letter")]
    Letter,
    #[serde(rename = "digit")]
    Digit,
    #[serde(rename = "whitespace")]
    Whitespace,
    #[serde(rename = "punctuation")]
    Punctuation,
    #[serde(rename = "symbol")]
    Symbol,
}
#[doc = "Base type for token filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenFilter {
    #[doc = "The name of the token filter. It must only contain letters, digits, spaces, dashes or underscores, can only start and end with alphanumeric characters, and is limited to 128 characters."]
    pub name: String,
}
impl TokenFilter {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "A URI fragment specifying the type of token filter."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum TokenFilterUnion {
    #[serde(rename = "#Microsoft.Azure.Search.AsciiFoldingTokenFilter")]
    MicrosoftAzureSearchAsciiFoldingTokenFilter(AsciiFoldingTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.CjkBigramTokenFilter")]
    MicrosoftAzureSearchCjkBigramTokenFilter(CjkBigramTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.CommonGramTokenFilter")]
    MicrosoftAzureSearchCommonGramTokenFilter(CommonGramTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.DictionaryDecompounderTokenFilter")]
    MicrosoftAzureSearchDictionaryDecompounderTokenFilter(DictionaryDecompounderTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.EdgeNGramTokenFilter")]
    MicrosoftAzureSearchEdgeNGramTokenFilter(EdgeNGramTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.EdgeNGramTokenFilterV2")]
    MicrosoftAzureSearchEdgeNGramTokenFilterV2(EdgeNGramTokenFilterV2),
    #[serde(rename = "#Microsoft.Azure.Search.ElisionTokenFilter")]
    MicrosoftAzureSearchElisionTokenFilter(ElisionTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.KeepTokenFilter")]
    MicrosoftAzureSearchKeepTokenFilter(KeepTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.KeywordMarkerTokenFilter")]
    MicrosoftAzureSearchKeywordMarkerTokenFilter(KeywordMarkerTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.LengthTokenFilter")]
    MicrosoftAzureSearchLengthTokenFilter(LengthTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.LimitTokenFilter")]
    MicrosoftAzureSearchLimitTokenFilter(LimitTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.NGramTokenFilter")]
    MicrosoftAzureSearchNGramTokenFilter(NGramTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.NGramTokenFilterV2")]
    MicrosoftAzureSearchNGramTokenFilterV2(NGramTokenFilterV2),
    #[serde(rename = "#Microsoft.Azure.Search.PatternCaptureTokenFilter")]
    MicrosoftAzureSearchPatternCaptureTokenFilter(PatternCaptureTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.PatternReplaceTokenFilter")]
    MicrosoftAzureSearchPatternReplaceTokenFilter(PatternReplaceTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.PhoneticTokenFilter")]
    MicrosoftAzureSearchPhoneticTokenFilter(PhoneticTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.ShingleTokenFilter")]
    MicrosoftAzureSearchShingleTokenFilter(ShingleTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.SnowballTokenFilter")]
    MicrosoftAzureSearchSnowballTokenFilter(SnowballTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.StemmerOverrideTokenFilter")]
    MicrosoftAzureSearchStemmerOverrideTokenFilter(StemmerOverrideTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.StemmerTokenFilter")]
    MicrosoftAzureSearchStemmerTokenFilter(StemmerTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.StopwordsTokenFilter")]
    MicrosoftAzureSearchStopwordsTokenFilter(StopwordsTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.SynonymTokenFilter")]
    MicrosoftAzureSearchSynonymTokenFilter(SynonymTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.TruncateTokenFilter")]
    MicrosoftAzureSearchTruncateTokenFilter(TruncateTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.UniqueTokenFilter")]
    MicrosoftAzureSearchUniqueTokenFilter(UniqueTokenFilter),
    #[serde(rename = "#Microsoft.Azure.Search.WordDelimiterTokenFilter")]
    MicrosoftAzureSearchWordDelimiterTokenFilter(WordDelimiterTokenFilter),
}
#[doc = "Defines the names of all token filters supported by the search engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TokenFilterName")]
pub enum TokenFilterName {
    #[serde(rename = "arabic_normalization")]
    ArabicNormalization,
    #[serde(rename = "apostrophe")]
    Apostrophe,
    #[serde(rename = "asciifolding")]
    Asciifolding,
    #[serde(rename = "cjk_bigram")]
    CjkBigram,
    #[serde(rename = "cjk_width")]
    CjkWidth,
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "common_grams")]
    CommonGrams,
    #[serde(rename = "edgeNGram_v2")]
    EdgeNGramV2,
    #[serde(rename = "elision")]
    Elision,
    #[serde(rename = "german_normalization")]
    GermanNormalization,
    #[serde(rename = "hindi_normalization")]
    HindiNormalization,
    #[serde(rename = "indic_normalization")]
    IndicNormalization,
    #[serde(rename = "keyword_repeat")]
    KeywordRepeat,
    #[serde(rename = "kstem")]
    Kstem,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "nGram_v2")]
    NGramV2,
    #[serde(rename = "persian_normalization")]
    PersianNormalization,
    #[serde(rename = "phonetic")]
    Phonetic,
    #[serde(rename = "porter_stem")]
    PorterStem,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "scandinavian_normalization")]
    ScandinavianNormalization,
    #[serde(rename = "scandinavian_folding")]
    ScandinavianFolding,
    #[serde(rename = "shingle")]
    Shingle,
    #[serde(rename = "snowball")]
    Snowball,
    #[serde(rename = "sorani_normalization")]
    SoraniNormalization,
    #[serde(rename = "stemmer")]
    Stemmer,
    #[serde(rename = "stopwords")]
    Stopwords,
    #[serde(rename = "trim")]
    Trim,
    #[serde(rename = "truncate")]
    Truncate,
    #[serde(rename = "unique")]
    Unique,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "word_delimiter")]
    WordDelimiter,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TokenFilterName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TokenFilterName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TokenFilterName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ArabicNormalization => serializer.serialize_unit_variant("TokenFilterName", 0u32, "arabic_normalization"),
            Self::Apostrophe => serializer.serialize_unit_variant("TokenFilterName", 1u32, "apostrophe"),
            Self::Asciifolding => serializer.serialize_unit_variant("TokenFilterName", 2u32, "asciifolding"),
            Self::CjkBigram => serializer.serialize_unit_variant("TokenFilterName", 3u32, "cjk_bigram"),
            Self::CjkWidth => serializer.serialize_unit_variant("TokenFilterName", 4u32, "cjk_width"),
            Self::Classic => serializer.serialize_unit_variant("TokenFilterName", 5u32, "classic"),
            Self::CommonGrams => serializer.serialize_unit_variant("TokenFilterName", 6u32, "common_grams"),
            Self::EdgeNGramV2 => serializer.serialize_unit_variant("TokenFilterName", 7u32, "edgeNGram_v2"),
            Self::Elision => serializer.serialize_unit_variant("TokenFilterName", 8u32, "elision"),
            Self::GermanNormalization => serializer.serialize_unit_variant("TokenFilterName", 9u32, "german_normalization"),
            Self::HindiNormalization => serializer.serialize_unit_variant("TokenFilterName", 10u32, "hindi_normalization"),
            Self::IndicNormalization => serializer.serialize_unit_variant("TokenFilterName", 11u32, "indic_normalization"),
            Self::KeywordRepeat => serializer.serialize_unit_variant("TokenFilterName", 12u32, "keyword_repeat"),
            Self::Kstem => serializer.serialize_unit_variant("TokenFilterName", 13u32, "kstem"),
            Self::Length => serializer.serialize_unit_variant("TokenFilterName", 14u32, "length"),
            Self::Limit => serializer.serialize_unit_variant("TokenFilterName", 15u32, "limit"),
            Self::Lowercase => serializer.serialize_unit_variant("TokenFilterName", 16u32, "lowercase"),
            Self::NGramV2 => serializer.serialize_unit_variant("TokenFilterName", 17u32, "nGram_v2"),
            Self::PersianNormalization => serializer.serialize_unit_variant("TokenFilterName", 18u32, "persian_normalization"),
            Self::Phonetic => serializer.serialize_unit_variant("TokenFilterName", 19u32, "phonetic"),
            Self::PorterStem => serializer.serialize_unit_variant("TokenFilterName", 20u32, "porter_stem"),
            Self::Reverse => serializer.serialize_unit_variant("TokenFilterName", 21u32, "reverse"),
            Self::ScandinavianNormalization => serializer.serialize_unit_variant("TokenFilterName", 22u32, "scandinavian_normalization"),
            Self::ScandinavianFolding => serializer.serialize_unit_variant("TokenFilterName", 23u32, "scandinavian_folding"),
            Self::Shingle => serializer.serialize_unit_variant("TokenFilterName", 24u32, "shingle"),
            Self::Snowball => serializer.serialize_unit_variant("TokenFilterName", 25u32, "snowball"),
            Self::SoraniNormalization => serializer.serialize_unit_variant("TokenFilterName", 26u32, "sorani_normalization"),
            Self::Stemmer => serializer.serialize_unit_variant("TokenFilterName", 27u32, "stemmer"),
            Self::Stopwords => serializer.serialize_unit_variant("TokenFilterName", 28u32, "stopwords"),
            Self::Trim => serializer.serialize_unit_variant("TokenFilterName", 29u32, "trim"),
            Self::Truncate => serializer.serialize_unit_variant("TokenFilterName", 30u32, "truncate"),
            Self::Unique => serializer.serialize_unit_variant("TokenFilterName", 31u32, "unique"),
            Self::Uppercase => serializer.serialize_unit_variant("TokenFilterName", 32u32, "uppercase"),
            Self::WordDelimiter => serializer.serialize_unit_variant("TokenFilterName", 33u32, "word_delimiter"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Truncates the terms to a specific length. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TruncateTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "The length at which terms will be truncated. Default and maximum is 300."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
}
impl TruncateTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            length: None,
        }
    }
}
#[doc = "Tokenizes urls and emails as one token. This tokenizer is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UaxUrlEmailTokenizer {
    #[serde(flatten)]
    pub lexical_tokenizer: LexicalTokenizer,
    #[doc = "The maximum token length. Default is 255. Tokens longer than the maximum length are split. The maximum token length that can be used is 300 characters."]
    #[serde(rename = "maxTokenLength", default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
}
impl UaxUrlEmailTokenizer {
    pub fn new(lexical_tokenizer: LexicalTokenizer) -> Self {
        Self {
            lexical_tokenizer,
            max_token_length: None,
        }
    }
}
#[doc = "Filters out tokens with same text as the previous token. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniqueTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A value indicating whether to remove duplicates only at the same position. Default is false."]
    #[serde(rename = "onlyOnSamePosition", default, skip_serializing_if = "Option::is_none")]
    pub only_on_same_position: Option<bool>,
}
impl UniqueTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            only_on_same_position: None,
        }
    }
}
#[doc = "Contains configuration options related to vector search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VectorSearch {
    #[doc = "Defines combinations of configurations to use with vector search."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub profiles: Vec<VectorSearchProfile>,
    #[doc = "Contains configuration options specific to the algorithm used during indexing or querying."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub algorithms: Vec<VectorSearchAlgorithmConfigurationUnion>,
    #[doc = "Contains configuration options on how to vectorize text vector queries."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vectorizers: Vec<VectorSearchVectorizerUnion>,
}
impl VectorSearch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains configuration options specific to the algorithm used during indexing or querying."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorSearchAlgorithmConfiguration {
    #[doc = "The name to associate with this particular configuration."]
    pub name: String,
}
impl VectorSearchAlgorithmConfiguration {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The algorithm used for indexing and querying."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum VectorSearchAlgorithmConfigurationUnion {
    #[serde(rename = "exhaustiveKnn")]
    ExhaustiveKnn(ExhaustiveKnnVectorSearchAlgorithmConfiguration),
    #[serde(rename = "hnsw")]
    Hnsw(HnswVectorSearchAlgorithmConfiguration),
}
#[doc = "The algorithm used for indexing and querying."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VectorSearchAlgorithmKind")]
pub enum VectorSearchAlgorithmKind {
    #[serde(rename = "hnsw")]
    Hnsw,
    #[serde(rename = "exhaustiveKnn")]
    ExhaustiveKnn,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VectorSearchAlgorithmKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VectorSearchAlgorithmKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VectorSearchAlgorithmKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Hnsw => serializer.serialize_unit_variant("VectorSearchAlgorithmKind", 0u32, "hnsw"),
            Self::ExhaustiveKnn => serializer.serialize_unit_variant("VectorSearchAlgorithmKind", 1u32, "exhaustiveKnn"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The similarity metric to use for vector comparisons."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VectorSearchAlgorithmMetric")]
pub enum VectorSearchAlgorithmMetric {
    #[serde(rename = "cosine")]
    Cosine,
    #[serde(rename = "euclidean")]
    Euclidean,
    #[serde(rename = "dotProduct")]
    DotProduct,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VectorSearchAlgorithmMetric {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VectorSearchAlgorithmMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VectorSearchAlgorithmMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cosine => serializer.serialize_unit_variant("VectorSearchAlgorithmMetric", 0u32, "cosine"),
            Self::Euclidean => serializer.serialize_unit_variant("VectorSearchAlgorithmMetric", 1u32, "euclidean"),
            Self::DotProduct => serializer.serialize_unit_variant("VectorSearchAlgorithmMetric", 2u32, "dotProduct"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Defines a combination of configurations to use with vector search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorSearchProfile {
    #[doc = "The name to associate with this particular vector search profile."]
    pub name: String,
    #[doc = "The name of the vector search algorithm configuration that specifies the algorithm and optional parameters."]
    pub algorithm: String,
    #[doc = "The name of the kind of vectorization method being configured for use with vector search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vectorizer: Option<String>,
}
impl VectorSearchProfile {
    pub fn new(name: String, algorithm: String) -> Self {
        Self {
            name,
            algorithm,
            vectorizer: None,
        }
    }
}
#[doc = "Specifies the vectorization method to be used during query time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorSearchVectorizer {
    #[doc = "The name to associate with this particular vectorization method."]
    pub name: String,
}
impl VectorSearchVectorizer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The vectorization method to be used during query time."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum VectorSearchVectorizerUnion {
    #[serde(rename = "azureOpenAI")]
    AzureOpenAi(AzureOpenAiVectorizer),
    #[serde(rename = "customWebApi")]
    CustomWebApi(CustomVectorizer),
}
#[doc = "The vectorization method to be used during query time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VectorSearchVectorizerKind")]
pub enum VectorSearchVectorizerKind {
    #[serde(rename = "azureOpenAI")]
    AzureOpenAi,
    #[serde(rename = "customWebApi")]
    CustomWebApi,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VectorSearchVectorizerKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VectorSearchVectorizerKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VectorSearchVectorizerKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureOpenAi => serializer.serialize_unit_variant("VectorSearchVectorizerKind", 0u32, "azureOpenAI"),
            Self::CustomWebApi => serializer.serialize_unit_variant("VectorSearchVectorizerKind", 1u32, "customWebApi"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The strings indicating what visual feature types to return."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VisualFeature")]
pub enum VisualFeature {
    #[serde(rename = "adult")]
    Adult,
    #[serde(rename = "brands")]
    Brands,
    #[serde(rename = "categories")]
    Categories,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "faces")]
    Faces,
    #[serde(rename = "objects")]
    Objects,
    #[serde(rename = "tags")]
    Tags,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VisualFeature {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VisualFeature {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VisualFeature {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Adult => serializer.serialize_unit_variant("VisualFeature", 0u32, "adult"),
            Self::Brands => serializer.serialize_unit_variant("VisualFeature", 1u32, "brands"),
            Self::Categories => serializer.serialize_unit_variant("VisualFeature", 2u32, "categories"),
            Self::Description => serializer.serialize_unit_variant("VisualFeature", 3u32, "description"),
            Self::Faces => serializer.serialize_unit_variant("VisualFeature", 4u32, "faces"),
            Self::Objects => serializer.serialize_unit_variant("VisualFeature", 5u32, "objects"),
            Self::Tags => serializer.serialize_unit_variant("VisualFeature", 6u32, "tags"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A dictionary of http request headers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiHttpHeaders {}
impl WebApiHttpHeaders {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A skill that can call a Web API endpoint, allowing you to extend a skillset by having it call your custom code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApiSkill {
    #[serde(flatten)]
    pub search_indexer_skill: SearchIndexerSkill,
    #[doc = "The url for the Web API."]
    pub uri: String,
    #[doc = "A dictionary of http request headers."]
    #[serde(rename = "httpHeaders", default, skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<WebApiHttpHeaders>,
    #[doc = "The method for the http request."]
    #[serde(rename = "httpMethod", default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[doc = "The desired timeout for the request. Default is 30 seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "The desired batch size which indicates number of documents."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[doc = "If set, the number of parallel calls that can be made to the Web API."]
    #[serde(rename = "degreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism: Option<i32>,
    #[doc = "Applies to custom skills that connect to external code in an Azure function or some other application that provides the transformations. This value should be the application ID created for the function or app when it was registered with Azure Active Directory. When specified, the custom skill connects to the function or app using a managed ID (either system or user-assigned) of the search service and the access token of the function or app, using this value as the resource id for creating the scope of the access token."]
    #[serde(rename = "authResourceId", default, skip_serializing_if = "Option::is_none")]
    pub auth_resource_id: Option<String>,
    #[doc = "Abstract base type for data identities."]
    #[serde(rename = "authIdentity", default, skip_serializing_if = "Option::is_none")]
    pub auth_identity: Option<SearchIndexerDataIdentityUnion>,
}
impl WebApiSkill {
    pub fn new(search_indexer_skill: SearchIndexerSkill, uri: String) -> Self {
        Self {
            search_indexer_skill,
            uri,
            http_headers: None,
            http_method: None,
            timeout: None,
            batch_size: None,
            degree_of_parallelism: None,
            auth_resource_id: None,
            auth_identity: None,
        }
    }
}
#[doc = "Splits words into subwords and performs optional transformations on subword groups. This token filter is implemented using Apache Lucene."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WordDelimiterTokenFilter {
    #[serde(flatten)]
    pub token_filter: TokenFilter,
    #[doc = "A value indicating whether to generate part words. If set, causes parts of words to be generated; for example \"AzureSearch\" becomes \"Azure\" \"Search\". Default is true."]
    #[serde(rename = "generateWordParts", default, skip_serializing_if = "Option::is_none")]
    pub generate_word_parts: Option<bool>,
    #[doc = "A value indicating whether to generate number subwords. Default is true."]
    #[serde(rename = "generateNumberParts", default, skip_serializing_if = "Option::is_none")]
    pub generate_number_parts: Option<bool>,
    #[doc = "A value indicating whether maximum runs of word parts will be catenated. For example, if this is set to true, \"Azure-Search\" becomes \"AzureSearch\". Default is false."]
    #[serde(rename = "catenateWords", default, skip_serializing_if = "Option::is_none")]
    pub catenate_words: Option<bool>,
    #[doc = "A value indicating whether maximum runs of number parts will be catenated. For example, if this is set to true, \"1-2\" becomes \"12\". Default is false."]
    #[serde(rename = "catenateNumbers", default, skip_serializing_if = "Option::is_none")]
    pub catenate_numbers: Option<bool>,
    #[doc = "A value indicating whether all subword parts will be catenated. For example, if this is set to true, \"Azure-Search-1\" becomes \"AzureSearch1\". Default is false."]
    #[serde(rename = "catenateAll", default, skip_serializing_if = "Option::is_none")]
    pub catenate_all: Option<bool>,
    #[doc = "A value indicating whether to split words on caseChange. For example, if this is set to true, \"AzureSearch\" becomes \"Azure\" \"Search\". Default is true."]
    #[serde(rename = "splitOnCaseChange", default, skip_serializing_if = "Option::is_none")]
    pub split_on_case_change: Option<bool>,
    #[doc = "A value indicating whether original words will be preserved and added to the subword list. Default is false."]
    #[serde(rename = "preserveOriginal", default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<bool>,
    #[doc = "A value indicating whether to split on numbers. For example, if this is set to true, \"Azure1Search\" becomes \"Azure\" \"1\" \"Search\". Default is true."]
    #[serde(rename = "splitOnNumerics", default, skip_serializing_if = "Option::is_none")]
    pub split_on_numerics: Option<bool>,
    #[doc = "A value indicating whether to remove trailing \"'s\" for each subword. Default is true."]
    #[serde(rename = "stemEnglishPossessive", default, skip_serializing_if = "Option::is_none")]
    pub stem_english_possessive: Option<bool>,
    #[doc = "A list of tokens to protect from being delimited."]
    #[serde(
        rename = "protectedWords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_words: Vec<String>,
}
impl WordDelimiterTokenFilter {
    pub fn new(token_filter: TokenFilter) -> Self {
        Self {
            token_filter,
            generate_word_parts: None,
            generate_number_parts: None,
            catenate_words: None,
            catenate_numbers: None,
            catenate_all: None,
            split_on_case_change: None,
            preserve_original: None,
            split_on_numerics: None,
            stem_english_possessive: None,
            protected_words: Vec::new(),
        }
    }
}
