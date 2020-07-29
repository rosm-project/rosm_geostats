//! Serde structs for Mapbox's JSON-based `geostats` format.
//!
//! **Note**: `geostats` writer implementations (most notably [mapbox-geostats](https://github.com/mapbox/mapbox-geostats))
//! may have limitations on how many layers, attributes and values they can process in detail. That's why there are separate
//! `count` properties for these arrays: the arrays may not contain `count` entries, and even `count` may not represent
//! the actual number of elements in the input.

use serde::{Deserialize, Serialize};

/// Root `geostats` object.
///
/// # Examples
///
/// ```no_run
/// use rosm_geostats::Tilestats;
///
/// let tilestats: Tilestats = serde_json::from_str("...").unwrap();
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tilestats {
    /// Number of layers in the source data.
    pub layer_count: usize,

    /// Array of details about the layers.
    pub layers: Vec<Layer>,
}

/// Layer details.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    /// Name of the layer.
    pub layer: String,

    /// Number of features in the layer.
    pub count: usize,

    /// Dominant geometry type in the layer.
    pub geometry: Geometry,

    /// Number of unique attributes in the layer.
    pub attribute_count: usize,

    /// Array of details about the attributes.
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

/// Possible geometry types in layers.
#[derive(Debug, Serialize, Deserialize)]
pub enum Geometry {
    Point, LineString, Polygon
}

/// Feature attribute details.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    /// Name of the attribute.
    pub attribute: String,

    /// Number of unique values for the attribute.
    pub count: usize,

    /// Type of the attribute's values.
    pub r#type: ValueType,

    /// Unique values of the attribute.
    pub values: Vec<serde_json::Value>,

    /// Minimum of any number value.
    pub min: Option<f64>,

    /// Maximum of any number value.
    pub max: Option<f64>,
}

/// Possible attribute value types.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValueType {
    String, Number, Boolean, Null, Mixed
}
