# A22 DSL Specification

## 1. A22 Overview
A22 is a declarative domain-specific language (DSL) for defining, orchestrating, and deploying agentic systems. It allows architects to define agents, tools, memories, and their interactions as a static, deterministic configuration.

A22 treats agent systems as **Acyclic Directed Graphs (ADGs)** of execution steps, where data flows between nodes (agents, tools, routers) based on events and state. It is designed to be the "Terraform for Agents"—providing a clear, readable, and version-controllable representation of complex AI behaviors.

## 2. Design Principles
1.  **Declarative over Imperative**: Define *what* the system looks like, not *how* to construct it step-by-step.
2.  **Explicit Dependencies**: All relationships between agents and tools must be explicit. No hidden side channels.
3.  **Deterministic Configuration**: The same A22 file should always yield the same system graph structure.
4.  **Minimalism**: Only essential primitives (`agent`, `tool`, `event`, `route`, `memory`). No "magic" or hidden behaviors.
5.  **Composition**: Systems are built from reusable modules and blocks.
6.  **Type Safety**: Inputs and outputs are typed to ensure graph validity at compile time.

## 3. Language Syntax & Grammar (EBNF)
The syntax is inspired by HCL (HashiCorp Configuration Language) but simplified.

```ebnf
configuration = { block } ;

block         = identifier , [ string_literal ] , "{" , { attribute | block } , "}" ;

attribute     = identifier , "=" , expression ;

expression    = literal
              | variable_ref
              | list
              | map
              | function_call ;

literal       = string_literal | number_literal | boolean_literal ;
string_literal= '"' , { character } , '"' ;
variable_ref  = "${" , identifier , { "." , identifier } , "}" ;

list          = "[" , [ expression , { "," , expression } ] , "]" ;
map           = "{" , [ identifier , "=" , expression , { newline , identifier , "=" , expression } ] , "}" ;
```

## 4. Lexical Structure
-   **Comments**: `#` for single line, `//` for single line.
-   **Strings**: Double-quoted `"string"`. Multi-line strings using heredoc syntax `<<EOF ... EOF` are supported.
-   **Identifiers**: Alphanumeric, underscores, hyphens. Must start with a letter.
-   **Case Sensitivity**: Identifiers are case-sensitive.
-   **Whitespace**: Ignored, except for separation of tokens. No significant indentation.

## 5. Core Blocks
The language is built around these primary top-level blocks:
-   `agent`: Defines an autonomous actor.
-   `tool`: Defines an executable capability.
-   `event`: Defines a signal schema.
-   `route`: Defines logic for directing flow.
-   `memory`: Defines storage persistence.
-   `graph`: Defines the execution flow and wiring.
-   `var`: Defines input variables.
-   `module`: Instantiates a reusable A22 module.

## 6. Agent Block Specification
Agents are the primary nodes. They consume inputs (messages, events) and produce outputs (actions, responses).

```hcl
agent "researcher" {
  model       = "gpt-4-turbo"
  temperature = 0.2
  system_prompt = "You are a senior research analyst."

  # Tools this agent has access to
  tools = [
    tool.web_search,
    tool.summarizer
  ]

  # Memory scope
  memory = memory.short_term_context

  inputs {
    query = string
  }

  outputs {
    report = string
  }
}
```

## 7. Tool Block Specification
Tools are deterministic functions exposed to agents.

```hcl
tool "web_search" {
  description = "Search the web for information."
  
  # Implementation reference (e.g., path to TS/Python file or API endpoint)
  source = "./tools/search.ts" 

  inputs {
    query = string
    limit = number
  }

  outputs {
    results = list(string)
  }
}
```

## 8. Event & Route Specification
Events trigger execution. Routes decide where events go.

```hcl
event "user_message" {
  schema = {
    text = string
    user_id = string
  }
}

route "triage" {
  input = event.user_message

  # Logic block - simple conditional routing
  step "classify" {
    # Inline reasoning or simple logic
    condition = contains(input.text, "help")
    target    = agent.support_bot
  }

  default = agent.general_chat
}
```

## 9. Memory & State Specification
Memory defines how state is persisted and retrieved.

```hcl
memory "conversation_history" {
  type = "vector_store" # or "key_value", "ephemeral"
  ttl  = "1h"
  
  config = {
    provider = "pinecone"
    index    = "main-index"
  }
}
```

## 10. Graph & Execution Model (ADG)
The `graph` block explicitly wires components together if not implied by direct references. It defines the "main" entry point.

```hcl
graph "main" {
  start = route.triage

  # Explicit edges can be defined if needed for visualization or complex flows
  # edge route.triage -> agent.support_bot
  # edge route.triage -> agent.general_chat
}
```

**Execution Semantics**:
1.  **Trigger**: An external input triggers an `event`.
2.  **Flow**: The event flows through `routes` or directly to an `agent`.
3.  **Step**: An `agent` processes the input, potentially calling `tools`.
4.  **State**: `memory` is read/written during steps.
5.  **Output**: The graph produces a final output or emits a new `event`.

