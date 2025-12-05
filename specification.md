# A22 Specification v0.1 (Foundational Draft)

**Status:** Draft
**License:** Apache 2.0
**Audience:** Runtime implementers, language tooling developers, contributors
**Goal:** Define the syntax, semantics, structures, and expectations of the A22 declarative agent language.

---

## 0. Design Principles

A22 is built around the following principles:

1.  **Declarative First**
    A22 describes *what* the agentic system is, not *how* it executes.
2.  **Composable Blocks**
    Everything is defined through reusable blocks: agents, tools, data, events, workflows.
3.  **Capability-Oriented**
    Agents declare capabilities, and runtimes supply implementations.
4.  **Stable, Minimal, Predictable**
    Small surface area, predictable evolution, conservative changes.
5.  **Portable and Vendor-Neutral**
    Any runtime should be able to execute a compliant A22 program.

---

## 1. File Structure

### 1.1 A22 Files
*   The default extension is `.a22`.
*   A22 files contain a sequence of declarations, each defining one block.

### 1.2 Valid Blocks

A22 v1.0 defines the following blocks:
*   `agent`
*   `capability`
*   `tool`
*   `event`
*   `workflow`
*   `data` (schema definition)
*   `config` (optional metadata)

Blocks can appear in any order.

---

## 2. Syntax Overview

A22 uses a Terraform-style declarative syntax with:
*   Double-quoted identifiers for named resources
*   Curly-brace block structure
*   HCL-like expressions (strings, booleans, numbers, arrays, objects)

### 2.1 Example

```a22
agent "support_bot" {
    capabilities = ["memory", "retrieval"]

    on event "user_message" {
        use tool "embedder"
        call workflow "answer_user"
    }
}
```

---

## 3. Core Blocks

---

### 3.1 AGENT Block

Defines an autonomous or semi-autonomous agent.

**Syntax**

```a22
agent "<name>" {
    capabilities = [ ... ]
    state        = data.<schema>?   // optional
    model        = "<model-id>"?    // optional (runtime-dependent)

    on event "<event-name>" {
        call workflow "<workflow-name>"?
        use tool "<tool-name>"?
    }
}
```

**Fields**

| Field | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `capabilities` | `array(string)` | Yes | Declares required capabilities |
| `state` | `reference` | No | Data schema attached to agent memory |
| `model` | `string` | No | Preferred model; runtime may override |
| `on event` | `block` | No | Event handler |

**Semantics**
*   Agents do nothing unless triggered by events or workflows.
*   Capabilities must be supplied by runtime or bound at execution time.
*   Events are resolved by name and passed to handlers.

---

### 3.2 CAPABILITY Block

Defines an abstract capability an agent may require.

**Syntax**

```a22
capability "retrieval" {
    inputs  = ["query"]
    outputs = ["docs"]
    kind    = "external" | "system" | "builtin"
}
```

Capabilities are contracts only; implementations are runtime-specific.

---

### 3.3 TOOL Block

Represents a callable function, API, or external executor.

**Syntax**

```a22
tool "<name>" {
    schema {
        field1: string
        field2: number
        field3: array<string>
    }
    handler = external("<binding>")
}
```

**Semantics**
*   Tools are side-effecting operations.
*   Runtime determines actual function binding (cloud, local, API, etc).

---

### 3.4 EVENT Block

Defines an event that can be emitted or listened for.

**Syntax**

```a22
event "user_message" {
    payload = data.UserMessage
}
```

---

### 3.5 WORKFLOW Block

Workflows orchestrate steps, tools, and agent calls.

**Syntax**

```a22
workflow "answer_user" {
    steps {
        embed  = tool "embedder" { text = input.text }
        search = capability "retrieval" { query = embed.vector }
        reply  = agent "support_bot" { context = search.docs }
    }
    returns = data.Answer
}
```

**Execution Semantics**
*   Steps run in order unless `parallel` keyword is used.
*   Steps reference tools, agents, or capabilities.
*   Workflows can emit events.

---

### 3.6 DATA Block

Defines structured data schemas.

**Syntax**

```a22
data Answer {
    text: string
    confidence: number
}
```

Supported primitive types:
*   `string`
*   `number`
*   `boolean`
*   `array<T>`
*   `object { ... }`

---

## 4. Expressions & Values

A22 supports:
*   Strings: `"hello"`
*   Numbers: `42`, `3.14`
*   Booleans: `true`, `false`
*   Arrays: `[1, 2, 3]`
*   Objects:

```a22
{
    a = 1
    b = true
}
```

No custom functions or expressions allowed v1.0 — keeps language pure & deterministic.

---

## 5. Execution Model

### 5.1 Runtime Responsibilities

A runtime MUST:
1.  Parse & validate A22 files
2.  Resolve capabilities
3.  Bind tools to handlers
4.  Execute workflows in deterministic order
5.  Maintain agent state if defined
6.  Enforce sandboxing & security boundaries
7.  Support event dispatch & routing

### 5.2 Determinism
*   Workflow step ordering is deterministic.
*   Tools may be nondeterministic; A22 does not impose constraints.

### 5.3 Error Handling

Runtimes must:
*   Surface schema validation errors
*   Surface missing capability binding errors
*   Provide structured error messages

---

## 6. AST & IR Requirements

A22 languages must translate to a canonical IR containing:
*   Block types + names
*   Field bodies
*   Data schemas
*   Workflow step graph
*   Event routing table
*   Capability contract requirements

Runtimes must accept this IR structure.

AST structure is implementation-specific, but must be lossy of no data.

---

## 7. Validation Rules

A22 v1.0 requires the following validation:
*   Agents must reference valid events + workflows
*   Tools must reference valid schemas
*   Data schemas must be acyclic
*   Workflow steps must reference defined entities
*   Capabilities must match invocation shapes
*   No duplicate block names of same type
*   No undeclared identifiers

---

## 8. Versioning

A22 follows semantic versioning:
*   `0.x` = not yet stable
*   `1.x` = stable language
*   `2.x` = potential breaking syntax changes

Backward compatibility is required unless otherwise stated.

---

## 9. Reserved Keywords (v1.0)

*   `agent`
*   `tool`
*   `capability`
*   `workflow`
*   `event`
*   `data`
*   `config`
*   `steps`
*   `schema`
*   `handler`
*   `external`
*   `on`
*   `parallel`
*   `returns`
*   `input`

---

## 10. Security & Isolation

A22 itself is declarative; runtimes must enforce:
*   capability-level isolation
*   tool sandboxing
*   per-agent memory boundaries
*   event permission rules

A22 does not define execution locality (cloud, local, edge).

---

## 11. Minimal Example

```a22
data Question {
    text: string
}

data Answer {
    text: string
}

event "user_question" {
    payload = data.Question
}

tool "generate_answer" {
    schema {
        prompt: string
    }
    handler = external("model.generate")
}

workflow "qa" {
    steps {
        answer = tool "generate_answer" { prompt = input.text }
    }
    returns = data.Answer
}

agent "qa_bot" {
    capabilities = []
    on event "user_question" {
        call workflow "qa"
    }
}
```
