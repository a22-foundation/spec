# A22 Specification v3.0

**Pure Event Ontology**

**Status:** Draft
**License:** Apache 2.0
**Last Updated:** 2025-12-07

---

## Abstract

A22 is a declarative, second-order, natural-language DSL for describing agentic systems entirely as **events unfolding in time**.

Agents perceive events, interpret histories, and produce new events.
Contexts are implicit event histories.
Projections are interpretations of those histories.
Workflows are temporal guarantees.
Tools are deterministic event transforms.

Nothing needs explicit definition except agents and system-level constraints.

---

## Table of Contents

1. [Language Philosophy](#1-language-philosophy)
2. [Guiding Principles](#2-guiding-principles)
3. [Entity Model](#3-entity-model)
4. [Keyword Model](#4-keyword-model)
5. [Event Model](#5-event-model)
6. [Agents](#6-agents)
7. [Systems](#7-systems)
8. [Temporal Guarantees](#8-temporal-guarantees)
9. [Constraints](#9-constraints)
10. [Runtime Interpretation](#10-runtime-interpretation)
11. [Examples](#11-examples)

---

## 1. Language Philosophy

### 1.1 Describe behavior across time, not steps

A22 expresses how entities relate through events — not how they run.

### 1.2 Everything begins and ends with events

Events are the atomic ontology of A22.
All meaning is temporal.

### 1.3 Natural language, not configuration

A22 reads more like English than a programming language.

### 1.4 Second-order logic, not procedures

The language describes classes, relationships, and guarantees — not imperative sequences.

### 1.5 Agents reason from histories

Agents look at accumulated event streams and interpret them into meaningful structures.

### 1.6 Minimal vocabulary

A small, domain-native set of keywords keeps the language human.

### 1.7 Runtimes interpret, not execute

Runtimes build a semantic graph and respond to event flow — nothing is procedural.

---

## 2. Guiding Principles

### 2.1 Intent over mechanism

A22 describes what a system is and means, not how it runs.

### 2.2 Everything is temporal

The unit of truth is an event happening in time.

### 2.3 Accumulation over mutation

History grows; it is never overwritten.

### 2.4 Relationships define behavior

Agents act because of relations — not instructions.

### 2.5 Interpretations are first-class

Meaning (extractions, inferences, intent) is explicit.

### 2.6 Constraints shape systems

A22 expresses what must always hold as events unfold.

### 2.7 Systems emerge, not execute

Behavior emerges from event flows defined in the spec.

---

## 3. Entity Model

In v3.0, only **two entity types** require explicit definition:

```a22
system "..."
agent "..."
```

All other constructs appear implicitly as event relationships.

---

## 4. Keyword Model

### 4.1 Agent Capabilities

```a22
knows how to <capability>
is not allowed to <capability>
```

**Examples:**
```a22
knows how to chat, summarize, retrieve
is not allowed to execute_code, access_secrets
```

### 4.2 Event Relations

```a22
listens to <event>
notices <event>
speaks <event>
```

**Examples:**
```a22
listens to message.received
notices user.*
speaks message.reply
```

### 4.3 Event History Interpretation

```a22
looks at <event_pattern> history
interprets <field> from <event_pattern> history
```

**Examples:**
```a22
looks at message.* history
interprets intent from message.* history
interprets sentiment from conversation history
```

### 4.4 Constraints and Guarantees

```a22
when <event>
    the system should eventually <event>

holds <property>
ensures <condition>
expects <condition>
```

**Examples:**
```a22
when message.received
    the system should eventually speak support.complete

holds "response time < 5s"
ensures "all messages have replies"
```

### 4.5 Descriptions

```a22
description: "text"
purpose: "text"
goal: "text"
```

---

## 5. Event Model

### 5.1 Events Are Implicit

Events do not need to be declared.

Any identifier of the form:
- `namespace.name`
- `namespace.action`
- `namespace.*` (pattern)

is valid.

**Examples:**
```a22
message.received
message.reply
user.login
retrieval.request
retrieval.done
task.completed
```

### 5.2 Event Patterns

Patterns describe sets of events in time:

```a22
message.*           # all message events
conversation.*      # every event in conversation domain
user.*              # all user events
```

Patterns apply to:
- History interpretation
- Constraints
- Temporal rules

### 5.3 Event Structure (Runtime)

While events are implicit in the language, runtimes represent them as:

```
event = {
  type: qualified_name,
  time: timestamp_utc,
  source: agent_id,
  correlation_id: uuid,
  data: map
}
```

---

## 6. Agents

Agents perceive event streams and produce events.

### 6.1 Basic Agent

```a22
agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
    looks at message.* history
```

### 6.2 Multi-Capability Agent

```a22
agent "assistant"
    knows how to chat, summarize, analyze
    is not allowed to execute_code
    listens to message.received
    speaks message.reply
    looks at message.* history
    interprets intent from message.* history
```

### 6.3 Agent with Multiple Event Sources

```a22
agent "orchestrator"
    knows how to coordinate
    listens to task.created
    listens to task.completed
    speaks workflow.done
    looks at task.* history
```

### 6.4 Observer Agent

An agent that notices events but doesn't directly respond:

```a22
agent "logger"
    knows how to log
    notices message.*
    speaks log.entry
```

**Key difference:**
- `listens to` — agent processes and responds
- `notices` — agent observes for side effects (logging, monitoring)

---

## 7. Systems

Systems declare top-level constraints and temporal guarantees.

### 7.1 Basic System

```a22
system "chat_system"
    description: "simple conversational system"
```

### 7.2 System with Guarantees

```a22
system "support_system"
    description: "customer support with response guarantees"
    ensures "all messages receive replies within 5 minutes"

when message.received
    the system should eventually speak support.complete
```

### 7.3 System with Multiple Agents

```a22
system "multi_agent_rag"
    description: "retrieval-augmented generation system"

    holds "knowledge retrieval happens before answer generation"
```

---

## 8. Temporal Guarantees

Temporal rules express **liveness** and **safety** properties without procedural flow.

### 8.1 Eventually Guarantees (Liveness)

```a22
when message.received
    the system should eventually speak message.reply
```

### 8.2 Ordering Guarantees

```a22
when retrieval.request
    the system should eventually speak retrieval.done

when retrieval.done
    the system should eventually speak answer.generated
```

### 8.3 Compound Guarantees

```a22
when transaction.started
    the system should eventually speak transaction.processed

when transaction.processed
    the system should eventually speak transaction.logged
```

---

## 9. Constraints

Constraints define invariants that must hold across event flows.

### 9.1 Safety Properties

```a22
holds "no sensitive data in logs"
ensures "all user requests are authenticated"
expects "response time < 5s"
```

### 9.2 System-Level Constraints

```a22
system "safe_chat"
    ensures "all outputs are reviewed for safety"
    holds "no code execution allowed"
```

---

## 10. Runtime Interpretation

### 10.1 Semantic Graph Construction

The runtime builds an event-driven semantic graph:

**Nodes:**
- Agents
- System
- Implicit events
- Implicit histories
- Inferred projections

**Edges:**
- `listens_to`
- `speaks`
- `interprets`
- `temporal_guarantees`

### 10.2 History Construction

A history is the ordered stream of events matching a pattern:

```a22
<event_pattern> history
```

The runtime collects:
- All matching events
- In temporal order
- With no mutation (append-only)

### 10.3 Interpretation

For each interpretation rule:

```a22
interprets intent from message.* history
```

The runtime computes a derived field and makes it available for agent invocation.

### 10.4 Agent Invocation

An agent is invoked when:
1. A subscribed event occurs (`listens to`)
2. The agent's required histories and interpretations are available
3. System constraints allow it

Outputs become new events.

### 10.5 Temporal Guarantee Evaluation

Rules like:

```a22
when message.received
    the system should eventually speak support.complete
```

Yield:
- Liveness checks
- Expectation monitors
- Temporal validation

### 10.6 Emergent Behavior

The runtime orchestrates nothing.
All behavior emerges from event relations and temporal logic.

---

## 11. Examples

### Example 1: Simple Chat Agent

```a22
system "chat_system"
    description: "single agent that replies to messages"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
    looks at message.* history
```

### Example 2: Intent Interpretation

```a22
system "intent_system"
    description: "agent interprets user intent and replies"

agent "assistant"
    knows how to chat, summarize
    listens to message.received
    speaks message.reply
    looks at message.* history
    interprets intent from message.* history
```

### Example 3: Two-Agent Retrieval Chain

```a22
system "retrieval_conversation"
    description: "assistant uses a retriever through events"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks retrieval.request
    looks at message.* history
    interprets intent from message.* history

agent "retriever"
    knows how to retrieve
    listens to retrieval.request
    speaks retrieval.done

agent "assistant"
    listens to retrieval.done
    speaks message.reply
```

### Example 4: Support Guarantee

```a22
system "support"
    description: "conversation must eventually complete"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply

when message.received
    the system should eventually speak support.complete
```

### Example 5: Sentiment and Intent Analysis

```a22
system "analysis_system"
    description: "agent interprets sentiment and user intent"

agent "nlp_agent"
    knows how to analyze
    listens to message.received
    speaks analysis.generated
    looks at message.* history
    interprets intent from message.* history
    interprets sentiment from message.* history

when message.received
    the system should eventually speak analysis.generated
```

### Example 6: Multi-Agent Orchestration

```a22
system "multi_agent"
    description: "assistant, planner, and executor coordinate via events"

agent "assistant"
    knows how to understand
    listens to message.received
    speaks plan.request
    looks at message.* history
    interprets intent from message.* history

agent "planner"
    knows how to generate_plans
    listens to plan.request
    speaks plan.created
    interprets tasks from plan.request history

agent "executor"
    knows how to perform_tasks
    listens to plan.created
    speaks task.completed

when message.received
    the system should eventually speak task.completed
```

### Example 7: RAG System

```a22
system "rag_system"
    description: "RAG using event-based structures"

agent "assistant"
    knows how to answer
    listens to message.received
    speaks retrieval.request
    looks at message.* history

agent "retriever"
    knows how to retrieve
    listens to retrieval.request
    speaks retrieval.done

agent "assistant"
    listens to retrieval.done
    speaks answer.generated

when message.received
    the system should eventually speak answer.generated
```

### Example 8: Safety Guard

```a22
system "safety_system"
    description: "safety agent verifies outputs before sending"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks draft.reply

agent "safety_agent"
    knows how to review
    listens to draft.reply
    speaks safe.reply
    interprets risk from draft.reply history

when message.received
    the system should eventually speak safe.reply
```

### Example 9: Monitoring & Logging

```a22
system "monitoring_system"
    description: "observer agent logs all message events"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply

agent "logger"
    knows how to log
    notices message.*
    speaks log.entry
```

### Example 10: Multi-Step Transaction

```a22
system "transaction_system"
    description: "every transaction should be acknowledged and recorded"

agent "processor"
    knows how to handle_transactions
    listens to transaction.started
    speaks transaction.processed

agent "auditor"
    knows how to audit
    listens to transaction.processed
    speaks transaction.logged

when transaction.started
    the system should eventually speak transaction.logged
```

---

## Appendices

### A. Comparison with v1.0

**v1.0:**
- Explicit workflows, tools, context blocks
- Imperative-style step definitions
- Complex type system
- Heavy on configuration

**v3.0:**
- Pure event ontology
- Natural language relationships
- Minimal keywords
- Systems emerge from event flows

### B. Migration from v1.0 to v3.0

v1.0 workflows become temporal guarantees.
v1.0 tools become agents with deterministic transforms.
v1.0 projections become interpretations.
v1.0 context becomes implicit event history.

### C. Formal Semantics

See whitepaper for complete formal foundations including:
- Event algebra
- Temporal logic
- Liveness and safety properties
- Determinism proofs

---

**A22 v3.0** — Specification Complete
