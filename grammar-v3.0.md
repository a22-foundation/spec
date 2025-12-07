# A22 Grammar v3.0

**Pure Event Ontology — Minimal, Natural Language**

**Status:** Draft
**Last Updated:** 2025-12-07

---

## Overview

This grammar is intentionally minimal and natural-language aligned.

A22 v3.0 has exactly **two constructs**: `system` and `agent`.
Everything else emerges from event relationships.

---

## Complete Grammar (EBNF)

```ebnf
Program          ::= (SystemDecl | AgentDecl | TemporalRule | NEWLINE)*

SystemDecl       ::= "system" String NEWLINE
                      INDENT SystemBody DEDENT

SystemBody       ::= (Description | Constraint | TemporalRule | NEWLINE)*

AgentDecl        ::= "agent" String NEWLINE
                      INDENT AgentBody DEDENT

AgentBody        ::= (Description
                    | Capability
                    | ForbiddenCapability
                    | Listens
                    | Notices
                    | Speaks
                    | LooksAtHistory
                    | Interprets
                    | Constraint
                    | NEWLINE)*

Capability       ::= "knows how to" CapabilityList

ForbiddenCapability ::= "is not allowed to" CapabilityList

CapabilityList   ::= CapabilityName ("," CapabilityName)*

CapabilityName   ::= Identifier

Listens          ::= "listens to" EventRef

Notices          ::= "notices" EventPattern

Speaks           ::= "speaks" EventRef

LooksAtHistory   ::= "looks at" EventPattern "history"

Interprets       ::= "interprets" Identifier "from" EventPattern "history"

TemporalRule     ::= "when" EventRef NEWLINE
                      INDENT "the system should eventually" EventRef DEDENT

Constraint       ::= ("holds" | "ensures" | "expects") Expression

Description      ::= ("description:" | "purpose:" | "goal:") String

EventPattern     ::= EventRef
                    | EventRef ".*"

EventRef         ::= Identifier ("." Identifier)*

Identifier       ::= Lowercase (Lowercase | "_")*

String           ::= "\"" Characters "\""

Characters       ::= AnyCharacterExceptQuote

Expression       ::= FreeFormTextUntilNewline

INDENT           ::= <indentation token>
DEDENT           ::= <dedentation token>
NEWLINE          ::= "\n"
Lowercase        ::= "a".."z"
```

---

## Lexical Elements

### Keywords

```
system
agent
knows how to
is not allowed to
listens to
notices
speaks
looks at
history
interprets
from
when
the system should eventually
holds
ensures
expects
description:
purpose:
goal:
```

### Symbols

```
"       # String delimiter
.       # Namespace separator
.*      # Wildcard pattern
,       # List separator
:       # Property assignment
```

### Identifiers

```
Lowercase letters and underscores
Examples: chat, message_received, user, retrieval_done
```

### Event References

```
namespace.name          # Specific event
namespace.action        # Specific event
namespace.*             # Event pattern (wildcard)
```

### Strings

```
"text content"
```

---

## Indentation Rules

A22 uses **Python/YAML-style indentation**:

- **Tabs or 4 spaces** (consistent within a file)
- Indentation denotes scope
- Dedentation closes scope

**Example:**

```a22
system "example"
    description: "system level"

agent "worker"
    knows how to chat
    listens to message.received
```

---

## Productions

### System Declaration

```ebnf
SystemDecl ::= "system" String NEWLINE INDENT SystemBody DEDENT
```

**Example:**

```a22
system "chat_system"
    description: "conversational system"
    ensures "all messages receive replies"
```

### Agent Declaration

```ebnf
AgentDecl ::= "agent" String NEWLINE INDENT AgentBody DEDENT
```

**Example:**

```a22
agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
```

### Capabilities

```ebnf
Capability ::= "knows how to" CapabilityList
```

**Example:**

```a22
knows how to chat, summarize, analyze
```

### Forbidden Capabilities

```ebnf
ForbiddenCapability ::= "is not allowed to" CapabilityList
```

**Example:**

```a22
is not allowed to execute_code, access_secrets
```

### Event Subscriptions

```ebnf
Listens ::= "listens to" EventRef
Notices ::= "notices" EventPattern
Speaks  ::= "speaks" EventRef
```

