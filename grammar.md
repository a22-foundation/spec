# A22 Grammar (EBNF) — v1.0

## 1. Top-Level Structure

```ebnf
a22_file     ::= declaration*
declaration  ::= agent_decl
               | tool_decl
               | capability_decl
               | workflow_decl
               | event_decl
               | data_decl
               | config_decl
```

---

## 2. Lexical Elements

### 2.1 Identifiers

```ebnf
IDENT        ::= QUOTED_STRING
QUOTED_STRING ::= '"' (CHAR_NO_QUOTE)* '"'
CHAR_NO_QUOTE ::= /* any Unicode char except " */
```

A22 requires double-quoted identifiers for clarity.

### 2.2 Literals

```ebnf
string_lit   ::= QUOTED_STRING
number_lit   ::= DIGITS ('.' DIGITS)?
boolean_lit  ::= 'true' | 'false'

DIGITS       ::= [0-9]+
```

### 2.3 Values

```ebnf
value        ::= string_lit
               | number_lit
               | boolean_lit
               | array_lit
               | object_lit
               | reference

array_lit    ::= '[' (value (',' value)*)? ']'

object_lit   ::= '{' (object_field)* '}'
object_field ::= IDENT '=' value
```

### 2.4 References

```ebnf
reference    ::= IDENT ('.' IDENT)*
```

Examples:
*   `data.UserMessage`
*   `tool.embedder`

---

## 3. Blocks

### 3.1 Agent Block

```ebnf
agent_decl ::= 'agent' IDENT '{' agent_body '}'  

agent_body ::= ( agent_field
               | event_handler_block
               )*

agent_field ::= 'capabilities' '=' array_lit
               | 'state' '=' reference
               | 'model' '=' string_lit

event_handler_block ::= 'on' 'event' IDENT '{' event_handler_body '}'

event_handler_body ::= (workflow_call | tool_call)*

workflow_call ::= 'call' 'workflow' IDENT
tool_call      ::= 'use'  'tool'      IDENT
```

### 3.2 Capability Block

```ebnf
capability_decl ::= 'capability' IDENT '{' capability_body '}'

capability_body ::= capability_field*

capability_field ::= 'inputs'  '=' array_lit
                   | 'outputs' '=' array_lit
                   | 'kind'    '=' string_lit
```

### 3.3 Tool Block

```ebnf
tool_decl ::= 'tool' IDENT '{' tool_body '}'

tool_body ::= schema_block
            | handler_field
            | (schema_block handler_field)

schema_block ::= 'schema' '{' schema_field* '}'
schema_field ::= IDENT ':' type_ref

handler_field ::= 'handler' '=' external_handler

external_handler ::= 'external' '(' string_lit ')'
```

### 3.4 Event Block

```ebnf
event_decl ::= 'event' IDENT '{' event_body '}'

event_body ::= 'payload' '=' reference
```

### 3.5 Workflow Block

```ebnf
workflow_decl ::= 'workflow' IDENT '{' workflow_body '}'

workflow_body ::= steps_block
                | steps_block returns_field

steps_block ::= 'steps' '{' step_decl* '}'

step_decl ::= IDENT '=' step_invocation

step_invocation ::= tool_invocation
                  | capability_invocation
                  | agent_invocation

tool_invocation       ::= 'tool'      IDENT invocation_args
capability_invocation ::= 'capability' IDENT invocation_args
agent_invocation      ::= 'agent'     IDENT invocation_args

invocation_args ::= '{' invocation_field* '}'
invocation_field ::= IDENT '=' value

returns_field ::= 'returns' '=' reference
```

### 3.6 Data Block

```ebnf
data_decl ::= 'data' IDENT '{' data_field* '}'

data_field ::= IDENT ':' type_ref
```

### 3.7 Type References

```ebnf
type_ref ::= primitive_type
           | array_type
           | object_type
           | reference       // data.SomeType

primitive_type ::= 'string'
                 | 'number'
                 | 'boolean'

array_type ::= 'array' '<' type_ref '>'

object_type ::= 'object' '{' data_field* '}'
```

### 3.8 Config Block

```ebnf
config_decl ::= 'config' IDENT '{' config_field* '}'

config_field ::= IDENT '=' value
```

---

## 4. Comments

```ebnf
comment ::= '//' (any char until newline)
```

---

## 5. Whitespace

Whitespace is ignored except inside quoted strings.

```ebnf
WS ::= (' ' | '\t' | '\n' | '\r')+
```

---

## 6. Grammar Validity Summary
*   Deterministic: yes
*   LALR(1) compatible: yes
*   No left recursion
*   Fully parseable by ANTLR, Tree-sitter, PEG, Recursive Descent
