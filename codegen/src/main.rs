use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError, Renderable};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    #[serde(rename = "Categories")]
    pub categories: Vec<Category>,
    #[serde(rename = "Characteristics")]
    pub characteristics: Vec<Characteristic>,
    #[serde(rename = "Services")]
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Category")]
    pub number: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Characteristic {
    #[serde(rename = "UUID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<Constraints>,
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Properties")]
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Constraints {
    #[serde(rename = "ValidValues")]
    pub valid_values: Option<HashMap<String, String>>,
    #[serde(rename = "MaximumValue")]
    pub max_value: Option<serde_json::Value>,
    #[serde(rename = "MinimumValue")]
    pub min_value: Option<serde_json::Value>,
    #[serde(rename = "StepValue")]
    pub step_value: Option<serde_json::Value>,
    #[serde(rename = "MaximumLength")]
    pub max_len: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    #[serde(rename = "UUID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RequiredCharacteristics")]
    pub required_characteristics: Vec<String>,
    #[serde(rename = "OptionalCharacteristics")]
    pub optional_characteristics: Vec<String>,
}

#[derive(Debug)]
struct MetadataEx<'a> {
    metadata: Metadata,
    characteristics: std::collections::HashMap<String, &'a Characteristic>,
}

fn if_eq_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    c: &Context,
    rc: &mut RenderContext<'reg>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let first = h.param(0).unwrap().value();
    let second = h.param(1).unwrap().value();
    let tmpl = if first == second { h.template() } else { h.inverse() };
    match tmpl {
        Some(ref t) => t.render(r, c, rc, out),
        None => Ok(()),
    }
}

fn trim_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let trim = s.replace(" ", "").replace(".", "_");
        out.write(&trim)?;
    }
    Ok(())
}

fn file_name_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let name = s.replace(" ", "_").replace(".", "_").to_lowercase();
        out.write(&name)?;
    }
    Ok(())
}

fn type_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => {
                out.write("bool")?;
            },
            "uint8" => {
                out.write("u8")?;
            },
            "uint16" => {
                out.write("u16")?;
            },
            "uint32" => {
                out.write("u32")?;
            },
            "uint64" => {
                out.write("u64")?;
            },
            "int" => {
                out.write("i32")?;
            },
            "int32" => {
                out.write("i32")?;
            },
            "float" => {
                out.write("f32")?;
            },
            "string" => {
                out.write("String")?;
            },
            "tlv8" => {
                out.write("Vec<u8>")?;
            },
            "data" => {
                out.write("Vec<u8>")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic format"));
            },
        }
    }
    Ok(())
}

fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => {
                out.write("Format::Bool")?;
            },
            "uint8" => {
                out.write("Format::UInt8")?;
            },
            "uint16" => {
                out.write("Format::UInt16")?;
            },
            "uint32" => {
                out.write("Format::UInt32")?;
            },
            "uint64" => {
                out.write("Format::UInt64")?;
            },
            "int" => {
                out.write("Format::Int32")?;
            },
            "int32" => {
                out.write("Format::Int32")?;
            },
            "float" => {
                out.write("Format::Float")?;
            },
            "string" => {
                out.write("Format::String")?;
            },
            "tlv8" => {
                out.write("Format::Tlv8")?;
            },
            "data" => {
                out.write("Format::Data")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic format"));
            },
        }
    }
    Ok(())
}

fn unit_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "percentage" => {
                out.write("Unit::Percentage")?;
            },
            "arcdegrees" => {
                out.write("Unit::ArcDegrees")?;
            },
            "celsius" => {
                out.write("Unit::Celsius")?;
            },
            "lux" => {
                out.write("Unit::Lux")?;
            },
            "seconds" => {
                out.write("Unit::Seconds")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic unit"));
            },
        }
    }
    Ok(())
}

fn uuid_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        out.write(&shorten_uuid(&s))?;
    }
    Ok(())
}

fn valid_values_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_object().unwrap();
    let mut output = String::from("vec![\n");
    for (key, val) in param {
        output.push_str(&format!("\t\t\t\t\t{}, // {}\n", key, val));
    }
    output.push_str("\t\t\t\t]");
    out.write(&output)?;
    Ok(())
}