**Examples:**

```a22
listens to message.received
notices user.*
speaks message.reply
```

### History References

```ebnf
LooksAtHistory ::= "looks at" EventPattern "history"
```

**Example:**

```a22
looks at message.* history
looks at conversation history
```

### Interpretations

```ebnf
Interprets ::= "interprets" Identifier "from" EventPattern "history"
```

**Examples:**

```a22
interprets intent from message.* history
interprets sentiment from conversation history
interprets risk from draft.reply history
```

### Temporal Guarantees

```ebnf
TemporalRule ::= "when" EventRef NEWLINE
                  INDENT "the system should eventually" EventRef DEDENT
```

**Example:**

```a22
when message.received
    the system should eventually speak support.complete
```

### Constraints

```ebnf
Constraint ::= ("holds" | "ensures" | "expects") Expression
```

**Examples:**

```a22
holds "response time < 5s"
ensures "all messages have replies"
expects "no code execution"
```

### Descriptions

```ebnf
Description ::= ("description:" | "purpose:" | "goal:") String
```

**Examples:**

```a22
description: "conversational agent"
purpose: "answer user questions"
goal: "provide helpful responses"
```

---

## Event Patterns

Event patterns describe sets of events:

### Specific Event

```a22
message.received
user.login
task.completed
```

### Wildcard Pattern

```a22
message.*         # All message events
user.*            # All user events
task.*            # All task events
```

### Usage

Patterns can be used in:

1. **Event subscriptions:**
   ```a22
   notices user.*
   ```

2. **History references:**
   ```a22
   looks at message.* history
   ```

3. **Interpretations:**
   ```a22
   interprets intent from message.* history
   ```

---

## Complete Examples

### Example 1: Minimal Agent

```a22
agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
```

### Example 2: Agent with Interpretation

```a22
agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply
    looks at message.* history
    interprets intent from message.* history
```

### Example 3: System with Guarantee

```a22
system "support_system"
    description: "customer support system"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks message.reply

when message.received
    the system should eventually speak support.complete
```

### Example 4: Multi-Agent Chain

```a22
system "rag_system"
    description: "retrieval-augmented generation"

agent "assistant"
    knows how to answer
    listens to message.received
    speaks retrieval.request

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

### Example 5: Safety Guard

```a22
system "safe_chat"
    ensures "all outputs are reviewed"

agent "assistant"
    knows how to chat
    listens to message.received
    speaks draft.reply

agent "safety_guard"
    knows how to review
    listens to draft.reply
    speaks safe.reply
    interprets risk from draft.reply history
```

---

## Parsing Notes

### Whitespace

- Indentation is significant
- Blank lines are ignored
- Comments are not defined in v3.0 (future consideration)

### Case Sensitivity

- Keywords are **case-sensitive** and **lowercase**
- Identifiers are **case-sensitive**
- Strings preserve case

### String Escaping

Strings use simple quote escaping:

```a22
description: "she said \"hello\""
```

### Multi-line Strings

Not supported in v3.0. Use single-line strings.

---

## Grammar Validation

Valid A22 v3.0 programs must:

1. Start with either `system` or `agent` declarations
2. Use consistent indentation (tabs or 4 spaces)
3. Reference valid event patterns (`namespace.name` or `namespace.*`)
4. Use only defined keywords

Invalid:

```a22
# Missing string quotes
system example

# Invalid event reference (no namespace)
listens to message

# Mixed indentation
system "example"
	description: "mixed"
    ensures: "bad"
```

Valid:

```a22
system "example"
    description: "valid system"

agent "worker"
    knows how to chat
    listens to message.received
```

---

## Appendix: Keyword Index

### Structural

- `system`
- `agent`

### Capabilities

- `knows how to`
- `is not allowed to`

### Event Relations

- `listens to`
- `notices`
- `speaks`
- `looks at ... history`
- `interprets ... from ... history`

### Temporal

- `when`
- `the system should eventually`

### Constraints

- `holds`
- `ensures`
- `expects`

### Descriptions

- `description:`
- `purpose:`
- `goal:`

---

**A22 Grammar v3.0** — Complete
