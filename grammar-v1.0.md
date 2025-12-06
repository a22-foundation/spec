# A22 Grammar Specification v1.0

**Indentation-Based, Natural Language Syntax for Functional, Immutable, Temporal Agentic Systems**

**Status:** Stable

---

## Table of Contents

1. [Lexical Elements](#1-lexical-elements)
2. [Program Structure](#2-program-structure)
3. [Type System](#3-type-system)
4. [Event Type Definitions](#4-event-type-definitions)
5. [Projections and Queries](#5-projections-and-queries)
6. [Agents](#6-agents)
7. [Tools](#7-tools)
8. [Workflows](#8-workflows)
9. [Multi-Agent Coordination](#9-multi-agent-coordination)
10. [Streaming](#10-streaming)
11. [Error Handling](#11-error-handling)
12. [Policies & Privacy](#12-policies--privacy)
13. [Providers](#13-providers)
14. [Human-in-the-Loop](#14-human-in-the-loop)
15. [Scheduling](#15-scheduling)
16. [Observability](#16-observability)
17. [Deployment](#17-deployment)
18. [Testing](#18-testing)
19. [Imports & Exports](#19-imports--exports)
20. [Common Patterns](#20-common-patterns)

---

## 1. Lexical Elements

### 1.1 Tokens

```ebnf
KEYWORD      ::= 'agent' | 'tool' | 'workflow' | 'policy' | 'provider' | 'capability'
               | 'can' | 'use' | 'do' | 'has' | 'is' | 'when' | 'needs' | 'where'
               | 'steps' | 'parallel' | 'branch' | 'loop' | 'break' | 'continue' | 'return'
               | 'import' | 'from' | 'as' | 'export'
               | 'test' | 'given' | 'expect'
               | 'schedule' | 'every' | 'at' | 'in' | 'run' | 'with'
               | 'human_in_loop' | 'show' | 'ask' | 'options'
               | 'state' | 'prompt' | 'remembers' | 'isolation'
               | 'validates' | 'sandbox' | 'auth' | 'config' | 'limits'
               | 'allow' | 'deny' | 'primary' | 'fallback' | 'strategy'
               | 'type' | 'fields' | 'union' | 'optional'
               | 'event_type' | 'schema' | 'derives_from'
               | 'projection' | 'query' | 'filter' | 'transform' | 'aggregate'
               | 'context' | 'order_by' | 'limit' | 'group_by' | 'having'
               | 'role' | 'coordinator' | 'worker' | 'delegates_to' | 'reports_to'
               | 'stream' | 'window' | 'tumbling_window' | 'sliding_window' | 'session_window'
               | 'on_error' | 'retry' | 'fallback_chain' | 'circuit_breaker' | 'saga'
               | 'compensations' | 'on_failure' | 'rollback_from'
               | 'privacy' | 'pii_fields' | 'mark' | 'retention' | 'audit'
               | 'right_to_be_forgotten' | 'consent'
               | 'metrics' | 'logging' | 'trace' | 'trace_span' | 'debug_session'
               | 'deployment' | 'includes' | 'dependencies' | 'environment' | 'resources'
               | 'scaling' | 'high_availability'
               | 'migration' | 'transform_events' | 'add_metadata'
               | 'spec_version' | 'requires'

IDENT        ::= [a-zA-Z_][a-zA-Z0-9_]*
SYMBOL       ::= ':' [a-zA-Z_][a-zA-Z0-9_]*
REFERENCE    ::= '.' [a-zA-Z_][a-zA-Z0-9_]*
TAG          ::= '@' [a-zA-Z_][a-zA-Z0-9_]*

STRING       ::= '"' [^"]* '"'
NUMBER       ::= [0-9]+ ('.' [0-9]+)?
BOOLEAN      ::= 'true' | 'false'
UUID         ::= [0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}
TIMESTAMP    ::= ISO8601_DATETIME

ARROW        ::= '->'
RANGE        ::= '..'
COMMA        ::= ','
COLON        ::= ':'
DOT          ::= '.'
EQUALS       ::= '='
LT           ::= '<'
GT           ::= '>'
LE           ::= '<='
GE           ::= '>='
EQ           ::= '=='
NE           ::= '!='

OPEN_BRACKET  ::= '['
CLOSE_BRACKET ::= ']'
OPEN_BRACE    ::= '{'
CLOSE_BRACE   ::= '}'
OPEN_PAREN    ::= '('
CLOSE_PAREN   ::= ')'

INDENT       ::= <increase indentation>
DEDENT       ::= <decrease indentation>
NEWLINE      ::= '\n'
```

### 1.2 Comments

```ebnf
COMMENT      ::= '#' [^\n]* '\n'
```

### 1.3 Literals

```ebnf
duration     ::= NUMBER ('s' | 'm' | 'h' | 'd' | 'w')
size         ::= NUMBER ('kb' | 'mb' | 'gb' | 'tb')
```

---

## 2. Program Structure

### 2.1 Top Level

```ebnf
program      ::= spec_version? (declaration NEWLINE*)*

spec_version ::= 'spec_version' STRING NEWLINE

declaration  ::= type_decl
               | event_type_decl
               | projection_decl
               | agent_decl
               | tool_decl
               | workflow_decl
               | policy_decl
               | privacy_decl
               | provider_decl
               | schedule_decl
               | test_decl
               | import_decl
               | prompt_decl
               | capability_decl
               | metrics_decl
               | logging_decl
               | audit_decl
               | deployment_decl
               | migration_decl
               | config_decl
```

---

## 3. Type System

### 3.1 Type Declarations

```ebnf
type_decl    ::= 'type' IDENT NEWLINE INDENT type_body DEDENT

type_body    ::= struct_type
               | union_type
               | alias_type

struct_type  ::= 'fields' NEWLINE INDENT field_list DEDENT
                 validates_block?

field_list   ::= (field_decl NEWLINE)*

field_decl   ::= IDENT COLON type_ref

union_type   ::= 'union' NEWLINE INDENT union_variants DEDENT

union_variants ::= (union_variant NEWLINE)*

union_variant ::= IDENT (COLON type_ref)?
                | IDENT NEWLINE INDENT field_list DEDENT

alias_type   ::= '=' type_ref
```

### 3.2 Type References

```ebnf
type_ref     ::= basic_type
               | generic_type
               | optional_type
               | list_type
               | map_type
               | custom_type

basic_type   ::= 'text' | 'number' | 'boolean' | 'timestamp' | 'duration'
               | 'uuid' | 'any' | 'never'

generic_type ::= IDENT '<' type_ref (',' type_ref)* '>'

optional_type ::= 'optional' '<' type_ref '>'

list_type    ::= 'list' '<' type_ref '>'

map_type     ::= 'map' '<' type_ref ',' type_ref '>'

custom_type  ::= IDENT
```

---

## 4. Event Type Definitions

```ebnf
event_type_decl ::= 'event_type' STRING NEWLINE INDENT event_type_body DEDENT

event_type_body ::= (event_type_stmt NEWLINE)*

event_type_stmt ::= schema_stmt
                  | derives_from_stmt
                  | validates_stmt

schema_stmt     ::= 'schema' NEWLINE INDENT field_list DEDENT

derives_from_stmt ::= 'derives_from' COLON STRING
```

---

## 5. Projections and Queries

### 5.1 Projections

```ebnf
projection_decl ::= 'projection' STRING NEWLINE INDENT projection_body DEDENT

projection_body ::= (projection_stmt NEWLINE)*

projection_stmt ::= from_stmt
                  | filter_stmt
                  | transform_stmt
                  | aggregate_stmt
                  | output_stmt

from_stmt    ::= 'from' 'context'

filter_stmt  ::= 'filter' filter_expr

filter_expr  ::= 'type' COLON STRING
               | condition

transform_stmt ::= 'transform' NEWLINE INDENT transform_items DEDENT

transform_items ::= (transform_item NEWLINE)*

transform_item ::= IDENT '=' expr

aggregate_stmt ::= 'aggregate' NEWLINE INDENT aggregate_items DEDENT

aggregate_items ::= (aggregate_item NEWLINE)*

aggregate_item ::= IDENT COLON aggregate_expr

aggregate_expr ::= 'count' '(' expr ')'
                 | 'sum' '(' expr ',' 'field' COLON STRING ')'
                 | 'average' '(' expr ',' 'field' COLON STRING ')'
                 | 'min' '(' expr ',' 'field' COLON STRING ')'
                 | 'max' '(' expr ',' 'field' COLON STRING ')'
                 | 'percentile' '(' expr ',' 'field' COLON STRING ',' 'p' COLON NUMBER ')'

output_stmt  ::= 'output' NEWLINE INDENT output_items DEDENT

output_items ::= (output_item NEWLINE)*

output_item  ::= IDENT COLON expr
```

### 5.2 Context Queries

```ebnf
query_expr   ::= 'query' 'context' NEWLINE INDENT query_clauses DEDENT

query_clauses ::= (query_clause NEWLINE)*

query_clause ::= filter_clause
               | where_clause
               | order_by_clause
               | limit_clause
               | group_by_clause
               | having_clause
               | between_clause
               | correlation_clause

filter_clause ::= 'filter' filter_expr

where_clause ::= 'where' condition

order_by_clause ::= 'order_by' IDENT ('asc' | 'desc')?

limit_clause ::= 'limit' NUMBER

group_by_clause ::= 'group_by' IDENT

having_clause ::= 'having' condition

between_clause ::= 'between' TIMESTAMP 'and' TIMESTAMP

correlation_clause ::= 'correlation_id' COLON (IDENT | UUID)
```

---

## 6. Agents

### 6.1 Agent Declaration

```ebnf
agent_decl   ::= 'agent' STRING NEWLINE INDENT agent_body DEDENT

agent_body   ::= (agent_stmt NEWLINE)*

agent_stmt   ::= can_stmt
               | use_stmt
               | has_stmt
               | prompt_stmt
               | state_stmt
               | remembers_stmt
               | when_stmt
               | role_stmt
               | delegates_to_stmt
               | reports_to_stmt
               | isolation_stmt
               | workflow_stmt
```

### 6.2 Agent Statements

```ebnf
can_stmt     ::= 'can' capability_list

capability_list ::= IDENT (',' IDENT)*

role_stmt    ::= 'role' SYMBOL

delegates_to_stmt ::= 'delegates_to' NEWLINE INDENT delegate_options DEDENT

delegate_options ::= (delegate_option NEWLINE)*

delegate_option ::= 'agents' COLON list
                  | 'max_concurrent' COLON NUMBER
                  | 'strategy' COLON SYMBOL

reports_to_stmt ::= 'reports_to' COLON IDENT

use_stmt     ::= 'use' use_target

use_target   ::= simple_use
               | model_use
               | tool_use
               | prompt_use

simple_use   ::= IDENT COLON (SYMBOL | list | expr)

model_use    ::= 'model' (simple_model | advanced_model)

simple_model ::= COLON SYMBOL ('from' SYMBOL)?

advanced_model ::= NEWLINE INDENT model_config DEDENT

model_config ::= (model_option NEWLINE)*

model_option ::= 'primary' model_spec
               | 'fallback' list_of_models
               | 'strategy' SYMBOL

model_spec   ::= SYMBOL 'from' SYMBOL

list_of_models ::= '[' model_spec (',' model_spec)* ']'

tool_use     ::= 'tools' COLON list

prompt_use   ::= 'prompt' COLON SYMBOL

has_stmt     ::= 'has' property

property     ::= IDENT COLON (SYMBOL | value)

state_stmt   ::= 'state' SYMBOL NEWLINE INDENT state_options DEDENT

state_options ::= (state_option NEWLINE)*

state_option ::= 'backend' COLON SYMBOL
               | 'ttl' COLON duration
               | 'persist_to' COLON STRING

remembers_stmt ::= 'remembers' NEWLINE INDENT remember_items DEDENT

remember_items ::= (remember_item NEWLINE)*

remember_item ::= IDENT COLON remember_spec

remember_spec ::= 'last' NUMBER IDENT
                | 'always'
                | 'current_session'

when_stmt    ::= 'when' condition NEWLINE INDENT action DEDENT

action       ::= ARROW (call_expr | REFERENCE)
```

---

## 7. Tools

```ebnf
tool_decl    ::= 'tool' STRING NEWLINE INDENT tool_body DEDENT

tool_body    ::= (tool_stmt NEWLINE)*

tool_stmt    ::= 'endpoint' STRING
               | 'runtime' SYMBOL
               | 'handler' STRING
               | 'version' STRING
               | 'image' STRING
               | 'command' list
               | auth_stmt
               | input_stmt
               | output_stmt
               | validates_stmt
               | sandbox_stmt

auth_stmt    ::= 'auth' (env_ref | STRING)

env_ref      ::= 'env.' IDENT

input_stmt   ::= 'input' (COLON type_ref | NEWLINE INDENT field_list DEDENT)

output_stmt  ::= 'output' (COLON type_ref | NEWLINE INDENT field_list DEDENT)

validates_stmt ::= 'validates' NEWLINE INDENT validation_rules DEDENT

validation_rules ::= (validation_rule NEWLINE)*

validation_rule ::= field_path NEWLINE INDENT validation_constraints DEDENT

field_path   ::= IDENT ('.' IDENT)*

validation_constraints ::= (validation_constraint NEWLINE)*

validation_constraint ::= 'pattern' COLON STRING
                        | 'min_length' COLON NUMBER
                        | 'max_length' COLON NUMBER
                        | 'min' COLON NUMBER
                        | 'max' COLON NUMBER
                        | 'range' COLON NUMBER RANGE NUMBER
                        | 'deny_patterns' COLON list
                        | 'allowed_values' COLON list

sandbox_stmt ::= 'sandbox' NEWLINE INDENT sandbox_options DEDENT

sandbox_options ::= (sandbox_option NEWLINE)*

sandbox_option ::= 'timeout' COLON duration
                 | 'memory' COLON size
                 | 'cpu_limit' COLON NUMBER 'core' ('s')?
                 | 'network' COLON network_spec
                 | 'filesystem' NEWLINE INDENT filesystem_options DEDENT
                 | 'environment' NEWLINE INDENT env_vars DEDENT
                 | 'resource_limits' NEWLINE INDENT resource_limits DEDENT

network_spec ::= 'none' | 'limited' | 'full'

filesystem_options ::= (filesystem_option NEWLINE)*

filesystem_option ::= 'readonly' COLON list
                    | 'readwrite' COLON list
                    | 'deny' COLON list

env_vars     ::= (env_var NEWLINE)*

env_var      ::= IDENT COLON STRING

resource_limits ::= (resource_limit NEWLINE)*

resource_limit ::= IDENT COLON NUMBER
```

---

## 8. Workflows

### 8.1 Workflow Declaration

```ebnf
workflow_decl ::= 'workflow' STRING NEWLINE INDENT workflow_body DEDENT

workflow_body ::= (workflow_stmt NEWLINE)*

workflow_stmt ::= input_stream_stmt
                | deadline_stmt
                | steps_stmt
                | on_error_stmt
                | saga_stmt
```

### 8.2 Steps

```ebnf
steps_stmt   ::= 'steps' NEWLINE INDENT step_list DEDENT

step_list    ::= (step NEWLINE*)*

step         ::= assignment
               | parallel_stmt
               | branch_stmt
               | loop_stmt
               | return_stmt
               | when_stmt
               | stream_stmt

assignment   ::= IDENT '=' step_expr

step_expr    ::= call_expr
               | agent_call
               | tool_call
               | query_expr
               | projection_call
```

### 8.3 Parallel Execution

```ebnf
parallel_stmt ::= 'parallel' parallel_options? NEWLINE INDENT parallel_steps DEDENT

parallel_options ::= 'max' COLON NUMBER

parallel_steps ::= (parallel_step NEWLINE)*

parallel_step ::= assignment
                | for_loop
```

### 8.4 Branching

```ebnf
branch_stmt  ::= 'branch' expr NEWLINE INDENT branch_cases DEDENT

branch_cases ::= (branch_case NEWLINE)*

branch_case  ::= when_case
               | otherwise_case

when_case    ::= 'when' condition NEWLINE INDENT action DEDENT

otherwise_case ::= 'otherwise' NEWLINE INDENT action DEDENT
```

### 8.5 Loops

```ebnf
loop_stmt    ::= 'loop' loop_options NEWLINE INDENT loop_body DEDENT

loop_options ::= 'max' COLON NUMBER

loop_body    ::= (loop_item NEWLINE)*

loop_item    ::= step
               | break_stmt
               | continue_stmt

break_stmt   ::= 'when' condition NEWLINE INDENT ARROW 'break' DEDENT

continue_stmt ::= 'when' condition NEWLINE INDENT ARROW 'continue' DEDENT

for_loop     ::= 'for' IDENT 'in' expr NEWLINE INDENT step_list DEDENT
```

### 8.6 Return

```ebnf
return_stmt  ::= 'return' expr
```

---

## 9. Multi-Agent Coordination

Multi-agent syntax is integrated into agent declarations (see §6).

Agent calls within workflows:

```ebnf
agent_call   ::= 'agent' (STRING | IDENT) NEWLINE INDENT agent_call_args DEDENT

agent_call_args ::= (agent_call_arg NEWLINE)*

agent_call_arg ::= IDENT COLON expr
```

---

## 10. Streaming

### 10.1 Input Streams

```ebnf
input_stream_stmt ::= 'input_stream' COLON IDENT NEWLINE INDENT stream_filter DEDENT

stream_filter ::= 'filter' filter_expr
```

### 10.2 Stream Processing

```ebnf
stream_stmt  ::= 'stream' NEWLINE INDENT stream_body DEDENT

stream_body  ::= (stream_item NEWLINE)*

stream_item  ::= next_event_stmt
               | window_stmt
               | step

next_event_stmt ::= IDENT '=' 'next_event' '(' ')'

window_stmt  ::= IDENT '=' window_type NEWLINE INDENT window_config DEDENT

window_type  ::= 'window'
               | 'tumbling_window'
               | 'sliding_window'
               | 'session_window'

window_config ::= (window_option NEWLINE)*

window_option ::= 'size' COLON NUMBER IDENT
                | 'duration' COLON duration
                | 'slide' COLON (NUMBER IDENT | duration)
                | 'gap' COLON duration
                | 'compute' NEWLINE INDENT compute_items DEDENT

compute_items ::= (compute_item NEWLINE)*

compute_item ::= IDENT COLON aggregate_expr
```

---

## 11. Error Handling

### 11.1 Error Handling Statement

```ebnf
on_error_stmt ::= 'on_error' NEWLINE INDENT error_handlers DEDENT

error_handlers ::= (error_handler NEWLINE)*

error_handler ::= when_error_handler
                | retry_handler
                | fallback_handler
                | circuit_breaker_handler
                | otherwise_handler

when_error_handler ::= 'when' condition NEWLINE INDENT error_action DEDENT

error_action ::= retry_stmt
               | fallback_stmt
               | compensate_stmt

retry_stmt   ::= 'retry' NEWLINE INDENT retry_options DEDENT

retry_options ::= (retry_option NEWLINE)*

retry_option ::= 'max_attempts' COLON NUMBER
               | 'backoff' COLON backoff_strategy
               | 'initial_delay' COLON duration
               | 'max_delay' COLON duration
               | 'multiplier' COLON NUMBER
               | 'jitter' COLON NUMBER
               | 'retry_on' COLON list
               | 'give_up_on' COLON list

backoff_strategy ::= 'exponential' | 'linear' | 'constant'

fallback_stmt ::= 'fallback' NEWLINE INDENT call_expr DEDENT

fallback_handler ::= 'fallback_chain' NEWLINE INDENT fallback_chain DEDENT

fallback_chain ::= (fallback_chain_item NEWLINE)*

fallback_chain_item ::= ARROW call_expr
                      | 'on_error' ARROW call_expr

circuit_breaker_handler ::= 'circuit_breaker' NEWLINE INDENT circuit_breaker_options DEDENT

circuit_breaker_options ::= (circuit_breaker_option NEWLINE)*

circuit_breaker_option ::= 'failure_threshold' COLON NUMBER
                         | 'success_threshold' COLON NUMBER
                         | 'timeout' COLON duration
                         | 'half_open_max_calls' COLON NUMBER
                         | 'on_open' NEWLINE INDENT action DEDENT

compensate_stmt ::= 'compensate' NEWLINE INDENT compensate_options DEDENT

compensate_options ::= (compensate_option NEWLINE)*

compensate_option ::= 'rollback_steps' COLON list
                    | 'notify' COLON IDENT
                    | 'log_error' COLON IDENT

otherwise_handler ::= 'otherwise' NEWLINE INDENT error_action DEDENT
```

### 11.2 Saga Pattern

```ebnf
saga_stmt    ::= 'saga' NEWLINE INDENT saga_body DEDENT

saga_body    ::= steps_stmt
                 compensations_stmt
                 on_failure_stmt

compensations_stmt ::= 'compensations' NEWLINE INDENT compensation_items DEDENT

compensation_items ::= (compensation_item NEWLINE)*

compensation_item ::= IDENT ARROW call_expr

on_failure_stmt ::= 'on_failure' NEWLINE INDENT on_failure_options DEDENT

on_failure_options ::= (on_failure_option NEWLINE)*

on_failure_option ::= 'rollback_from' COLON IDENT
                    | 'rollback_order' COLON ('forward' | 'reverse')
                    | 'log_compensation' COLON BOOLEAN
                    | 'notification' NEWLINE INDENT notification_options DEDENT
```

### 11.3 Deadline

```ebnf
deadline_stmt ::= 'deadline' COLON duration
```

---

## 12. Policies & Privacy

### 12.1 Policy Declaration

```ebnf
policy_decl  ::= 'policy' SYMBOL NEWLINE INDENT policy_body DEDENT

policy_body  ::= (policy_stmt NEWLINE)*

policy_stmt  ::= allow_stmt
               | deny_stmt
               | require_stmt
               | limits_stmt

allow_stmt   ::= 'allow' NEWLINE INDENT allow_items DEDENT

allow_items  ::= (allow_item NEWLINE)*

allow_item   ::= 'tools' COLON list
               | 'agents' COLON list
               | 'capabilities' COLON list
               | 'data_access' NEWLINE INDENT data_access_rules DEDENT
               | 'operations' NEWLINE INDENT operations_rules DEDENT

data_access_rules ::= (data_access_rule NEWLINE)*

data_access_rule ::= 'level' COLON symbol_list
                   | 'not' COLON symbol_list

deny_stmt    ::= 'deny' NEWLINE INDENT deny_items DEDENT

deny_items   ::= (deny_item NEWLINE)*

deny_item    ::= allow_item

require_stmt ::= 'require' NEWLINE INDENT require_items DEDENT

require_items ::= (require_item NEWLINE)*

require_item ::= IDENT COLON BOOLEAN
               | 'mfa_for' NEWLINE INDENT mfa_actions DEDENT

limits_stmt  ::= 'limits' NEWLINE INDENT limit_items DEDENT

limit_items  ::= (limit_item NEWLINE)*

limit_item   ::= rate_limit
               | resource_limit
               | cost_limit

rate_limit   ::= 'rate' NEWLINE INDENT rate_options DEDENT

rate_options ::= (rate_option NEWLINE)*

rate_option  ::= IDENT COLON NUMBER

resource_limit ::= 'resource' NEWLINE INDENT resource_options DEDENT

cost_limit   ::= 'cost' NEWLINE INDENT cost_options DEDENT
```

### 12.2 Privacy Declaration

```ebnf
privacy_decl ::= 'privacy' NEWLINE INDENT privacy_body DEDENT

privacy_body ::= (privacy_stmt NEWLINE)*

privacy_stmt ::= pii_fields_stmt
               | retention_stmt
               | right_to_be_forgotten_stmt
               | consent_stmt

pii_fields_stmt ::= 'pii_fields' NEWLINE INDENT pii_mark_stmt DEDENT

pii_mark_stmt ::= 'mark' NEWLINE INDENT pii_marks DEDENT

pii_marks    ::= (pii_mark NEWLINE)*

pii_mark     ::= field_path COLON SYMBOL

retention_stmt ::= 'retention' NEWLINE INDENT retention_rules DEDENT

retention_rules ::= (retention_rule NEWLINE)*

retention_rule ::= IDENT NEWLINE INDENT retention_options DEDENT

retention_options ::= (retention_option NEWLINE)*

retention_option ::= 'ttl' COLON duration
                   | 'after_deletion' COLON IDENT
                   | 'immutable' COLON BOOLEAN

right_to_be_forgotten_stmt ::= 'right_to_be_forgotten' NEWLINE INDENT rtbf_body DEDENT

rtbf_body    ::= (rtbf_stmt NEWLINE)*

rtbf_stmt    ::= 'on_request' COLON IDENT NEWLINE INDENT rtbf_actions DEDENT

rtbf_actions ::= (rtbf_action NEWLINE)*

rtbf_action  ::= 'delete_all_events' NEWLINE INDENT where_clause DEDENT
               | 'anonymize_events' NEWLINE INDENT where_clause DEDENT
               | 'create_deletion_certificate' NEWLINE INDENT cert_options DEDENT

consent_stmt ::= 'consent' NEWLINE INDENT consent_options DEDENT
```

---

## 13. Providers

```ebnf
provider_decl ::= 'provider' SYMBOL NEWLINE INDENT provider_body DEDENT

provider_body ::= (provider_stmt NEWLINE)*

provider_stmt ::= 'type' SYMBOL
                | auth_stmt
                | config_stmt
                | limits_stmt
                | models_stmt

config_stmt  ::= 'config' NEWLINE INDENT config_items DEDENT

config_items ::= (config_item NEWLINE)*

config_item  ::= IDENT (STRING | NUMBER | SYMBOL | duration)
               | 'retry_policy' NEWLINE INDENT retry_options DEDENT

models_stmt  ::= 'models' NEWLINE INDENT model_defs DEDENT

model_defs   ::= (model_def NEWLINE)*

model_def    ::= SYMBOL NEWLINE INDENT model_properties DEDENT

model_properties ::= (model_property NEWLINE)*

model_property ::= IDENT COLON (STRING | NUMBER)
```

---

## 14. Human-in-the-Loop

```ebnf
human_in_loop_decl ::= 'human_in_loop' STRING NEWLINE INDENT hil_body DEDENT

hil_body     ::= (hil_stmt NEWLINE)*

hil_stmt     ::= show_stmt
               | ask_stmt
               | options_stmt
               | timeout_stmt
               | default_stmt
               | notifications_stmt
               | role_stmt

show_stmt    ::= 'show' (expr | NEWLINE INDENT show_items DEDENT)

show_items   ::= (show_item NEWLINE)*

show_item    ::= IDENT COLON expr

ask_stmt     ::= 'ask' COLON STRING

options_stmt ::= 'options' NEWLINE INDENT option_defs DEDENT

option_defs  ::= (option_def NEWLINE)*

option_def   ::= IDENT (NEWLINE INDENT option_properties DEDENT)?

option_properties ::= (option_property NEWLINE)*

option_property ::= 'label' COLON STRING
                  | 'style' COLON SYMBOL
                  | 'requires_input' COLON type_ref
                  | 'prompt' COLON STRING

timeout_stmt ::= 'timeout' COLON duration

default_stmt ::= 'default' COLON (IDENT | STRING)

notifications_stmt ::= 'notifications' NEWLINE INDENT notification_options DEDENT

notification_options ::= (notification_option NEWLINE)*

notification_option ::= 'channels' COLON list
                      | 'recipients' COLON list
                      | 'reminder_after' COLON duration
```

---

## 15. Scheduling

```ebnf
schedule_decl ::= 'schedule' STRING NEWLINE INDENT schedule_body DEDENT

schedule_body ::= (schedule_stmt NEWLINE)*

schedule_stmt ::= trigger_stmt
                | run_stmt
                | with_stmt
                | when_stmt
                | paused_during_stmt

trigger_stmt ::= interval_trigger
               | cron_trigger
               | event_trigger

interval_trigger ::= 'every' (duration | time_spec)

time_spec    ::= (NUMBER | IDENT) 'at' STRING ('in' STRING)?

cron_trigger ::= 'cron' STRING

event_trigger ::= 'when' (IDENT | query_expr)

run_stmt     ::= 'run' REFERENCE

with_stmt    ::= 'with' NEWLINE INDENT with_items DEDENT

with_items   ::= (with_item NEWLINE)*

with_item    ::= IDENT COLON value

paused_during_stmt ::= 'paused_during' NEWLINE INDENT pause_conditions DEDENT

pause_conditions ::= (pause_condition NEWLINE)*

pause_condition ::= IDENT COLON IDENT
```

---

## 16. Observability

### 16.1 Metrics

```ebnf
metrics_decl ::= 'metrics' NEWLINE INDENT metric_items DEDENT

metric_items ::= (metric_item NEWLINE)*

metric_item  ::= metric_def
               | business_metric_def

metric_def   ::= IDENT NEWLINE INDENT metric_config DEDENT

metric_config ::= (metric_config_item NEWLINE)*

metric_config_item ::= 'type' COLON metric_type
                     | 'from_events' COLON (STRING | list)
                     | 'labels' COLON list
                     | 'buckets' COLON list
                     | 'unit' COLON STRING
                     | 'compute' COLON expr

metric_type  ::= 'counter' | 'gauge' | 'histogram'

business_metric_def ::= 'business_metric' STRING NEWLINE INDENT business_metric_config DEDENT

business_metric_config ::= (business_metric_item NEWLINE)*

business_metric_item ::= 'from_events' COLON STRING
                       | 'compute' COLON expr
                       | 'window' COLON duration
                       | 'labels' COLON list
```

### 16.2 Logging

```ebnf
logging_decl ::= 'logging' NEWLINE INDENT logging_body DEDENT

logging_body ::= (logging_stmt NEWLINE)*

logging_stmt ::= 'level' COLON log_level
               | 'structured' COLON BOOLEAN
               | 'format' COLON SYMBOL
               | 'outputs' NEWLINE INDENT output_configs DEDENT
               | 'log_events' NEWLINE INDENT log_event_config DEDENT

log_level    ::= 'debug' | 'info' | 'warn' | 'error' | 'critical'

output_configs ::= (output_config NEWLINE)*

output_config ::= IDENT NEWLINE INDENT output_options DEDENT

output_options ::= (output_option NEWLINE)*

output_option ::= 'enabled' COLON BOOLEAN
                | 'level' COLON log_level
                | 'path' COLON STRING
                | 'endpoint' COLON STRING
                | 'rotation' NEWLINE INDENT rotation_options DEDENT
```

### 16.3 Tracing

```ebnf
trace_stmt   ::= 'trace' NEWLINE INDENT trace_config DEDENT

trace_config ::= (trace_config_item NEWLINE)*

trace_config_item ::= 'trace_id' COLON (IDENT | 'auto_generated')
                    | 'spans' NEWLINE INDENT span_config DEDENT
                    | 'visualization' COLON SYMBOL

trace_span_stmt ::= 'trace_span' STRING NEWLINE INDENT span_options DEDENT

span_options ::= (span_option NEWLINE)*

span_option  ::= 'tags' COLON list
               | 'attributes' NEWLINE INDENT attr_items DEDENT
```

### 16.4 Debug Session

```ebnf
debug_session_decl ::= 'debug_session' NEWLINE INDENT debug_body DEDENT

debug_body   ::= (debug_stmt NEWLINE)*

debug_stmt   ::= 'context' COLON IDENT
               | 'replay_from' COLON IDENT
               | 'breakpoints' NEWLINE INDENT breakpoint_list DEDENT
               | 'trace_level' COLON SYMBOL
               | 'step_mode' COLON BOOLEAN
               | 'variable_inspection' COLON BOOLEAN
               | 'context_modifications' NEWLINE INDENT context_mods DEDENT

breakpoint_list ::= (breakpoint NEWLINE)*

breakpoint   ::= 'on_event' COLON STRING (where_clause)?
               | 'on_condition' COLON condition
               | 'on_data_value' COLON condition
```

---

## 17. Deployment

```ebnf
deployment_decl ::= 'deployment' STRING NEWLINE INDENT deployment_body DEDENT

deployment_body ::= (deployment_stmt NEWLINE)*

deployment_stmt ::= 'version' COLON STRING
                  | 'spec_version' COLON STRING
                  | includes_stmt
                  | dependencies_stmt
                  | environment_stmt
                  | resources_stmt
                  | health_check_stmt
                  | scaling_stmt
                  | high_availability_stmt

includes_stmt ::= 'includes' NEWLINE INDENT include_items DEDENT

include_items ::= (include_item NEWLINE)*

include_item ::= IDENT COLON list

dependencies_stmt ::= 'dependencies' NEWLINE INDENT dependency_items DEDENT

dependency_items ::= (dependency_item NEWLINE)*

dependency_item ::= IDENT COLON STRING
                  | IDENT NEWLINE INDENT version_specs DEDENT

version_specs ::= (version_spec NEWLINE)*

version_spec ::= IDENT COLON STRING

environment_stmt ::= 'environment' NEWLINE INDENT env_body DEDENT

env_body     ::= (env_section NEWLINE)*

env_section  ::= 'requires' NEWLINE INDENT requirements DEDENT
               | 'optional' NEWLINE INDENT requirements DEDENT
               | 'secrets' NEWLINE INDENT secret_mappings DEDENT

requirements ::= (requirement NEWLINE)*

requirement  ::= IDENT COLON STRING

secret_mappings ::= (secret_mapping NEWLINE)*

secret_mapping ::= IDENT COLON env_ref

resources_stmt ::= 'resources' NEWLINE INDENT resource_specs DEDENT

resource_specs ::= (resource_spec NEWLINE)*

resource_spec ::= 'memory' COLON size
                | 'cpu' COLON NUMBER ('core' | 'cores')
                | 'storage' COLON size
                | 'gpu' COLON ('optional' | NUMBER)

health_check_stmt ::= 'health_check' NEWLINE INDENT health_options DEDENT

health_options ::= (health_option NEWLINE)*

health_option ::= 'endpoint' COLON STRING
                | 'interval' COLON duration
                | 'timeout' COLON duration
                | 'retries' COLON NUMBER

scaling_stmt ::= 'scaling' NEWLINE INDENT scaling_options DEDENT

scaling_options ::= (scaling_option NEWLINE)*

scaling_option ::= 'auto_scaling' NEWLINE INDENT auto_scaling_config DEDENT
                 | 'load_balancing' NEWLINE INDENT lb_config DEDENT

auto_scaling_config ::= (auto_scaling_item NEWLINE)*

auto_scaling_item ::= 'enabled' COLON BOOLEAN
                    | 'min_instances' COLON NUMBER
                    | 'max_instances' COLON NUMBER
                    | 'metrics' NEWLINE INDENT scaling_metrics DEDENT
                    | 'scale_up' NEWLINE INDENT scale_action DEDENT
                    | 'scale_down' NEWLINE INDENT scale_action DEDENT

high_availability_stmt ::= 'high_availability' NEWLINE INDENT ha_options DEDENT

ha_options   ::= (ha_option NEWLINE)*

ha_option    ::= 'replicas' COLON NUMBER
               | 'distribution' COLON IDENT
               | 'failover_time' COLON (LT duration)
```

---

## 18. Testing

```ebnf
test_decl    ::= 'test' STRING NEWLINE INDENT test_body DEDENT

test_body    ::= (test_stmt NEWLINE)*

test_stmt    ::= given_stmt
               | when_stmt
               | scenario_stmt
               | expect_stmt
               | property_stmt

given_stmt   ::= 'given' NEWLINE INDENT given_items DEDENT

given_items  ::= (given_item NEWLINE)*

given_item   ::= IDENT COLON (SYMBOL | value | IDENT)
               | 'context' list
               | 'input' (value | NEWLINE INDENT input_fields DEDENT)

scenario_stmt ::= 'scenario' NEWLINE INDENT scenario_steps DEDENT

scenario_steps ::= (scenario_step NEWLINE*)*

scenario_step ::= user_input_stmt
                | expect_stmt

user_input_stmt ::= IDENT STRING

expect_stmt  ::= 'expect' NEWLINE INDENT expect_items DEDENT

expect_items ::= (expect_item NEWLINE)*

expect_item  ::= 'produces' expect_event
               | 'calls' call_assertion
               | 'completes' time_assertion
               | 'no' IDENT
               | assertion_expr

expect_event ::= ('event' | IDENT) (NEWLINE INDENT event_assertions DEDENT)?

event_assertions ::= (event_assertion NEWLINE)*

event_assertion ::= field_path (comparison | 'contains' | 'is_not') expr

call_assertion ::= (IDENT | 'tool' | 'agent') IDENT (with_clause | count_clause)?

with_clause  ::= 'with' NEWLINE INDENT call_with_items DEDENT

call_with_items ::= (call_with_item NEWLINE)*

call_with_item ::= IDENT comparison expr

count_clause ::= ('once' | NUMBER ('time' | 'times'))

time_assertion ::= 'within' duration

assertion_expr ::= expr comparison expr
                 | expr ('contains' | 'is' | 'is_not') expr

property_stmt ::= 'property' NEWLINE INDENT property_body DEDENT

property_body ::= (property_item NEWLINE)*

property_item ::= 'for_all' IDENT 'in' IDENT NEWLINE INDENT assertions DEDENT

assertions   ::= (assertion NEWLINE)*

assertion    ::= 'assert' assertion_expr
```

---

## 19. Imports & Exports

```ebnf
import_decl  ::= 'import' NEWLINE INDENT import_items DEDENT
               | 'import' 'from' STRING

import_items ::= (import_item NEWLINE)*

import_item  ::= import_type STRING ('as' STRING)?
               | import_type STRING 'from' STRING

import_type  ::= 'agent' | 'tool' | 'workflow' | 'policy' | 'type' | 'capability'
```

### Migration

```ebnf
migration_decl ::= 'migration' STRING NEWLINE INDENT migration_body DEDENT

migration_body ::= (migration_stmt NEWLINE)*

migration_stmt ::= 'from_version' COLON STRING
                 | 'to_version' COLON STRING
                 | transform_events_stmt
                 | add_metadata_stmt
                 | validate_stmt

transform_events_stmt ::= 'transform_events' NEWLINE INDENT event_transforms DEDENT

event_transforms ::= (event_transform NEWLINE)*

event_transform ::= 'when' condition NEWLINE INDENT transform_action DEDENT

transform_action ::= ARROW 'new_event' NEWLINE INDENT new_event_spec DEDENT

new_event_spec ::= (event_field NEWLINE)*

event_field  ::= IDENT COLON expr

add_metadata_stmt ::= 'add_metadata' NEWLINE INDENT metadata_rules DEDENT

metadata_rules ::= (metadata_rule NEWLINE)*

metadata_rule ::= 'events' COLON (IDENT | 'all')
                | 'metadata' NEWLINE INDENT metadata_items DEDENT

validate_stmt ::= 'validate' NEWLINE INDENT validation_assertions DEDENT

validation_assertions ::= (validation_assertion NEWLINE)*

validation_assertion ::= IDENT COLON (BOOLEAN | expr)
```

---

## 20. Common Patterns

### 20.1 Expressions

```ebnf
expr         ::= literal
               | IDENT
               | field_access
               | SYMBOL
               | REFERENCE
               | call_expr
               | comparison
               | logical_expr
               | list
               | map

literal      ::= STRING | NUMBER | BOOLEAN | UUID | TIMESTAMP

field_access ::= IDENT ('.' IDENT)+

call_expr    ::= IDENT (OPEN_PAREN arg_list CLOSE_PAREN | call_args)?

arg_list     ::= expr (',' expr)*

call_args    ::= (IDENT COLON expr | NEWLINE INDENT arg_items DEDENT)

arg_items    ::= (arg_item NEWLINE)*

arg_item     ::= IDENT COLON expr

comparison   ::= expr comp_op expr

comp_op      ::= EQ | NE | LT | GT | LE | GE
               | 'contains' | 'is' | 'is_not' | 'in' | 'not' 'in'

logical_expr ::= expr ('and' | 'or') expr
               | 'not' expr

list         ::= OPEN_BRACKET (expr (',' expr)*)? CLOSE_BRACKET

map          ::= OPEN_BRACE (map_entry (',' map_entry)*)? CLOSE_BRACE

map_entry    ::= (IDENT | STRING) COLON expr

symbol_list  ::= SYMBOL (',' SYMBOL)*

condition    ::= comparison | logical_expr | expr
```

### 20.2 Value References

```ebnf
value        ::= literal | list | map | SYMBOL | REFERENCE
```

---

## 21. Indentation Rules

### 21.1 Basic Rules

1. **Consistent Indentation**: Use either tabs OR spaces (4 spaces recommended), not mixed
2. **Block Scope**: INDENT starts a block, DEDENT ends it
3. **Nested Blocks**: Each level adds one indent unit
4. **Empty Lines**: Ignored (don't affect indentation level)
5. **Comments**: Don't affect indentation level

### 21.2 Indentation Stack

The lexer maintains an indentation stack:

```
Initial state: stack = [0]

On new line:
  1. Measure leading whitespace (spaces/tabs)
  2. If indent > stack.top: push indent, emit INDENT
  3. If indent < stack.top:
     While stack.top > indent:
       pop stack, emit DEDENT
  4. If indent == stack.top: continue

On EOF:
  While stack.length > 1:
    pop stack, emit DEDENT
```

### 21.3 Tab Expansion

Tabs are treated as 4 spaces for consistency.

---

## 22. Reserved Keywords

All keywords are reserved and cannot be used as identifiers:

```
agent, tool, workflow, policy, provider, capability, type
can, use, do, has, is, when, needs, where, otherwise
steps, parallel, branch, loop, break, continue, return, for, in
import, from, as, export
test, given, expect, scenario, property, assert
schedule, every, at, in, run, with, paused_during
human_in_loop, show, ask, options, timeout, default
state, prompt, remembers, isolation, role
validates, sandbox, auth, config, limits, resources
allow, deny, require, primary, fallback, strategy
fields, union, optional, schema, derives_from
event_type, projection, query, filter, transform, aggregate, output
context, order_by, limit, group_by, having, between
coordinator, worker, delegates_to, reports_to
stream, window, tumbling_window, sliding_window, session_window
next_event, compute
on_error, retry, fallback_chain, circuit_breaker, saga
compensations, on_failure, rollback_from, deadline
privacy, pii_fields, mark, retention, audit
right_to_be_forgotten, consent
metrics, logging, trace, trace_span, debug_session
deployment, includes, dependencies, environment
scaling, high_availability, health_check
migration, transform_events, add_metadata, validate
spec_version, requires
```

---

## 23. Error Recovery

Parsers should provide helpful error messages for common mistakes:

- **Missing indentation**: "Expected indented block after 'agent'"
- **Inconsistent indentation**: "Indentation error: mixing tabs and spaces"
- **Invalid keyword placement**: "'can' statement only valid in agent body"
- **Missing required fields**: "Agent declaration requires 'can' statement"
- **Type mismatches**: "Expected type 'number', got 'text'"
- **Invalid references**: "Reference '.unknown_workflow' not found"
- **Circular dependencies**: "Circular dependency detected: A → B → A"
- **Unknown event types**: "Unknown event type 'custom.event' (did you mean 'tool.custom'?)"

---

**End of Grammar Specification**

**A22 v1.0 Grammar — Stable**
**© 2024 A22 Foundation**
**License: Apache 2.0**