fn perms_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let params = h.param(0).unwrap().value().as_array().unwrap();
    for param in params {
        match param.as_str() {
            Some("read") => {
                out.write("\n\t\t\t\t\tPerm::PairedRead,")?;
            },
            Some("write") => {
                out.write("\n\t\t\t\t\tPerm::PairedWrite,")?;
            },
            Some("cnotify") => {
                out.write("\n\t\t\t\t\tPerm::Events,")?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn float_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let format = h.param(0).unwrap().value().as_str().unwrap();
    if format == "float" {
        out.write(" as f32")?;
    }
    Ok(())
}

fn shorten_uuid(id: &str) -> String {
    id.split("-").collect::<Vec<&str>>()[0]
        .trim_start_matches('0')
        .to_owned()
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let name = param.replace(" ", "_").replace(".", "_").to_lowercase();
    out.write(&name)?;
    Ok(())
}

fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let name = param
        .to_lowercase()
        .split(" ")
        .into_iter()
        .map(|word| {
            let mut c = word.chars().collect::<Vec<char>>();
            c[0] = c[0].to_uppercase().nth(0).unwrap();
            c.into_iter().collect::<String>()
        })
        .collect::<String>();
    let name = name.replace(" ", "").replace(".", "_");
    out.write(&name)?;
    Ok(())
}

static CATEGORIES: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::{Deserialize, Serialize};

/// HAP Accessory category.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AccessoryCategory {
{{#each Categories as |c|}}\
\t{{pascal_case c.Name}} = {{c.Category}},
{{/each}}\
}
";

static HAP_TYPE: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use std::str::FromStr;

use crate::Error;

/// HAP Service and Characteristic type representation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HapType {
    Unknown,
{{#each Characteristics as |c|}}\
\t{{pascal_case c.Name}},
{{/each}}\
{{#each Services as |s|}}\
\t{{pascal_case s.Name}},
{{/each}}\
}

impl ToString for HapType {
    fn to_string(&self) -> String {
        match self {
            HapType::Unknown => \"unknown\".into(),
{{#each Characteristics as |c|}}\
\t\t\tHapType::{{pascal_case c.Name}} => \"{{uuid c.UUID}}\".into(),
{{/each}}\
{{#each Services as |s|}}\
\t\t\tHapType::{{pascal_case s.Name}} => \"{{uuid s.UUID}}\".into(),
{{/each}}\
\t\t}
    }
}

impl FromStr for HapType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            \"unknown\" => Ok(HapType::Unknown),
{{#each Characteristics as |c|}}\
\t\t\t\"{{uuid c.UUID}}\" => Ok(HapType::{{pascal_case c.Name}}),
{{/each}}\
{{#each Services as |s|}}\
\t\t\t\"{{uuid s.UUID}}\" => Ok(HapType::{{pascal_case s.Name}}),
{{/each}}\
\t\t\t_ => Err(Error::InvalidHapTypeString(s.to_string())),
\t\t}
    }
}

impl Default for HapType {
    fn default() -> HapType { HapType::Unknown }
}

impl<'de> Deserialize<'de> for HapType {
    fn deserialize<D>(deserializer: D) -> Result<HapType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let hap_type = HapType::from_str(&s).map_err(de::Error::custom)?;
        Ok(hap_type)
    }
}

impl Serialize for HapType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
";

static CHARACTERISTIC: &'static str = "// this file is auto-generated by hap-codegen\n
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;

use crate::{
    characteristic::{
        AsyncCharacteristicCallbacks,
        Characteristic,
        CharacteristicCallbacks,
        Format,
        HapCharacteristic,
        HapCharacteristicSetup,
        HapType,
        OnReadFn,
        OnReadFuture,
        OnUpdateFn,
        OnUpdateFuture,
        Perm,
        Unit,
    },
    pointer,
    Error,
    Result,
};

/// {{characteristic.Name}} Characteristic.
#[derive(Debug, Default, Serialize)]
pub struct {{pascal_case characteristic.Name}}Characteristic(Characteristic<{{type characteristic.Format}}>);

impl {{pascal_case characteristic.Name}}Characteristic {
    /// Creates a new {{characteristic.Name}} Characteristic.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self(Characteristic::<{{type characteristic.Format}}> {
            id,
            accessory_id,
            hap_type: HapType::{{pascal_case characteristic.Name}},
            format: {{format characteristic.Format}},
            perms: vec![{{perms characteristic.Properties}}
            ],\
            {{#if characteristic.Unit}}\n\t\t\t\tunit: Some({{unit characteristic.Unit}}),{{/if}}\
            {{#if characteristic.Constraints.MaximumValue includeZero=true}}\n\t\t\t\tmax_value: Some({{characteristic.Constraints.MaximumValue}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.Constraints.MinimumValue includeZero=true}}\n\t\t\t\tmin_value: Some({{characteristic.Constraints.MinimumValue}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.Constraints.StepValue includeZero=true}}\n\t\t\t\tstep_value: Some({{characteristic.Constraints.StepValue}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.Constraints.MaximumLength includeZero=true}}\n\t\t\t\tmax_len: Some({{characteristic.Constraints.MaximumLength}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.Constraints.MaximumDataLength includeZero=true}}\n\t\t\t\tmax_data_len: Some({{characteristic.Constraints.MaximumDataLength}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.Constraints.ValidValues includeZero=true}}\n\t\t\t\tvalid_values: Some({{valid_values characteristic.Constraints.ValidValues}}),{{/if}}
            ..Default::default()
        })
    }
}

#[async_trait]
impl HapCharacteristic for {{pascal_case characteristic.Name}}Characteristic {
    fn get_id(&self) -> u64 { self.0.get_id() }

    fn get_type(&self) -> HapType { self.0.get_type() }

    fn get_format(&self) -> Format { self.0.get_format() }

    fn get_perms(&self) -> Vec<Perm> { self.0.get_perms() }

    fn get_event_notifications(&self) -> Option<bool> { self.0.get_event_notifications() }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        self.0.set_event_notifications(event_notifications)
    }

    async fn get_value(&mut self) -> Result<serde_json::Value> {
        let value = self.0.get_value().await?;
        Ok(json!(value))
    }

    async fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        let v;
        // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
        if self.0.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::InvalidValue(self.get_format()));
            }
        } else {
            v = serde_json::from_value(value).map_err(|_| Error::InvalidValue(self.get_format()))?;
        }
        self.0.set_value(v).await
    }

    fn get_unit(&self) -> Option<Unit> { self.0.get_unit() }

    fn get_max_value(&self) -> Option<serde_json::Value> { self.0.get_max_value().map(|v| json!(v)) }

    fn get_min_value(&self) -> Option<serde_json::Value> { self.0.get_min_value().map(|v| json!(v)) }

    fn get_step_value(&self) -> Option<serde_json::Value> { self.0.get_step_value().map(|v| json!(v)) }

    fn get_max_len(&self) -> Option<u16> { self.0.get_max_len() }
}

impl HapCharacteristicSetup for {{pascal_case characteristic.Name}}Characteristic {
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        self.0.set_event_emitter(event_emitter)
    }
}

impl CharacteristicCallbacks<{{type characteristic.Format}}> for {{pascal_case characteristic.Name}}Characteristic {
    fn on_read(&mut self, f: Option<impl OnReadFn<{{type characteristic.Format}}>>) { self.0.on_read(f) }

    fn on_update(&mut self, f: Option<impl OnUpdateFn<{{type characteristic.Format}}>>) { self.0.on_update(f) }
}

impl AsyncCharacteristicCallbacks<{{type characteristic.Format}}> for {{pascal_case characteristic.Name}}Characteristic {
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<{{type characteristic.Format}}>>) { self.0.on_read_async(f) }

    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<{{type characteristic.Format}}>>) { self.0.on_update_async(f) }
}
";

static CHARACTERISTIC_MOD: &'static str = "// this file is auto-generated by hap-codegen
{{#each characteristics as |c|}}\npub mod {{c}};{{/each}}
";

static SERVICE: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
{{#each required_characteristics as |r|}}\
\t\t{{snake_case r.Name}}::{{pascal_case r.Name}}Characteristic,
{{/each}}\
{{#each optional_characteristics as |r|}}\
\t\t{{snake_case r.Name}}::{{pascal_case r.Name}}Characteristic,
{{/each}}\
\t},
    HapType,
};

/// {{service.Name}} Service.
#[derive(Debug, Default)]
pub struct {{pascal_case service.Name}}Service {
    /// ID of the {{service.Name}} Service.
    id: u64,
    /// `HapType` of the {{service.Name}} Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

{{#each required_characteristics as |r|}}\
\t/// {{r.Name}} Characteristic (required).
\tpub {{snake_case r.Name}}: {{pascal_case r.Name}}Characteristic,
{{/each}}\
\n{{#each optional_characteristics as |r|}}\
\t/// {{r.Name}} Characteristic (optional).
\tpub {{snake_case r.Name}}: Option<{{pascal_case r.Name}}Characteristic>,
{{/each}}\
}

impl {{pascal_case service.Name}}Service {
    /// Creates a new {{service.Name}} Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::{{pascal_case service.Name}},
{{#each required_characteristics as |r|}}\
\t\t\t{{snake_case r.Name}}: {{pascal_case r.Name}}Characteristic::new(id + 1 + {{@index}}, accessory_id),
{{/each}}\
        \t\t\t..Default::default()
        }
    }
}

impl HapService for {{pascal_case service.Name}}Service {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
{{#each required_characteristics as |r|}}\
\t\t\t&self.{{snake_case r.Name}},
{{/each}}\
        \t\t];
{{#each optional_characteristics as |r|}}\
\t\tif let Some(c) = &self.{{snake_case r.Name}} {
\t\t    characteristics.push(c);
\t\t}
{{/each}}\
        \t\tcharacteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
{{#each required_characteristics as |r|}}\
\t\t\t&mut self.{{snake_case r.Name}},
{{/each}}\
        \t\t];
{{#each optional_characteristics as |r|}}\
\t\tif let Some(c) = &mut self.{{snake_case r.Name}} {
\t\t    characteristics.push(c);
\t\t}
{{/each}}\
        \t\tcharacteristics
    }
}

impl Serialize for {{pascal_case service.Name}}Service {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct(\"HapService\", 5)?;
        state.serialize_field(\"iid\", &self.get_id())?;
        state.serialize_field(\"type\", &self.get_type())?;
        state.serialize_field(\"hidden\", &self.get_hidden())?;
        state.serialize_field(\"primary\", &self.get_primary())?;
        state.serialize_field(\"characteristics\", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
";

static SERVICE_MOD: &'static str = "// this file is auto-generated by hap-codegen
{{#each services as |s|}}\npub mod {{s}};{{/each}}
";

static ACCESSORY: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
\taccessory::{AccessoryInformation, HapAccessory},
\tservice::{HapService, accessory_information::AccessoryInformationService, {{snake_case service.Name}}::{{pascal_case service.Name}}Service},
\tHapType,
\tResult,
};

/// {{service.Name}} Accessory.
#[derive(Debug, Default)]
pub struct {{pascal_case service.Name}}Accessory {
    /// ID of the {{service.Name}} Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// {{service.Name}} Service.
    pub {{snake_case service.Name}}: {{pascal_case service.Name}}Service,
}

impl {{pascal_case service.Name}}Accessory {
    /// Creates a new {{service.Name}} Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let {{snake_case service.Name}}_id = accessory_information.get_characteristics().len() as u64;
        let mut {{snake_case service.Name}} = {{pascal_case service.Name}}Service::new(1 + {{snake_case service.Name}}_id + 1, id);
        {{snake_case service.Name}}.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            {{snake_case service.Name}},
        })
    }
}

impl HapAccessory for {{pascal_case service.Name}}Accessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![
            &self.accessory_information,
            &self.{{snake_case service.Name}},
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.{{snake_case service.Name}},
        ]
    }
}

impl Serialize for {{pascal_case service.Name}}Accessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct(\"HapAccessory\", 2)?;
        state.serialize_field(\"aid\", &self.get_id())?;
        state.serialize_field(\"services\", &self.get_services())?;
        state.end()
    }
}
";

static ACCESSORY_MOD: &'static str = "// this file is auto-generated by hap-codegen
{{#each accessories as |a|}}\npub mod {{a}};{{/each}}
";

// static EXAMPLE: &'static str = "\
// use std::net::{IpAddr, SocketAddr};

// use hap::{
//     accessory::{ {{~snake_case service.Name~}} ::{{pascal_case service.Name}}Accessory, AccessoryCategory,
// AccessoryInformation},     server::{IpServer, Server},
//     storage::FileStorage,
//     tokio,
//     Config,
//     MacAddress,
//     Pin,
// };

// #[tokio::main]
// async fn main() {
//     let current_ipv4 = || -> Option<IpAddr> {
//         for iface in pnet::datalink::interfaces() {
//             for ip_network in iface.ips {
//                 if ip_network.is_ipv4() {
//                     let ip = ip_network.ip();
//                     if !ip.is_loopback() {
//                         return Some(ip);
//                     }
//                 }
//             }
//         }
//         None
//     };

//     let lightbulb = {{pascal_case service.Name}}Accessory::new(1, AccessoryInformation {
//         name: \"Acme {{service.Name}}\".into(),
//         ..Default::default()
//     })
//     .unwrap();

//     let config = Config {
//         socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
//         pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
//         name: \"Acme {{service.Name}}\".into(),
//         device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
//         category: AccessoryCategory::{{pascal_case service.Name}},
//         ..Default::default()
//     };
//     let storage = FileStorage::current_dir().await.unwrap();

//     let mut server = IpServer::new(config, storage).unwrap();
//     server.add_accessory(lightbulb).await.unwrap();

//     let handle = server.run_handle();

//     std::env::set_var(\"RUST_LOG\", \"hap=info\");
//     env_logger::init();

//     handle.await;
// }
// ";

fn main() {
    let metadata_file = File::open("codegen/gen/default.json").unwrap();
    let mut metadata_ex = MetadataEx {
        metadata: serde_json::from_reader(&metadata_file).unwrap(),
        characteristics: std::collections::HashMap::new(),
    };
    let metadata = &metadata_ex.metadata;

    // build characteristic map
    for c in &metadata.characteristics {
        metadata_ex.characteristics.insert(c.id.to_string(), &c);
    }

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("if_eq", Box::new(if_eq_helper));
    handlebars.register_helper("trim", Box::new(trim_helper));
    handlebars.register_helper("file_name", Box::new(file_name_helper));
    handlebars.register_helper("format", Box::new(format_helper));
    handlebars.register_helper("type", Box::new(type_helper));
    handlebars.register_helper("unit", Box::new(unit_helper));
    handlebars.register_helper("uuid", Box::new(uuid_helper));
    handlebars.register_helper("valid_values", Box::new(valid_values_helper));
    handlebars.register_helper("perms", Box::new(perms_helper));
    handlebars.register_helper("float", Box::new(float_helper));
    handlebars.register_helper("snake_case", Box::new(snake_case_helper));
    handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
    handlebars.register_template_string("categories", CATEGORIES).unwrap();
    handlebars.register_template_string("hap_type", HAP_TYPE).unwrap();
    handlebars
        .register_template_string("characteristic", CHARACTERISTIC)
        .unwrap();
    handlebars
        .register_template_string("characteristic_mod", CHARACTERISTIC_MOD)
        .unwrap();
    handlebars.register_template_string("service", SERVICE).unwrap();
    handlebars.register_template_string("service_mod", SERVICE_MOD).unwrap();
    handlebars.register_template_string("accessory", ACCESSORY).unwrap();
    handlebars
        .register_template_string("accessory_mod", ACCESSORY_MOD)
        .unwrap();
    // handlebars.register_template_string("example", EXAMPLE).unwrap();

    let categories = handlebars.render("categories", &metadata).unwrap();
    let categories_path = "src/accessory/category.rs".to_owned();
    let mut categories_file = File::create(&categories_path).unwrap();
    categories_file.write_all(categories.as_bytes()).unwrap();

    let hap_type = handlebars.render("hap_type", &metadata).unwrap();
    let hap_type_path = "src/hap_type.rs".to_owned();
    let mut hap_type_file = File::create(&hap_type_path).unwrap();
    hap_type_file.write_all(hap_type.as_bytes()).unwrap();

    let characteristic_base_path = "src/characteristic/generated/";
    if std::path::Path::new(&characteristic_base_path).exists() {
        fs::remove_dir_all(&characteristic_base_path).unwrap();
    }
    fs::create_dir_all(&characteristic_base_path).unwrap();
    let mut characteristsic_names = vec![];
    for c in &metadata.characteristics {
        let characteristic = handlebars
            .render("characteristic", &json!({ "characteristic": c }))
            .unwrap();
        let characteristic_file_name = c.name.replace(" ", "_").replace(".", "_").to_lowercase();
        let mut characteristic_path = String::from(characteristic_base_path);
        characteristic_path.push_str(&characteristic_file_name);
        characteristic_path.push_str(".rs");
        let mut characteristic_file = File::create(&characteristic_path).unwrap();
        characteristic_file.write_all(characteristic.as_bytes()).unwrap();
        characteristsic_names.push(characteristic_file_name);
    }
    let characteristic_mod = handlebars
        .render(
            "characteristic_mod",
            &json!({ "characteristics": characteristsic_names }),
        )
        .unwrap();
    let mut characteristic_mod_file = File::create(&format!("{}mod.rs", characteristic_base_path)).unwrap();
    characteristic_mod_file
        .write_all(characteristic_mod.as_bytes())
        .unwrap();

    let service_base_path = "src/service/generated/";
    let accessory_base_path = "src/accessory/generated/";
    if std::path::Path::new(&service_base_path).exists() {
        fs::remove_dir_all(&service_base_path).unwrap();
    }
    if std::path::Path::new(&accessory_base_path).exists() {
        fs::remove_dir_all(&accessory_base_path).unwrap();
    }
    fs::create_dir_all(&service_base_path).unwrap();
    fs::create_dir_all(&accessory_base_path).unwrap();
    let mut service_names = vec![];
    let mut accessory_names = vec![];
    for s in &metadata.services {
        let mut required_characteristics = Vec::new();
        let mut optional_characteristics = Vec::new();

        for c in &s.required_characteristics {
            required_characteristics.push(metadata_ex.characteristics[c]);
        }

        for c in &s.optional_characteristics {
            optional_characteristics.push(metadata_ex.characteristics[c]);
        }

        let service = handlebars
            .render(
                "service",
                &json!({
                    "service": s,
                    "required_characteristics": &required_characteristics,
                    "optional_characteristics": &optional_characteristics,
                }),
            )
            .unwrap();

        let service_file_name = s.name.replace(" ", "_").replace(".", "_").to_lowercase();
        let mut service_path = String::from(service_base_path);
        service_path.push_str(&service_file_name);
        service_path.push_str(".rs");
        let mut service_file = File::create(&service_path).unwrap();
        service_file.write_all(service.as_bytes()).unwrap();
        service_names.push(service_file_name.clone());

        if s.name != "Accessory Information"
            && s.name != "Battery Service"
            && s.name != "Camera RTP Stream Management"
            && s.name != "Doorbell"
            && s.name != "Faucet"
            && s.name != "Filter Maintenance"
            && s.name != "Irrigation System"
            && s.name != "Lock Management"
            && s.name != "Lock Mechanism"
            && s.name != "Microphone"
            && s.name != "Service Label"
            && s.name != "Slat"
            && s.name != "Speaker"
            && s.name != "Television"
            && s.name != "Input Source"
        {
            let accessory = handlebars
                .render(
                    "accessory",
                    &json!({"service": s, "characteristics": &metadata.characteristics}),
                )
                .unwrap();
            let mut accessory_path = String::from(accessory_base_path);
            accessory_path.push_str(&service_file_name);
            accessory_path.push_str(".rs");
            let mut accessory_file = File::create(&accessory_path).unwrap();
            accessory_file.write_all(accessory.as_bytes()).unwrap();
            accessory_names.push(service_file_name);
        }
    }
    let service_mod = handlebars
        .render("service_mod", &json!({ "services": service_names }))
        .unwrap();
    let mut service_mod_file = File::create(&format!("{}mod.rs", service_base_path)).unwrap();
    service_mod_file.write_all(service_mod.as_bytes()).unwrap();
    let accessory_mod = handlebars
        .render("accessory_mod", &json!({ "accessories": accessory_names }))
        .unwrap();
    let mut accessory_mod_file = File::create(&format!("{}mod.rs", accessory_base_path)).unwrap();
    accessory_mod_file.write_all(accessory_mod.as_bytes()).unwrap();
}
