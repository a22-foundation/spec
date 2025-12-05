/**
 * A22 Canonical IR v1.0
 * Rust Definition Reference
 * 
 * Auto-generated from spec/ir.schema.json
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct A22IR {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub agents: Vec<Agent>,
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<CustomType>>,
    pub flows: Vec<Flow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Capability>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelConfig>,
    pub system_prompt: Option<String>,
    pub inputs: Option<Vec<TypeRef>>,
    pub outputs: Option<Vec<TypeRef>>,
    pub memory: Option<MemoryConfig>,
    pub permissions: Option<Vec<Permission>>,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub provider: String,
    pub name: String,
    pub params: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryConfig {
    pub enabled: Option<bool>,
    pub strategy: Option<MemoryStrategy>,
    pub params: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MemoryStrategy {
    None,
    Window,
    Semantic,
    Hybrid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub resource: String,
    pub action: PermissionAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PermissionAction {
    Read,
    Write,
    Execute,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub schema: ToolSchema,
    pub runtime: Option<ToolRuntime>,
    pub config: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolSchema {
    pub input: Option<TypeRef>,
    pub output: Option<TypeRef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolRuntime {
    Http,
    Js,
    Python,
    Native,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomType {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: TypeRef,
    pub optional: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeRef {
    #[serde(rename = "type")]
    pub type_name: TypeName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<TypeRef>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TypeName {
    String,
    Number,
    Boolean,
    Object,
    Array,
    Any,
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flow {
    pub id: String,
    pub name: String,
    pub inputs: Option<Vec<TypeRef>>,
    pub steps: Vec<FlowStep>,
    pub outputs: Option<Vec<TypeRef>>,
    pub error_boundary: Option<ErrorBoundary>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlowStep {
    pub id: String,
    pub kind: FlowStepKind,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subflow: Option<String>,
    
    pub input_map: Option<HashMap<String, serde_json::Value>>,
    pub output_map: Option<HashMap<String, serde_json::Value>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<Branch>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel: Option<Vec<FlowStep>>,
    
    pub retry: Option<RetryPolicy>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FlowStepKind {
    Agent,
    Tool,
    Branch,
    Parallel,
    Subflow,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub when: String,
    pub steps: Vec<FlowStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetryPolicy {
    pub max_retries: Option<u32>,
    pub backoff: Option<BackoffStrategy>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BackoffStrategy {
    None,
    Linear,
    Exponential,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorBoundary {
    pub on_error: Option<ErrorStrategy>,
    pub handler_flow: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ErrorStrategy {
    Retry,
    Skip,
    Stop,
    Handler,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Capability {
    pub id: String,
    pub description: String,
    pub params: Option<HashMap<String, serde_json::Value>>,
}
