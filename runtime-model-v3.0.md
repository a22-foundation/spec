# A22 Runtime Interpretation Model v3.0

**Pure Event Ontology**

**Status:** Draft
**Last Updated:** 2025-12-07

---

## Overview

This document describes how A22 v3.0 programs are interpreted by runtimes.

A22 runtimes **do not execute**. They **interpret**.

The runtime builds a semantic graph of event relationships and responds to event flow — nothing is procedural.

---

## Runtime Architecture

### 1. Build an Event-Driven Semantic Graph

**Phase 1: Parse the A22 specification**

The runtime reads the A22 program and constructs a graph.

**Nodes:**
- `system` declarations
- `agent` declarations
- Implicit events (referenced but not declared)
- Implicit histories (event pattern + `history`)
- Inferred interpretations (from `interprets` clauses)

**Edges:**
- `listens_to` (agent → event)
- `notices` (agent → event pattern)
- `speaks` (agent → event)
- `looks_at` (agent → history)
- `interprets` (agent → derived field)
- `temporal_guarantee` (event → event)

---

### 2. History Construction

A **history** is the ordered, append-only stream of events matching a pattern.

**Syntax:**

```a22
looks at <event_pattern> history
```

**Runtime Interpretation:**

The runtime maintains:
- All events matching the pattern
- In temporal order (by event timestamp)
- Append-only (no mutation or deletion)

**Example:**

```a22
looks at message.* history
```

Runtime maintains:
```
[
  { type: "message.received", time: T1, data: {...} },
  { type: "message.reply", time: T2, data: {...} },
  { type: "message.received", time: T3, data: {...} }
]
```

---

### 3. Interpretation

**Syntax:**

```a22
interprets <field> from <event_pattern> history
```

**Runtime Interpretation:**

The runtime computes a **derived field** from the history and makes it available to the agent.

**Example:**

```a22
interprets intent from message.* history
```

Runtime:
1. Retrieves `message.*` history
2. Applies interpretation logic (model-based, rule-based, etc.)
3. Stores result as `intent` field accessible to agent

**Implementation Note:**

The interpretation mechanism is runtime-specific. It may use:
- LLM-based extraction
- Rule-based parsing
- Statistical models
- External tools

The specification does not prescribe **how** interpretation happens, only **that** it happens.

---

### 4. Agent Invocation

An agent is invoked when **all** of the following are true:

1. **A subscribed event occurs** (`listens to`)
2. **Required histories are available** (`looks at`)
3. **Required interpretations are complete** (`interprets`)
4. **System constraints allow invocation** (`ensures`, `holds`)

**Invocation Process:**

```
1. Event E arrives
2. Runtime checks: which agents listen to E?
3. For each agent A:
   a. Are all required histories available? → Retrieve them
   b. Are all required interpretations complete? → Compute them
   c. Do system constraints allow invocation? → Check
4. If all checks pass:
   a. Invoke agent A with event E + histories + interpretations
   b. Agent produces new event E'
   c. Emit E' to the event stream
```

**Example:**

```a22
agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
    looks at message.* history
    interprets intent from message.* history
```

**Runtime:**

1. `message.received` event arrives
2. Runtime retrieves `message.*` history
3. Runtime computes `intent` interpretation
4. Runtime invokes agent with:
   - Event: `message.received`
   - History: `message.*`
   - Interpretation: `intent`
5. Agent produces `message.reply` event
6. Runtime emits `message.reply` to event stream

---

### 5. Temporal Guarantee Evaluation

**Syntax:**

```a22
when <event_E1>
    the system should eventually <event_E2>
```

**Runtime Interpretation:**

The runtime monitors event flow and validates temporal properties.

**Liveness Check:**

For each occurrence of `E1`, the runtime expects `E2` to eventually occur.

**Example:**

```a22
when message.received
    the system should eventually speak support.complete
```

**Runtime:**

1. Track every `message.received` event
2. For each, start a liveness monitor
3. Monitor waits for `support.complete` event
4. If timeout expires without `support.complete`, raise violation
5. If `support.complete` occurs, mark guarantee satisfied

**Timeout Configuration:**

The runtime may define default or configurable timeouts for "eventually".

---

### 6. Constraint Evaluation

**Syntax:**

```a22
holds "<property>"
ensures "<condition>"
expects "<condition>"
```

**Runtime Interpretation:**

Constraints are evaluated continuously against the event stream.

**Example:**

```a22
ensures "all messages have replies"
```

**Runtime:**

1. For every `message.received` event
2. Check if a corresponding `message.reply` exists
3. If not, raise constraint violation

**Example:**

```a22
holds "response time < 5s"
```

**Runtime:**

1. For every agent invocation
2. Measure elapsed time until event is produced
3. If > 5s, raise constraint violation

---

### 7. Emergent Behavior