## 11. Variable System & Interpolation
Variables allow parameterization of configurations.

```hcl
var "api_key" {
  type    = string
  default = "env:OPENAI_API_KEY"
  sensitive = true
}

agent "writer" {
  # ...
  env = {
    API_KEY = "${var.api_key}"
  }
}
```

**Interpolation**: `${type.name.attribute}` syntax is used to reference values from other blocks.

## 12. Module System
Modules allow grouping resources into reusable components.

```hcl
module "customer_support" {
  source = "./modules/support-team"
  
  # Pass variables to module
  tier_level = "premium"
}
```

## 13. AST Design
The Abstract Syntax Tree (AST) should represent the configuration as a structured object graph.

**Node Types**:
-   `Program`: Root node.
-   `Block`: Generic block (agent, tool, etc.).
-   `Attribute`: Key-value pair.
-   `Expression`: Literal, Reference, Call.

**TypeScript Interface Sketch**:
```typescript
interface Program {
  blocks: Block[];
}

interface Block {
  type: string; // "agent", "tool"
  label?: string; // "researcher"
  attributes: Record<string, Expression>;
  children: Block[]; // Nested blocks
}
```

## 14. Validation & Static Checks
The compiler must enforce:
1.  **Reference Integrity**: All `${...}` references must point to existing blocks.
2.  **Type Checking**: Input/Output types between connected blocks must match.
3.  **Cycle Detection**: The execution graph must be acyclic (no infinite loops without explicit exit conditions).
4.  **Required Attributes**: Ensure mandatory fields (e.g., `model` for `agent`) are present.

## 15. Error Handling Rules
-   **Parse Errors**: Invalid syntax (line/column reported).
-   **Validation Errors**: Invalid references or types (reported before execution).
-   **Runtime Errors**: Tool failures or agent timeouts. Handled via `error_handler` blocks (optional on agents).

```hcl
agent "risky_agent" {
  # ...
  on_error {
    retry = 3
    fallback = agent.safe_mode
  }
}
```

## 16. Execution Semantics
The runtime executes the graph.
-   **Lazy Evaluation**: Blocks are instantiated only when reached.
-   **Concurrency**: Parallel branches in the graph execute concurrently.
-   **Idempotency**: Re-running a graph with the same state and input should yield the same result (assuming deterministic agents/tools).

## 17. Developer Experience Guidelines
-   **LSP**: Language Server Protocol implementation for auto-complete and go-to-definition.
-   **Formatter**: `a22 fmt` to standardize layout.
-   **Visualizer**: `a22 graph` to generate Mermaid or DOT diagrams.

## 18. Reserved Keywords
`agent`, `tool`, `event`, `route`, `memory`, `graph`, `var`, `module`, `true`, `false`, `null`, `if`, `else`, `for`, `in`.

## 19. Full Examples

### Small: Simple Q&A
```hcl
# main.a22
var "model_name" { default = "gpt-4o" }

agent "assistant" {
  model = "${var.model_name}"
  system_prompt = "You are a helpful assistant."
}

graph "main" {
  start = agent.assistant
}
```

### Medium: RAG System
```hcl
# rag.a22
var "docs_path" { default = "./docs" }

memory "docs_db" {
  type = "vector"
  config = { path = "${var.docs_path}" }
}

tool "retrieve" {
  source = "./tools/retrieve.ts"
  inputs { query = string }
  outputs { context = string }
}

agent "rag_bot" {
  model = "gpt-4-turbo"
  tools = [tool.retrieve]
  memory = memory.docs_db
  
  system_prompt = <<EOF
  Use the retrieve tool to find context before answering.
  EOF
}
```

### Large: Multi-Agent Research Team
```hcl
# research_team.a22

# --- Definitions ---

agent "planner" {
  model = "o1-preview"
  system_prompt = "Break down the user request into research tasks."
  outputs { tasks = list(string) }
}

agent "researcher" {
  model = "gpt-4-turbo"
  tools = [tool.web_search, tool.scrape]
  inputs { task = string }
  outputs { findings = string }
}

agent "editor" {
  model = "gpt-4-turbo"
  inputs { 
    original_request = string
    all_findings = list(string) 
  }
  system_prompt = "Compile findings into a final report."
}

tool "web_search" { source = "./std/search" }
tool "scrape" { source = "./std/scrape" }

# --- Orchestration ---

route "dispatch" {
  input = agent.planner.tasks
  
  # Fan-out pattern
  foreach "task" in input {
    target = agent.researcher
    args   = { task = task }
  }
  
  next = agent.editor
}

graph "research_flow" {
  start = agent.planner
  
  # Explicitly define the flow for clarity, though 'next' in route handles it
  # agent.planner -> route.dispatch -> agent.researcher (xN) -> agent.editor
}
```

## 20. Appendix: Extensions
Future support for:
-   **Remote State**: Storing memory/state in S3/GCS.
-   **Policy as Code**: Defining guardrails (e.g., "no agent can spend > $10").
