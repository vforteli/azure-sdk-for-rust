#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-10-01-preview")]
pub mod package_2023_10_01_preview;
#[cfg(feature = "package-2023-10-searchindex-preview")]
pub mod package_2023_10_searchindex_preview;
#[cfg(feature = "package-2023-10-searchservice-preview")]
pub mod package_2023_10_searchservice_preview;
#[cfg(feature = "package-2023-11-searchindex")]
pub mod package_2023_11_searchindex;
#[cfg(feature = "package-2023-11-searchservice")]
pub mod package_2023_11_searchservice;
#[cfg(all(feature = "default_tag", feature = "package-2023-11-searchservice"))]
pub use package_2023_11_searchservice::*;
