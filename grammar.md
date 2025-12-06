# A22 Grammar Specification v0.1

**Indentation-Based, Natural Language Syntax**

---

## 1. Lexical Elements

### 1.1 Tokens

```
KEYWORD      ::= 'agent' | 'tool' | 'workflow' | 'policy' | 'provider'
               | 'can' | 'use' | 'do' | 'has' | 'is' | 'when' | 'needs'
               | 'steps' | 'parallel' | 'branch' | 'loop' | 'return'
               | 'import' | 'from' | 'test' | 'given' | 'expect'
               | 'schedule' | 'every' | 'at' | 'in' | 'run'
               | 'human_in_loop' | 'show' | 'ask' | 'options'
               | 'state' | 'prompt' | 'validates' | 'sandbox'
               | 'auth' | 'config' | 'limits' | 'allow' | 'deny'

IDENT        ::= [a-zA-Z_][a-zA-Z0-9_]*
SYMBOL       ::= ':' [a-zA-Z_][a-zA-Z0-9_]*
REFERENCE    ::= '.' [a-zA-Z_][a-zA-Z0-9_]*
TAG          ::= '@' [a-zA-Z_][a-zA-Z0-9_]*

STRING       ::= '"' [^"]* '"'
NUMBER       ::= [0-9]+ ('.' [0-9]+)?
BOOLEAN      ::= 'true' | 'false'

ARROW        ::= '->'
RANGE        ::= '..'
COMMA        ::= ','
COLON        ::= ':'

INDENT       ::= <increase indentation>
DEDENT       ::= <decrease indentation>
NEWLINE      ::= '\n'
```

### 1.2 Comments

```
COMMENT      ::= '#' [^\n]* '\n'
```

---

## 2. Program Structure

### 2.1 Top Level

```
program      ::= (declaration NEWLINE*)*

declaration  ::= agent_decl
               | tool_decl
               | workflow_decl
               | policy_decl
               | provider_decl
               | schedule_decl
               | test_decl
               | import_decl
               | prompt_decl
               | capability_decl
```

---

## 3. Agent Declaration

```
agent_decl   ::= 'agent' STRING NEWLINE INDENT agent_body DEDENT

agent_body   ::= (agent_stmt NEWLINE)*

agent_stmt   ::= can_stmt
               | use_stmt
               | do_stmt
               | has_stmt
               | when_stmt
               | prompt_stmt
               | state_stmt
               | remembers_stmt
               | isolation_stmt
```

### 3.1 Agent Statements

```
can_stmt     ::= 'can' capability_list

capability_list ::= IDENT (',' IDENT)*

use_stmt     ::= 'use' (simple_use | model_use | tool_use)

simple_use   ::= IDENT (use_options)?

model_use    ::= 'model' ( COLON SYMBOL | NEWLINE INDENT model_config DEDENT )

tool_use     ::= IDENT NEWLINE INDENT tool_options DEDENT

do_stmt      ::= 'do' REFERENCE (when_clause)?

has_stmt     ::= 'has' (property_list | property_block)

property_list ::= IDENT COLON value (',' IDENT COLON value)*

property_block ::= NEWLINE INDENT (IDENT COLON value NEWLINE)* DEDENT

when_stmt    ::= 'when' condition NEWLINE INDENT action DEDENT
```

### 3.2 Model Configuration

```
model_config ::= (model_option NEWLINE)*

model_option ::= 'primary' model_spec
               | 'fallback' list_of_models
               | 'strategy' SYMBOL
               | 'params' NEWLINE INDENT params_block DEDENT

model_spec   ::= SYMBOL 'from' SYMBOL

list_of_models ::= '[' model_spec (',' model_spec)* ']'
```

### 3.3 State Configuration

```
state_stmt   ::= 'state' SYMBOL NEWLINE INDENT state_options DEDENT

state_options ::= (state_option NEWLINE)*

state_option  ::= 'backend' SYMBOL
                | 'ttl' duration
                | 'persist_to' SYMBOL

remembers_stmt ::= 'remembers' NEWLINE INDENT remember_items DEDENT

remember_items ::= (remember_item NEWLINE)*

remember_item  ::= IDENT COLON remember_spec

remember_spec  ::= 'last' NUMBER IDENT
                 | 'always'
                 | 'current_session'
```

### 3.4 Isolation Configuration

```
isolation_stmt ::= 'isolation' NEWLINE INDENT isolation_options DEDENT

isolation_options ::= (isolation_option NEWLINE)*

isolation_option ::= 'memory' COLON IDENT
                   | 'network' COLON IDENT
                   | 'filesystem' COLON IDENT
```

---

## 4. Tool Declaration

```
tool_decl    ::= 'tool' STRING NEWLINE INDENT tool_body DEDENT

tool_body    ::= (tool_stmt NEWLINE)*

tool_stmt    ::= 'endpoint' STRING
               | 'runtime' SYMBOL
               | auth_stmt
               | input_stmt
               | output_stmt
               | validates_stmt
               | sandbox_stmt
```

### 4.1 Tool Statements

