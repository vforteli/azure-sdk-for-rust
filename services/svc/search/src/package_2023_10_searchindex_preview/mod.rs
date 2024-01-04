#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: azure_core::Url,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<azure_core::Url>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub use azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD as DEFAULT_ENDPOINT;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<azure_core::Url>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    pub fn build(self) -> azure_core::Result<Client> {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = if let Some(scopes) = self.scopes {
            scopes
        } else {
            vec![endpoint.join(azure_core::auth::DEFAULT_SCOPE_SUFFIX)?.to_string()]
        };
        Ok(Client::new(endpoint, self.credential, scopes, self.options))
    }
}
impl Client {
    pub(crate) async fn bearer_token(&self) -> azure_core::Result<azure_core::auth::Secret> {
        let credential = self.token_credential();
        let response = credential.get_token(&self.scopes()).await?;
        Ok(response.token)
    }
    pub(crate) fn endpoint(&self) -> &azure_core::Url {
        &self.endpoint
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let context = azure_core::Context::default();
        self.pipeline.send(&context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<azure_core::Url>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn documents_client(&self) -> documents::Client {
        documents::Client(self.clone())
    }
}
pub mod documents {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Queries the number of documents in the index."]
        pub fn count(&self) -> count::RequestBuilder {
            count::RequestBuilder {
                client: self.0.clone(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Searches for documents in the index."]
        pub fn search_get(&self) -> search_get::RequestBuilder {
            search_get::RequestBuilder {
                client: self.0.clone(),
                search: None,
                count: None,
                facet: Vec::new(),
                filter: None,
                highlight: Vec::new(),
                highlight_post_tag: None,
                highlight_pre_tag: None,
                minimum_coverage: None,
                orderby: Vec::new(),
                query_type: None,
                scoring_parameter: Vec::new(),
                scoring_profile: None,
                semantic_query: None,
                semantic_configuration: None,
                semantic_error_handling: None,
                semantic_max_wait_in_milliseconds: None,
                debug: None,
                search_fields: Vec::new(),
                query_language: None,
                speller: None,
                answers: None,
                search_mode: None,
                scoring_statistics: None,
                session_id: None,
                select: Vec::new(),
                skip: None,
                top: None,
                captions: None,
                semantic_fields: Vec::new(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Searches for documents in the index."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `search_request`: The definition of the Search request."]
        pub fn search_post(&self, search_request: impl Into<models::SearchRequest>) -> search_post::RequestBuilder {
            search_post::RequestBuilder {
                client: self.0.clone(),
                search_request: search_request.into(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Retrieves a document from the index."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `key`: The key of the document to retrieve."]
        pub fn get(&self, key: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                key: key.into(),
                select: Vec::new(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Suggests documents in the index that match the given partial query text."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `search`: The search text to use to suggest documents. Must be at least 1 character, and no more than 100 characters."]
        #[doc = "* `suggester_name`: The name of the suggester as specified in the suggesters collection that's part of the index definition."]
        pub fn suggest_get(&self, search: impl Into<String>, suggester_name: impl Into<String>) -> suggest_get::RequestBuilder {
            suggest_get::RequestBuilder {
                client: self.0.clone(),
                search: search.into(),
                suggester_name: suggester_name.into(),
                filter: None,
                fuzzy: None,
                highlight_post_tag: None,
                highlight_pre_tag: None,
                minimum_coverage: None,
                orderby: Vec::new(),
                search_fields: Vec::new(),
                select: Vec::new(),
                top: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Suggests documents in the index that match the given partial query text."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `suggest_request`: The Suggest request."]
        pub fn suggest_post(&self, suggest_request: impl Into<models::SuggestRequest>) -> suggest_post::RequestBuilder {
            suggest_post::RequestBuilder {
                client: self.0.clone(),
                suggest_request: suggest_request.into(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Sends a batch of document write actions to the index."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `batch`: The batch of index actions."]
        pub fn index(&self, batch: impl Into<models::IndexBatch>) -> index::RequestBuilder {
            index::RequestBuilder {
                client: self.0.clone(),
                batch: batch.into(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Autocompletes incomplete query terms based on input text and matching terms in the index."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `search`: The incomplete term which should be auto-completed."]
        #[doc = "* `suggester_name`: The name of the suggester as specified in the suggesters collection that's part of the index definition."]
        pub fn autocomplete_get(&self, search: impl Into<String>, suggester_name: impl Into<String>) -> autocomplete_get::RequestBuilder {
            autocomplete_get::RequestBuilder {
                client: self.0.clone(),
                search: search.into(),
                suggester_name: suggester_name.into(),
                x_ms_client_request_id: None,
                autocomplete_mode: None,
                filter: None,
                fuzzy: None,
                highlight_post_tag: None,
                highlight_pre_tag: None,
                minimum_coverage: None,
                search_fields: Vec::new(),
                top: None,
            }
        }
        #[doc = "Autocompletes incomplete query terms based on input text and matching terms in the index."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `autocomplete_request`: The definition of the Autocomplete request."]
        pub fn autocomplete_post(&self, autocomplete_request: impl Into<models::AutocompleteRequest>) -> autocomplete_post::RequestBuilder {
            autocomplete_post::RequestBuilder {
                client: self.0.clone(),
                autocomplete_request: autocomplete_request.into(),
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod count {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<i64> {
                let bytes = self.0.into_body().collect().await?;
                let body: i64 = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/$count", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<i64>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<i64>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod search_get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SearchDocumentsResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SearchDocumentsResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) search: Option<String>,
            pub(crate) count: Option<bool>,
            pub(crate) facet: Vec<String>,
            pub(crate) filter: Option<String>,
            pub(crate) highlight: Vec<String>,
            pub(crate) highlight_post_tag: Option<String>,
            pub(crate) highlight_pre_tag: Option<String>,
            pub(crate) minimum_coverage: Option<f64>,
            pub(crate) orderby: Vec<String>,
            pub(crate) query_type: Option<String>,
            pub(crate) scoring_parameter: Vec<String>,
            pub(crate) scoring_profile: Option<String>,
            pub(crate) semantic_query: Option<String>,
            pub(crate) semantic_configuration: Option<String>,
            pub(crate) semantic_error_handling: Option<String>,
            pub(crate) semantic_max_wait_in_milliseconds: Option<i32>,
            pub(crate) debug: Option<String>,
            pub(crate) search_fields: Vec<String>,
            pub(crate) query_language: Option<String>,
            pub(crate) speller: Option<String>,
            pub(crate) answers: Option<String>,
            pub(crate) search_mode: Option<String>,
            pub(crate) scoring_statistics: Option<String>,
            pub(crate) session_id: Option<String>,
            pub(crate) select: Vec<String>,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
            pub(crate) captions: Option<String>,
            pub(crate) semantic_fields: Vec<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A full-text search query expression; Use \"*\" or omit this parameter to match all documents."]
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            #[doc = "A value that specifies whether to fetch the total count of results. Default is false. Setting this value to true may have a performance impact. Note that the count returned is an approximation."]
            pub fn count(mut self, count: bool) -> Self {
                self.count = Some(count);
                self
            }
            #[doc = "The list of facet expressions to apply to the search query. Each facet expression contains a field name, optionally followed by a comma-separated list of name:value pairs."]
            pub fn facet(mut self, facet: Vec<String>) -> Self {
                self.facet = facet;
                self
            }
            #[doc = "The OData $filter expression to apply to the search query."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The list of field names to use for hit highlights. Only searchable fields can be used for hit highlighting."]
            pub fn highlight(mut self, highlight: Vec<String>) -> Self {
                self.highlight = highlight;
                self
            }
            #[doc = "A string tag that is appended to hit highlights. Must be set with highlightPreTag. Default is &lt;/em&gt;."]
            pub fn highlight_post_tag(mut self, highlight_post_tag: impl Into<String>) -> Self {
                self.highlight_post_tag = Some(highlight_post_tag.into());
                self
            }
            #[doc = "A string tag that is prepended to hit highlights. Must be set with highlightPostTag. Default is &lt;em&gt;."]
            pub fn highlight_pre_tag(mut self, highlight_pre_tag: impl Into<String>) -> Self {
                self.highlight_pre_tag = Some(highlight_pre_tag.into());
                self
            }
            #[doc = "A number between 0 and 100 indicating the percentage of the index that must be covered by a search query in order for the query to be reported as a success. This parameter can be useful for ensuring search availability even for services with only one replica. The default is 100."]
            pub fn minimum_coverage(mut self, minimum_coverage: f64) -> Self {
                self.minimum_coverage = Some(minimum_coverage);
                self
            }
            #[doc = "The list of OData $orderby expressions by which to sort the results. Each expression can be either a field name or a call to either the geo.distance() or the search.score() functions. Each expression can be followed by asc to indicate ascending, and desc to indicate descending. The default is ascending order. Ties will be broken by the match scores of documents. If no OrderBy is specified, the default sort order is descending by document match score. There can be at most 32 $orderby clauses."]
            pub fn orderby(mut self, orderby: Vec<String>) -> Self {
                self.orderby = orderby;
                self
            }
            #[doc = "A value that specifies the syntax of the search query. The default is 'simple'. Use 'full' if your query uses the Lucene query syntax."]
            pub fn query_type(mut self, query_type: impl Into<String>) -> Self {
                self.query_type = Some(query_type.into());
                self
            }
            #[doc = "The list of parameter values to be used in scoring functions (for example, referencePointParameter) using the format name-values. For example, if the scoring profile defines a function with a parameter called 'mylocation' the parameter string would be \"mylocation--122.2,44.8\" (without the quotes)."]
            pub fn scoring_parameter(mut self, scoring_parameter: Vec<String>) -> Self {
                self.scoring_parameter = scoring_parameter;
                self
            }
            #[doc = "The name of a scoring profile to evaluate match scores for matching documents in order to sort the results."]
            pub fn scoring_profile(mut self, scoring_profile: impl Into<String>) -> Self {
                self.scoring_profile = Some(scoring_profile.into());
                self
            }
            #[doc = "Allows setting a separate search query that will be solely used for semantic reranking, semantic captions and semantic answers. Is useful for scenarios where there is a need to use different queries between the base retrieval and ranking phase, and the L2 semantic phase."]
            pub fn semantic_query(mut self, semantic_query: impl Into<String>) -> Self {
                self.semantic_query = Some(semantic_query.into());
                self
            }
            #[doc = "The name of the semantic configuration that lists which fields should be used for semantic ranking, captions, highlights, and answers"]
            pub fn semantic_configuration(mut self, semantic_configuration: impl Into<String>) -> Self {
                self.semantic_configuration = Some(semantic_configuration.into());
                self
            }
            #[doc = "Allows the user to choose whether a semantic call should fail completely, or to return partial results (default)."]
            pub fn semantic_error_handling(mut self, semantic_error_handling: impl Into<String>) -> Self {
                self.semantic_error_handling = Some(semantic_error_handling.into());
                self
            }
            #[doc = "Allows the user to set an upper bound on the amount of time it takes for semantic enrichment to finish processing before the request fails."]
            pub fn semantic_max_wait_in_milliseconds(mut self, semantic_max_wait_in_milliseconds: i32) -> Self {
                self.semantic_max_wait_in_milliseconds = Some(semantic_max_wait_in_milliseconds);
                self
            }
            #[doc = "Enables a debugging tool that can be used to further explore your search results."]
            pub fn debug(mut self, debug: impl Into<String>) -> Self {
                self.debug = Some(debug.into());
                self
            }
            #[doc = "The list of field names to which to scope the full-text search. When using fielded search (fieldName:searchExpression) in a full Lucene query, the field names of each fielded search expression take precedence over any field names listed in this parameter."]
            pub fn search_fields(mut self, search_fields: Vec<String>) -> Self {
                self.search_fields = search_fields;
                self
            }
            #[doc = "The language of the query."]
            pub fn query_language(mut self, query_language: impl Into<String>) -> Self {
                self.query_language = Some(query_language.into());
                self
            }
            #[doc = "Improve search recall by spell-correcting individual search query terms."]
            pub fn speller(mut self, speller: impl Into<String>) -> Self {
                self.speller = Some(speller.into());
                self
            }
            #[doc = "This parameter is only valid if the query type is `semantic`. If set, the query returns answers extracted from key passages in the highest ranked documents. The number of answers returned can be configured by appending the pipe character `|` followed by the `count-<number of answers>` option after the answers parameter value, such as `extractive|count-3`. Default count is 1. The confidence threshold can be configured by appending the pipe character `|` followed by the `threshold-<confidence threshold>` option after the answers parameter value, such as `extractive|threshold-0.9`. Default threshold is 0.7."]
            pub fn answers(mut self, answers: impl Into<String>) -> Self {
                self.answers = Some(answers.into());
                self
            }
            #[doc = "A value that specifies whether any or all of the search terms must be matched in order to count the document as a match."]
            pub fn search_mode(mut self, search_mode: impl Into<String>) -> Self {
                self.search_mode = Some(search_mode.into());
                self
            }
            #[doc = "A value that specifies whether we want to calculate scoring statistics (such as document frequency) globally for more consistent scoring, or locally, for lower latency."]
            pub fn scoring_statistics(mut self, scoring_statistics: impl Into<String>) -> Self {
                self.scoring_statistics = Some(scoring_statistics.into());
                self
            }
            #[doc = "A value to be used to create a sticky session, which can help to get more consistent results. As long as the same sessionId is used, a best-effort attempt will be made to target the same replica set. Be wary that reusing the same sessionID values repeatedly can interfere with the load balancing of the requests across replicas and adversely affect the performance of the search service. The value used as sessionId cannot start with a '_' character."]
            pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
                self.session_id = Some(session_id.into());
                self
            }
            #[doc = "The list of fields to retrieve. If unspecified, all fields marked as retrievable in the schema are included."]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "The number of search results to skip. This value cannot be greater than 100,000. If you need to scan documents in sequence, but cannot use $skip due to this limitation, consider using $orderby on a totally-ordered key and $filter with a range query instead."]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "The number of search results to retrieve. This can be used in conjunction with $skip to implement client-side paging of search results. If results are truncated due to server-side paging, the response will include a continuation token that can be used to issue another Search request for the next page of results."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "This parameter is only valid if the query type is `semantic`. If set, the query returns captions extracted from key passages in the highest ranked documents. When Captions is set to `extractive`, highlighting is enabled by default, and can be configured by appending the pipe character `|` followed by the `highlight-<true/false>` option, such as `extractive|highlight-true`. Defaults to `None`."]
            pub fn captions(mut self, captions: impl Into<String>) -> Self {
                self.captions = Some(captions.into());
                self
            }
            #[doc = "The list of field names used for semantic ranking."]
            pub fn semantic_fields(mut self, semantic_fields: Vec<String>) -> Self {
                self.semantic_fields = semantic_fields;
                self
            }
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        if let Some(search) = &this.search {
                            req.url_mut().query_pairs_mut().append_pair("search", search);
                        }
                        if let Some(count) = &this.count {
                            req.url_mut().query_pairs_mut().append_pair("$count", &count.to_string());
                        }
                        let facet = &this.facet;
                        for value in &this.facet {
                            req.url_mut().query_pairs_mut().append_pair("facet", &value.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(highlight_post_tag) = &this.highlight_post_tag {
                            req.url_mut().query_pairs_mut().append_pair("highlightPostTag", highlight_post_tag);
                        }
                        if let Some(highlight_pre_tag) = &this.highlight_pre_tag {
                            req.url_mut().query_pairs_mut().append_pair("highlightPreTag", highlight_pre_tag);
                        }
                        if let Some(minimum_coverage) = &this.minimum_coverage {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("minimumCoverage", &minimum_coverage.to_string());
                        }
                        if let Some(query_type) = &this.query_type {
                            req.url_mut().query_pairs_mut().append_pair("queryType", query_type);
                        }
                        let scoring_parameter = &this.scoring_parameter;
                        for value in &this.scoring_parameter {
                            req.url_mut().query_pairs_mut().append_pair("scoringParameter", &value.to_string());
                        }
                        if let Some(scoring_profile) = &this.scoring_profile {
                            req.url_mut().query_pairs_mut().append_pair("scoringProfile", scoring_profile);
                        }
                        if let Some(semantic_query) = &this.semantic_query {
                            req.url_mut().query_pairs_mut().append_pair("semanticQuery", semantic_query);
                        }
                        if let Some(semantic_configuration) = &this.semantic_configuration {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("semanticConfiguration", semantic_configuration);
                        }
                        if let Some(semantic_error_handling) = &this.semantic_error_handling {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("semanticErrorHandling", semantic_error_handling);
                        }
                        if let Some(semantic_max_wait_in_milliseconds) = &this.semantic_max_wait_in_milliseconds {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("semanticMaxWaitInMilliseconds", &semantic_max_wait_in_milliseconds.to_string());
                        }
                        if let Some(debug) = &this.debug {
                            req.url_mut().query_pairs_mut().append_pair("debug", debug);
                        }
                        if let Some(query_language) = &this.query_language {
                            req.url_mut().query_pairs_mut().append_pair("queryLanguage", query_language);
                        }
                        if let Some(speller) = &this.speller {
                            req.url_mut().query_pairs_mut().append_pair("speller", speller);
                        }
                        if let Some(answers) = &this.answers {
                            req.url_mut().query_pairs_mut().append_pair("answers", answers);
                        }
                        if let Some(search_mode) = &this.search_mode {
                            req.url_mut().query_pairs_mut().append_pair("searchMode", search_mode);
                        }
                        if let Some(scoring_statistics) = &this.scoring_statistics {
                            req.url_mut().query_pairs_mut().append_pair("scoringStatistics", scoring_statistics);
                        }
                        if let Some(session_id) = &this.session_id {
                            req.url_mut().query_pairs_mut().append_pair("sessionId", session_id);
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut().query_pairs_mut().append_pair("$skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(captions) = &this.captions {
                            req.url_mut().query_pairs_mut().append_pair("captions", captions);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SearchDocumentsResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SearchDocumentsResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod search_post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SearchDocumentsResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SearchDocumentsResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) search_request: models::SearchRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.search_request)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/search.post.search", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SearchDocumentsResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SearchDocumentsResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::LookupDocument> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::LookupDocument = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) key: String,
            pub(crate) select: Vec<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "List of field names to retrieve for the document; Any field not retrieved will be missing from the returned document."]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs('{}')", self.client.endpoint(), &self.key))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::LookupDocument>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::LookupDocument>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod suggest_get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SuggestDocumentsResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SuggestDocumentsResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) search: String,
            pub(crate) suggester_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) fuzzy: Option<bool>,
            pub(crate) highlight_post_tag: Option<String>,
            pub(crate) highlight_pre_tag: Option<String>,
            pub(crate) minimum_coverage: Option<f64>,
            pub(crate) orderby: Vec<String>,
            pub(crate) search_fields: Vec<String>,
            pub(crate) select: Vec<String>,
            pub(crate) top: Option<i32>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "An OData expression that filters the documents considered for suggestions."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "A value indicating whether to use fuzzy matching for the suggestions query. Default is false. When set to true, the query will find terms even if there's a substituted or missing character in the search text. While this provides a better experience in some scenarios, it comes at a performance cost as fuzzy suggestions queries are slower and consume more resources."]
            pub fn fuzzy(mut self, fuzzy: bool) -> Self {
                self.fuzzy = Some(fuzzy);
                self
            }
            #[doc = "A string tag that is appended to hit highlights. Must be set with highlightPreTag. If omitted, hit highlighting of suggestions is disabled."]
            pub fn highlight_post_tag(mut self, highlight_post_tag: impl Into<String>) -> Self {
                self.highlight_post_tag = Some(highlight_post_tag.into());
                self
            }
            #[doc = "A string tag that is prepended to hit highlights. Must be set with highlightPostTag. If omitted, hit highlighting of suggestions is disabled."]
            pub fn highlight_pre_tag(mut self, highlight_pre_tag: impl Into<String>) -> Self {
                self.highlight_pre_tag = Some(highlight_pre_tag.into());
                self
            }
            #[doc = "A number between 0 and 100 indicating the percentage of the index that must be covered by a suggestions query in order for the query to be reported as a success. This parameter can be useful for ensuring search availability even for services with only one replica. The default is 80."]
            pub fn minimum_coverage(mut self, minimum_coverage: f64) -> Self {
                self.minimum_coverage = Some(minimum_coverage);
                self
            }
            #[doc = "The list of OData $orderby expressions by which to sort the results. Each expression can be either a field name or a call to either the geo.distance() or the search.score() functions. Each expression can be followed by asc to indicate ascending, or desc to indicate descending. The default is ascending order. Ties will be broken by the match scores of documents. If no $orderby is specified, the default sort order is descending by document match score. There can be at most 32 $orderby clauses."]
            pub fn orderby(mut self, orderby: Vec<String>) -> Self {
                self.orderby = orderby;
                self
            }
            #[doc = "The list of field names to search for the specified search text. Target fields must be included in the specified suggester."]
            pub fn search_fields(mut self, search_fields: Vec<String>) -> Self {
                self.search_fields = search_fields;
                self
            }
            #[doc = "The list of fields to retrieve. If unspecified, only the key field will be included in the results."]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "The number of suggestions to retrieve. The value must be a number between 1 and 100. The default is 5."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let search = &this.search;
                        req.url_mut().query_pairs_mut().append_pair("search", search);
                        let suggester_name = &this.suggester_name;
                        req.url_mut().query_pairs_mut().append_pair("suggesterName", suggester_name);
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(fuzzy) = &this.fuzzy {
                            req.url_mut().query_pairs_mut().append_pair("fuzzy", &fuzzy.to_string());
                        }
                        if let Some(highlight_post_tag) = &this.highlight_post_tag {
                            req.url_mut().query_pairs_mut().append_pair("highlightPostTag", highlight_post_tag);
                        }
                        if let Some(highlight_pre_tag) = &this.highlight_pre_tag {
                            req.url_mut().query_pairs_mut().append_pair("highlightPreTag", highlight_pre_tag);
                        }
                        if let Some(minimum_coverage) = &this.minimum_coverage {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("minimumCoverage", &minimum_coverage.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/search.suggest", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SuggestDocumentsResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SuggestDocumentsResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod suggest_post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SuggestDocumentsResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SuggestDocumentsResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) suggest_request: models::SuggestRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.suggest_request)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/search.post.suggest", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SuggestDocumentsResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SuggestDocumentsResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod index {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IndexDocumentsResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IndexDocumentsResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) batch: models::IndexBatch,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.batch)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/search.index", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::IndexDocumentsResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::IndexDocumentsResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod autocomplete_get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AutocompleteResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AutocompleteResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) search: String,
            pub(crate) suggester_name: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) autocomplete_mode: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) fuzzy: Option<bool>,
            pub(crate) highlight_post_tag: Option<String>,
            pub(crate) highlight_pre_tag: Option<String>,
            pub(crate) minimum_coverage: Option<f64>,
            pub(crate) search_fields: Vec<String>,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the mode for Autocomplete. The default is 'oneTerm'. Use 'twoTerms' to get shingles and 'oneTermWithContext' to use the current context while producing auto-completed terms."]
            pub fn autocomplete_mode(mut self, autocomplete_mode: impl Into<String>) -> Self {
                self.autocomplete_mode = Some(autocomplete_mode.into());
                self
            }
            #[doc = "An OData expression that filters the documents used to produce completed terms for the Autocomplete result."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "A value indicating whether to use fuzzy matching for the autocomplete query. Default is false. When set to true, the query will find terms even if there's a substituted or missing character in the search text. While this provides a better experience in some scenarios, it comes at a performance cost as fuzzy autocomplete queries are slower and consume more resources."]
            pub fn fuzzy(mut self, fuzzy: bool) -> Self {
                self.fuzzy = Some(fuzzy);
                self
            }
            #[doc = "A string tag that is appended to hit highlights. Must be set with highlightPreTag. If omitted, hit highlighting is disabled."]
            pub fn highlight_post_tag(mut self, highlight_post_tag: impl Into<String>) -> Self {
                self.highlight_post_tag = Some(highlight_post_tag.into());
                self
            }
            #[doc = "A string tag that is prepended to hit highlights. Must be set with highlightPostTag. If omitted, hit highlighting is disabled."]
            pub fn highlight_pre_tag(mut self, highlight_pre_tag: impl Into<String>) -> Self {
                self.highlight_pre_tag = Some(highlight_pre_tag.into());
                self
            }
            #[doc = "A number between 0 and 100 indicating the percentage of the index that must be covered by an autocomplete query in order for the query to be reported as a success. This parameter can be useful for ensuring search availability even for services with only one replica. The default is 80."]
            pub fn minimum_coverage(mut self, minimum_coverage: f64) -> Self {
                self.minimum_coverage = Some(minimum_coverage);
                self
            }
            #[doc = "The list of field names to consider when querying for auto-completed terms. Target fields must be included in the specified suggester."]
            pub fn search_fields(mut self, search_fields: Vec<String>) -> Self {
                self.search_fields = search_fields;
                self
            }
            #[doc = "The number of auto-completed terms to retrieve. This must be a value between 1 and 100. The default is 5."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let search = &this.search;
                        req.url_mut().query_pairs_mut().append_pair("search", search);
                        let suggester_name = &this.suggester_name;
                        req.url_mut().query_pairs_mut().append_pair("suggesterName", suggester_name);
                        if let Some(autocomplete_mode) = &this.autocomplete_mode {
                            req.url_mut().query_pairs_mut().append_pair("autocompleteMode", autocomplete_mode);
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(fuzzy) = &this.fuzzy {
                            req.url_mut().query_pairs_mut().append_pair("fuzzy", &fuzzy.to_string());
                        }
                        if let Some(highlight_post_tag) = &this.highlight_post_tag {
                            req.url_mut().query_pairs_mut().append_pair("highlightPostTag", highlight_post_tag);
                        }
                        if let Some(highlight_pre_tag) = &this.highlight_pre_tag {
                            req.url_mut().query_pairs_mut().append_pair("highlightPreTag", highlight_pre_tag);
                        }
                        if let Some(minimum_coverage) = &this.minimum_coverage {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("minimumCoverage", &minimum_coverage.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/search.autocomplete", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AutocompleteResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AutocompleteResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod autocomplete_post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AutocompleteResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AutocompleteResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) autocomplete_request: models::AutocompleteRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The tracking ID sent with the request to help with debugging."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.autocomplete_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/docs/search.post.autocomplete", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-01-Preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AutocompleteResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AutocompleteResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