The runtime **does not orchestrate**.

All behavior emerges from:
- Event subscriptions (`listens to`, `notices`)
- Agent responses (`speaks`)
- Temporal guarantees (`when ... should eventually`)
- Constraints (`holds`, `ensures`, `expects`)

**Example:**

```a22
agent "assistant"
    listens to message.received
    speaks retrieval.request

agent "retriever"
    listens to retrieval.request
    speaks retrieval.done

agent "assistant"
    listens to retrieval.done
    speaks answer.generated
```

**Runtime Behavior:**

1. `message.received` → agent "assistant" invoked → emits `retrieval.request`
2. `retrieval.request` → agent "retriever" invoked → emits `retrieval.done`
3. `retrieval.done` → agent "assistant" invoked → emits `answer.generated`

**No explicit orchestration.** The chain emerges from event subscriptions.

---

## Runtime Event Model

While A22 does not require events to be declared, runtimes represent them with this structure:

```json
{
  "type": "message.received",
  "time": "2025-12-07T10:30:00Z",
  "source": "user_123",
  "correlation_id": "uuid-123",
  "data": {
    "content": "Hello, assistant!",
    "user_id": "user_123"
  }
}
```

**Fields:**

- `type` — Fully qualified event name (`namespace.action`)
- `time` — UTC timestamp (ISO 8601)
- `source` — Agent or external source ID
- `correlation_id` — UUID linking related events
- `data` — Event-specific payload (map)

---

## Runtime Guarantees

A22 runtimes must provide:

### 1. Determinism

Same events + same histories → same agent outputs.

### 2. Auditability

All events are logged with full trace information.

### 3. Immutability

Histories are append-only. Events never mutate.

### 4. Temporal Correctness

Temporal guarantees (`when ... eventually`) are monitored and violations are reported.

### 5. Constraint Enforcement

All `holds`, `ensures`, `expects` clauses are validated continuously.

---

## Runtime Implementation Notes

### Event Storage

Runtimes may use:
- In-memory streams (testing, development)
- Event logs (Kafka, Pulsar, etc.)
- Databases (Postgres, DynamoDB, etc.)
- Event stores (EventStoreDB, etc.)

### History Queries

Runtimes must support efficient pattern-based history retrieval:

```
GET /events?pattern=message.*&order=time_asc
```

### Interpretation Engines

Runtimes may use:
- LLM APIs (OpenAI, Anthropic, etc.)
- Local models (Ollama, etc.)
- Rule engines
- Custom logic

### Temporal Monitoring

Runtimes should implement:
- Liveness monitors (for `eventually` guarantees)
- Timeout configuration
- Violation alerts/logging

---

## Runtime Lifecycle

### 1. Initialization

```
1. Parse A22 specification
2. Build semantic graph
3. Initialize event storage
4. Start event listeners
```

### 2. Event Processing Loop

```
loop:
  1. Receive event E
  2. Store E in event log
  3. Update relevant histories
  4. Find agents subscribed to E
  5. For each agent:
     a. Retrieve required histories
     b. Compute required interpretations
     c. Invoke agent with E + histories + interpretations
     d. Agent produces new event E'
     e. Emit E' to event stream
  6. Evaluate temporal guarantees
  7. Evaluate constraints
```

### 3. Shutdown

```
1. Flush pending events
2. Close event storage
3. Report unsatisfied guarantees
```

---

## Example Runtime Trace

Given this A22 program:

```a22
system "chat"
    description: "simple chat system"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
    looks at message.* history
```

**Runtime Trace:**

```
[T0] Runtime starts
[T0] Parse specification
[T0] Build semantic graph:
     - Agent: "assistant"
     - Subscribes to: "message.received"
     - Produces: "message.reply"
     - Requires history: "message.*"

[T1] Event arrives: message.received
     { type: "message.received", data: { content: "Hello" } }

[T1] Store event in log

[T1] Find subscribed agents:
     - Agent "assistant" subscribes to "message.received"

[T1] Retrieve required histories:
     - message.* history = [{ type: "message.received", ... }]

[T1] Invoke agent "assistant":
     - Input: message.received event
     - History: message.* (1 event)
     - No interpretations required

[T2] Agent produces event: message.reply
     { type: "message.reply", data: { content: "Hi there!" } }

[T2] Emit event to stream

[T2] Store message.reply in log

[T2] No agents subscribe to message.reply → done
```

---

## Comparison with Traditional Runtimes

| Traditional Runtime | A22 Runtime |
|-------------------|-------------|
| Executes procedures | Interprets relationships |
| Mutable state | Immutable event log |
| Call stacks | Event streams |
| Control flow | Temporal guarantees |
| Variables | Histories |
| Functions | Agents |
| Loops/conditionals | Event subscriptions |

---

**A22 Runtime Model v3.0** — Complete