```
auth_stmt    ::= 'auth' env_ref

env_ref      ::= 'env.' IDENT

input_stmt   ::= 'input' NEWLINE INDENT input_fields DEDENT

input_fields ::= (field_decl NEWLINE)*

field_decl   ::= IDENT COLON type_spec

type_spec    ::= 'text' | 'number' | 'boolean' | 'list' | 'map' | 'any'
               | 'optional' type_spec

output_stmt  ::= 'output' NEWLINE INDENT output_fields DEDENT

validates_stmt ::= 'validates' NEWLINE INDENT validation_rules DEDENT

validation_rules ::= (validation_rule NEWLINE)*

validation_rule ::= IDENT ('.' IDENT)* COLON (value | comparison)

sandbox_stmt ::= 'sandbox' NEWLINE INDENT sandbox_options DEDENT

sandbox_options ::= (sandbox_option NEWLINE)*

sandbox_option ::= 'timeout' COLON duration
                 | 'memory' COLON size
                 | 'network' COLON network_spec
                 | 'filesystem' COLON filesystem_spec
```

---

## 5. Workflow Declaration

```
workflow_decl ::= 'workflow' STRING NEWLINE INDENT workflow_body DEDENT

workflow_body ::= (workflow_stmt NEWLINE)*

workflow_stmt ::= state_stmt
                | steps_stmt
                | parallel_stmt
                | on_failure_stmt
```

### 5.1 Steps

```
steps_stmt   ::= 'steps' NEWLINE INDENT step_list DEDENT

step_list    ::= (step NEWLINE)*

step         ::= IDENT '=' step_expr
               | parallel_stmt
               | branch_stmt
               | loop_stmt
               | return_stmt
               | when_stmt

step_expr    ::= call_expr
               | agent_call
               | tool_call
```

### 5.2 Parallel Execution

```
parallel_stmt ::= 'parallel' NEWLINE INDENT parallel_steps DEDENT

parallel_steps ::= (parallel_step NEWLINE)*

parallel_step  ::= IDENT '=' call_expr
```

### 5.3 Branching

```
branch_stmt  ::= 'branch' expr NEWLINE INDENT branch_cases DEDENT

branch_cases ::= (branch_case NEWLINE)*

branch_case  ::= 'when' condition ARROW action
```

### 5.4 Loops

```
loop_stmt    ::= 'loop' loop_options NEWLINE INDENT loop_body DEDENT

loop_options ::= 'max' COLON NUMBER

loop_body    ::= (loop_item NEWLINE)*

loop_item    ::= step
               | 'when' condition NEWLINE INDENT ARROW 'break' DEDENT
```

### 5.5 Return

```
return_stmt  ::= 'return' expr
```

---

## 6. Human-in-the-Loop

```
human_in_loop_decl ::= 'human_in_loop' STRING NEWLINE INDENT hil_body DEDENT

hil_body     ::= (hil_stmt NEWLINE)*

hil_stmt     ::= 'show' expr
               | 'ask' STRING
               | 'options' list
               | 'timeout' duration
               | 'default' SYMBOL
               | 'input' 'type' COLON SYMBOL
               | 'optional' COLON BOOLEAN
               | 'stores_in' COLON expr
```

---

## 7. Policy Declaration

```
policy_decl  ::= 'policy' SYMBOL NEWLINE INDENT policy_body DEDENT

policy_body  ::= (policy_stmt NEWLINE)*

policy_stmt  ::= allow_stmt
               | deny_stmt
               | limits_stmt

allow_stmt   ::= 'allow' NEWLINE INDENT allow_items DEDENT

allow_items  ::= (allow_item NEWLINE)*

allow_item   ::= 'tools' list
               | 'data' list
               | 'capabilities' list

deny_stmt    ::= 'deny' NEWLINE INDENT deny_items DEDENT

deny_items   ::= (deny_item NEWLINE)*

deny_item    ::= 'tools' list
               | 'data' list

limits_stmt  ::= 'limits' NEWLINE INDENT limit_items DEDENT

limit_items  ::= (limit_item NEWLINE)*

limit_item   ::= IDENT COLON (NUMBER | STRING)
```

---

## 8. Provider Declaration

```
provider_decl ::= 'provider' SYMBOL NEWLINE INDENT provider_body DEDENT

provider_body ::= (provider_stmt NEWLINE)*

provider_stmt ::= 'type' SYMBOL
                | auth_stmt
                | config_stmt
                | limits_stmt

config_stmt   ::= 'config' NEWLINE INDENT config_items DEDENT

config_items  ::= (config_item NEWLINE)*

config_item   ::= IDENT (STRING | NUMBER | SYMBOL)
```

---

## 9. Schedule Declaration

```
schedule_decl ::= 'schedule' STRING NEWLINE INDENT schedule_body DEDENT

schedule_body ::= (schedule_stmt NEWLINE)*

schedule_stmt ::= every_stmt
                | run_stmt
                | with_stmt

every_stmt    ::= 'every' (time_spec | duration)

time_spec     ::= IDENT 'at' STRING 'in' STRING

run_stmt      ::= 'run' (IDENT '.' IDENT)

with_stmt     ::= 'with' IDENT COLON value
```

---

## 10. Test Declaration

```
test_decl    ::= 'test' STRING NEWLINE INDENT test_body DEDENT

test_body    ::= (test_stmt NEWLINE)*

test_stmt    ::= given_stmt
               | when_stmt
               | expect_stmt

given_stmt   ::= 'given' NEWLINE INDENT given_items DEDENT

given_items  ::= (given_item NEWLINE)*

given_item   ::= IDENT SYMBOL
               | 'input' value
               | IDENT 'is_mock'

expect_stmt  ::= 'expect' NEWLINE INDENT expect_items DEDENT

expect_items ::= (expect_item NEWLINE)*

expect_item  ::= expr comparison expr
               | expr ('contains' | 'is') expr
               | 'calls' IDENT (NUMBER | 'once')
               | 'returns' IDENT
               | 'completes' 'within' duration
```

---

## 11. Import Declaration

```
import_decl  ::= 'import' NEWLINE INDENT import_items DEDENT
               | 'import' import_item

import_items ::= (import_item NEWLINE)*

import_item  ::= IDENT 'from' STRING
               | SYMBOL (',' SYMBOL)* 'from' STRING
               | 'agent' STRING
               | 'tool' STRING
```

---

## 12. Prompt Declaration

```
prompt_decl  ::= 'prompt' (STRING | SYMBOL) NEWLINE INDENT prompt_body DEDENT

prompt_body  ::= STRING
               | conditional_prompts

conditional_prompts ::= (conditional_prompt NEWLINE)*

conditional_prompt ::= 'when' condition NEWLINE INDENT ARROW STRING DEDENT
```

---

## 13. Expressions and Values

```
expr         ::= value
               | IDENT
               | IDENT '.' IDENT ('.' IDENT)*
               | SYMBOL
               | REFERENCE
               | call_expr
               | comparison
               | list
               | map

value        ::= STRING
               | NUMBER
               | BOOLEAN
               | SYMBOL
               | list
               | map

list         ::= '[' (value (',' value)*)? ']'

map          ::= '{' (map_entry (',' map_entry)*)? '}'

map_entry    ::= IDENT COLON value

call_expr    ::= IDENT (call_args)?

call_args    ::= IDENT COLON value
               | NEWLINE INDENT (arg_item NEWLINE)* DEDENT

arg_item     ::= IDENT COLON value

comparison   ::= expr ('==' | '!=' | '<' | '>' | '<=' | '>=') expr
               | expr RANGE expr

condition    ::= comparison
               | expr
               | IDENT '.' IDENT

action       ::= ARROW (IDENT | call_expr | REFERENCE)
```

---

## 14. Common Patterns

```
duration     ::= NUMBER ('s' | 'm' | 'h' | 'd')

size         ::= NUMBER ('kb' | 'mb' | 'gb')

network_spec ::= 'none' | 'limited' | 'full'
               | 'limited' 'to' list

filesystem_spec ::= 'none' | 'readonly' | 'full'
                  | 'readonly' list
```

---

## 15. Indentation Rules

1. **Consistent Indentation**: Use either tabs OR 4 spaces (not mixed)
2. **Block Scope**: INDENT starts a block, DEDENT ends it
3. **Nested Blocks**: Each level adds one indent
4. **Empty Lines**: Ignored (don't affect indentation)
5. **Comments**: Don't affect indentation level

---

## 16. Example Parse Tree

```a22
agent "robin"
	can chat, remember
	use model: :gpt4
```

**Parse Tree:**
```
Program
└─ AgentDeclaration
   ├─ Name: "robin"
   └─ Body
      ├─ CanStatement
      │  └─ Capabilities: [chat, remember]
      └─ UseStatement
         ├─ Target: model
         └─ Value: Symbol(:gpt4)
```

---

## 17. Parsing Strategy

### Indentation-Sensitive Lexing

The lexer must track indentation levels and emit INDENT/DEDENT tokens:

1. **Track Column**: Monitor leading whitespace on each line
2. **Indentation Stack**: Maintain stack of indentation levels
3. **Emit INDENT**: When indentation increases
4. **Emit DEDENT**: When indentation decreases (may emit multiple)
5. **Ignore Blank Lines**: Empty lines don't change indentation context

### Parser Implementation Notes

- Use recursive descent parsing
- Handle indentation via INDENT/DEDENT tokens
- Maintain symbol table for references
- Validate keyword usage during parse
- Collect imports for later resolution

---

## 18. Reserved Word List

All keywords are reserved and cannot be used as identifiers:

```
agent, tool, workflow, policy, provider, capability
can, use, do, has, is, when, needs
steps, parallel, branch, loop, break, continue, return
import, from, namespace, export
test, given, expect, show, ask
schedule, every, at, in, run
human_in_loop, options, timeout, default
state, prompt, remembers, isolation
validates, sandbox, auth, config, limits
allow, deny, primary, fallback, strategy
```

---

## 19. Error Recovery

Parsers should provide helpful error messages for common mistakes:

- Missing indentation
- Inconsistent indentation (mixing tabs/spaces)
- Invalid keyword placement
- Missing required fields
- Type mismatches
- Invalid references
- Circular dependencies
