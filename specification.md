# A22 Specification v0.1

**A Declarative Language for Functional, Immutable, Temporal Agent Systems**

**Status:** Not Stable
**License:** Apache 2.0

---

## 0. Overview

A22 is a natural-language inspired, indentation-based, functional, immutable, and temporal DSL for defining:
- Agents
- Tools
- Workflows
- Policies
- Providers
- Prompts
- Schedules
- State rules
- Tests

A22 programs describe agentic systems as **dataflow graphs over time**, not imperative code.

---

## 1. Core Semantic Model (Irreducible Primitives)

A22 has exactly **five semantic primitives**:

1. **Event**
2. **Context**
3. **Agent**
4. **Workflow**
5. **Tool**

All other constructs (state, policies, HIL, providers, schedules) are defined in terms of these.

---

### 1.1 Event

An event is the only thing that "happens" in A22:

```
event = {
  type: symbol,
  time: timestamp,
  data: map
}
```

- Events are **immutable**
- All system changes produce events
- Runtime appends events to context

Examples of event types:
- `message.incoming`
- `tool.output`
- `agent.output`
- `workflow.step`
- `hil.request`
- `hil.response`
- `policy.violation`
- `schedule.tick`

---

### 1.2 Context

Context is the entire **immutable history of events**:

```
context_t = [event_0, event_1, ..., event_t]
```

- **Append-only**
- Never mutated
- All agents, tools, workflows read only from context

**State = views over context**
(e.g., last 50 messages, current session, etc.)

---

### 1.3 Agent

Agents are **pure functions** from:

```
agent(context, input) -> event
```

- Agents do not mutate state or variables
- Agents only produce a new event
- Agents may call tools or models
- Policies may restrict allowed actions

Example:

```a22
agent "chatbot"
	can chat
	use model: :gpt4
```

---

### 1.4 Workflow

A workflow is a **temporal DAG** of pure steps.

Each step is a pure function:

```
step(context, inputs) -> event
```

Workflow execution:
1. A step runs when its inputs are available
2. Step produces an event
3. Event appended to context
4. Next steps become eligible

Built-in flow constructs:
- `steps`
- `parallel`
- `branch`
- `loop`
- `return`

---

### 1.5 Tool

Tools are pure from A22's perspective:

```
tool_call(input) -> event
```

Tools define:
- Input schema
- Output schema
- Auth
- Runtime (e.g., python, js)
- Sandbox (timeout, memory, fs, network)

Even if tools cause side effects externally, A22 sees only the tool output event.

---

## 2. Syntax Overview

A22 is **indentation-based** (tabs or 4 spaces).

### 2.1 Top-Level Declarations

```a22
agent "name"
tool "name"
workflow "name"
policy :name
provider :name
schedule "name"
prompt :name
test "name"
import
```

### 2.2 Basic Example

```a22
agent "assistant"
	can chat, search
	use model: :gpt4

	state :persistent
		remembers conversation: last 50

	when user.message
		-> respond
```

---

## 3. Agents

Agents describe autonomous behaviors as pure functions over context.

### 3.1 Structure

```a22
agent "name"
	can capability_list
	use ...               # tools, models, prompts
	has ...               # policies, resources
	state ...             # memory rules
	prompt ...            # system/user templates
	when condition
		-> action
```

### 3.2 Capabilities

Declare what an agent can do:

```a22
agent "researcher"
	can chat, search, remember, analyze
```

### 3.3 Model Configuration

#### Simple Model

```a22
agent "writer"
	use model: :gpt4
```

#### Advanced Model with Fallback

```a22
agent "resilient"
	use model
		primary :gpt4 from :openai
		fallback [:claude from :anthropic, :gemini from :google]
		strategy :failover
```

**Strategies:**
- `:failover` - Try providers in order until success
- `:cost_optimized` - Choose cheapest available
- `:latency_optimized` - Choose fastest based on history
- `:round_robin` - Distribute load evenly

### 3.4 State and Memory

State is a **projection over context**, not a mutable store.

```a22
agent "assistant"
	state :persistent
		backend :redis
		ttl 24h

	remembers
		conversation: last 50 messages
		preferences: always
		context: current_session
```

**State Backends:**
- `:memory` - In-memory (ephemeral)
- `:redis` - Redis persistence
- `:postgres` - PostgreSQL persistence
- `:custom` - Custom backend

**Remember Patterns:**
- `last N messages` - Keep N most recent items
- `always` - Persist permanently
- `current_session` - Session-scoped

### 3.5 Prompts

```a22
agent "assistant"
	prompt :system
		"You are a helpful AI assistant."

	prompt :user
		"Please respond concisely."
```

#### Conditional Prompts

```a22
agent "adaptive"
	prompt :system
		when user.expertise == "expert"
			-> "Use advanced technical explanations."
		when user.expertise == "beginner"
			-> "Use simple, clear language."
```

### 3.6 Event Handlers

```a22
agent "bot"
	when user.message
		-> respond

	when system.error
		-> log_and_notify
```

### 3.7 Complete Agent Example

```a22
agent "research_assistant"
	can search, analyze, summarize, cite_sources
	use model: :gpt4
	use tools: [web_search, arxiv_search]
	has policy: :safe_mode

	prompt :system
		"You are a research assistant that finds and analyzes academic information."

	state :persistent
		backend :redis
		ttl 86400

	remembers
		research_history: last 100 queries
		preferences: always

	when user.query
		-> .research_workflow
```

---

## 4. Tools

Tools wrap external functions/APIs as pure transformations from A22's perspective.

### 4.1 Structure

```a22
tool "name"
	endpoint "url"
	runtime :type
	auth credential_ref

	input
		field: type

	output
		field: type

	validates
		field: rules

	sandbox
		constraints
```

### 4.2 Basic Tool

```a22
tool "web_search"
	endpoint "https://api.search.com/v1"
	runtime :http
	auth env.SEARCH_KEY

	input
		query: text
		max_results: number

	output
		results: list
```

### 4.3 Input Validation

```a22
tool "send_email"
	input
		to: text
		subject: text
		body: text

	validates
		to
			pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
		subject
			max_length: 200
		body
			max_length: 10000
			deny_patterns: ["<script", "javascript:"]
```

### 4.4 Sandbox Configuration

```a22
tool "execute_code"
	sandbox
		timeout: 10s
		memory: 256mb
		network: none
		filesystem: readonly ["/tmp"]
```

**Sandbox Options:**
- `timeout` - Max execution time
- `memory` - Max memory usage
- `network` - `none`, `limited`, `full`
- `filesystem` - `none`, `readonly [paths]`, `readwrite [paths]`

### 4.5 Runtime Types

```a22
tool "python_script"
	runtime :python
	handler "scripts/analyze.py"

tool "js_function"
	runtime :js
	handler "functions/process.js"

tool "http_api"
	runtime :http
	endpoint "https://api.example.com"

tool "native_binary"
	runtime :native
	handler "/usr/bin/tool"
```

---

## 5. Workflows

Workflows orchestrate deterministic dataflow as temporal DAGs.

### 5.1 Sequential Steps

```a22
workflow "research"
	steps
		query = web_search
			query: input.topic
			max_results: 10

		summary = summarize
			content: query.results

		return summary
```

### 5.2 Parallel Execution

```a22
workflow "multi_source"
	steps
		parallel
			web = web_search query: input.topic
			arxiv = arxiv_search query: input.topic
			news = news_search query: input.topic

		combined = merge_results
			sources: [web.results, arxiv.results, news.results]

		return combined
```

### 5.3 Branching

```a22
workflow "quality_gate"
	steps
		draft = generate_content topic: input.topic

		quality = evaluate draft: draft.content

		branch quality.score
			when >8 -> publish draft
			when 5..8 -> edit_and_publish draft
			when <5 -> regenerate input.topic
```

### 5.4 Loops

```a22
workflow "iterative_improvement"
	steps
		draft = initial_draft topic: input.topic

		loop max: 3
			quality = evaluate draft: draft

			when quality.score >8
				-> break

			draft = improve
				content: draft
				feedback: quality.feedback

		return draft
```

### 5.5 Human-in-the-Loop

```a22
workflow "review_publish"
	steps
		draft = generate_article topic: input.topic

		approval = human_in_loop
			show: draft
			ask: "Approve for publishing?"
			options: [approve, reject, edit]
			timeout: 1h
			default: reject

		branch approval
			when "approve" -> publish draft
			when "reject" -> notify_rejection
			when "edit" -> .edit_workflow
```

### 5.6 Agent Calls

```a22
workflow "assisted_research"
	steps
		outline = agent "outliner"
			message: "Create an outline for: {input.topic}"

		research = agent "researcher"
			message: "Research each section"
			context: outline.content

		return research.content
```

### 5.7 Error Handling

```a22
workflow "resilient"
	steps
		result = risky_operation input: input.data

	on_failure
		retry max: 3 backoff: exponential
```

---

## 6. Policies

Policies constrain allowed behavior. They are checked before events are appended to context.

### 6.1 Structure

```a22
policy :name
	allow
		resources

	deny
		resources

	limits
		constraints
```

### 6.2 Basic Policy

```a22
policy :safe_mode
	allow
		tools [web_search, email_send]
		capabilities [chat, search]
		data [public_docs]

	deny
		tools [system_commands, file_delete]
		data [user_credentials]

	limits
		max_tokens: 10000
		max_execution_time: 30s
		max_tool_calls: 50
```

### 6.3 Policy Enforcement

When a policy violation occurs, runtime emits a `policy.violation` event and halts the operation.

```a22
agent "restricted"
	has policy: :safe_mode

	# This agent cannot use tools not in :safe_mode allow list
```

---

## 7. Providers

Providers configure external model systems.

### 7.1 Structure

```a22
provider :name
	type :category
	auth credential_ref
	config
		settings
	limits
		rate_limits
```

### 7.2 Examples

```a22
provider :openai
	type :llm
	auth env.OPENAI_KEY

	config
		endpoint "https://api.openai.com/v1"
		timeout 30s

	limits
		requests_per_minute: 60
		tokens_per_minute: 90000

provider :anthropic
	type :llm
	auth env.ANTHROPIC_KEY

	config
		endpoint "https://api.anthropic.com/v1"

	limits
		requests_per_minute: 50
```

### 7.3 Provider Types

- `:llm` - Large language models
- `:embedding` - Embedding models
- `:vision` - Vision models
- `:audio` - Audio/speech models

---

## 8. Prompts

Prompts can be declared independently and referenced.

### 8.1 Simple Prompt

```a22
prompt :system_default
	"You are a helpful AI assistant."

prompt :concise
	"Respond in 2-3 sentences maximum."
```

### 8.2 Conditional Prompts

```a22
prompt :adaptive
	when user.role == "developer"
		-> "Provide technical details and code examples."
	when user.role == "manager"
		-> "Focus on high-level summaries and business impact."
	when user.role == "student"
		-> "Explain concepts clearly with examples."
```

### 8.3 Using Prompts

```a22
agent "assistant"
	use prompt: :system_default
```

---

## 9. Human-in-the-Loop (HIL)

HIL introduces a pause for external human decision, implemented as events.

### 9.1 Structure

```a22
human_in_loop "name"
	show: expression
	ask: "question"
	options: [choices]
	timeout: duration
	default: choice
	optional: boolean
```

### 9.2 Runtime Behavior

When HIL step executes:
1. Runtime emits `hil.request` event with question and options
2. Workflow pauses
3. External system provides human input
4. Runtime emits `hil.response` event
5. Workflow continues

### 9.3 Example

```a22
workflow "content_approval"
	steps
		draft = generate_content topic: input.topic

		decision = human_in_loop
			show: draft.content
			ask: "Approve this content?"
			options: [approve, reject, revise]
			timeout: 2h
			default: reject

		branch decision
			when "approve" -> publish draft
			when "reject" -> archive draft
			when "revise" -> .revision_workflow
```

---

## 10. Scheduling

Schedules emit `schedule.tick` events at specified times.

### 10.1 Structure

```a22
schedule "name"
	trigger_spec
	run action
	with params
```

### 10.2 Interval-Based

```a22
schedule "hourly_sync"
	every 1h
	run .sync_workflow

schedule "daily_report"
	every day at "09:00" in "UTC"
	run .generate_report
```

### 10.3 Cron-Based

```a22
schedule "weekly_cleanup"
	every week on monday at "02:00"
	run .cleanup_workflow
	with
		mode: :full
```

### 10.4 Event-Based

```a22
schedule "on_new_data"
	when event "data.updated"
	run .process_data
```

---

## 11. Imports

Import declarations from other A22 files.

### 11.1 Import Specific Items

```a22
import
	agent "writer"
	tool "web_search"
	workflow "research"
	from "./library.a22"
```

### 11.2 Import with Aliases

```a22
import
	agent "writer" as "content_creator"
	tool "search" as "web_search"
	from "./external.a22"
```

### 11.3 Import All

```a22
import from "./utils.a22"
```

---

## 12. Testing

Tests verify system behavior.

### 12.1 Structure

```a22
test "name"
	given
		setup

	when
		action

	expect
		assertions
```

### 12.2 Agent Tests

```a22
test "agent responds to greeting"
	given
		agent :assistant
		input "Hello"

	expect
		response contains "Hello"
		completes within 5s
		calls model once
```

### 12.3 Workflow Tests

```a22
test "research workflow succeeds"
	given
		workflow :research
		input
			topic: "quantum computing"

	expect
		completes within 30s
		calls web_search once
		returns results
		result.summary is_not empty
```

### 12.4 Tool Tests

```a22
test "search validates input"
	given
		tool :web_search
		input
			query: ""

	expect
		fails with "validation_error"
```

---

## 13. Execution Model (Runtime Loop)

This is the formal runtime model:

```
loop:
  1. Receive input or trigger
  2. Component(context, input) → new_event
  3. Validate via policies
  4. Append event to context
  5. Trigger next steps / workflows
```

**Everything is an event.**
**Nothing mutates.**
**Time flows forward.**

### 13.1 Event Flow Example

```
Input: user.message("Hello")
  ↓
agent("chatbot", context, "Hello") → event(agent.output, "Hi there!")
  ↓
context' = context + [event]
  ↓
trigger next handlers
```

### 13.2 Context Evolution

```
context_0 = []
context_1 = [message.incoming]
context_2 = [message.incoming, agent.thinking]
context_3 = [message.incoming, agent.thinking, agent.output]
```

State views are computed from context:
```
conversation = filter(context, type="message.*")
last_50 = take(conversation, 50)
```

---

## 14. Design Principles

- **Functional** — No mutable state; pure transformations
- **Immutable** — All data is append-only
- **Temporal** — Context evolves through events over time
- **Declarative** — Users describe desired behavior, not code
- **Natural** — English-like syntax
- **Safe** — Policies and sandboxing built-in
- **Portable** — Vendor-neutral provider model
- **Deterministic** — Same context + same input = same output

---

## 15. Complete Example

```a22
# Providers
provider :openai
	type :llm
	auth env.OPENAI_KEY
	limits
		requests_per_minute: 60

# Tools
tool "web_search"
	endpoint "https://api.search.com/v1"
	runtime :http
	auth env.SEARCH_KEY

	input
		query: text
		max_results: number

	output
		results: list

	sandbox
		timeout: 10s
		network: limited

# Policies
policy :safe_mode
	allow
		tools [web_search]
		capabilities [search, analyze]

	limits
		max_tokens: 10000
		max_execution_time: 60s

# Workflows
workflow "research_topic"
	steps
		search = web_search
			query: input.topic
			max_results: 10

		analysis = agent "analyst"
			message: "Analyze these results"
			context: search.results

		approval = human_in_loop
			show: analysis.content
			ask: "Approve findings?"
			options: [approve, reject]
			timeout: 1h

		branch approval
			when "approve" -> return analysis
			when "reject" -> return "Research rejected"

# Agents
agent "research_assistant"
	can search, analyze, summarize
	use model: :gpt4
	use tools: [web_search]
	has policy: :safe_mode

	prompt :system
		"You are a research assistant that finds and analyzes information."

	state :persistent
		backend :redis
		ttl 24h

	remembers
		queries: last 100
		findings: always

	when user.query
		-> .research_topic

# Schedules
schedule "daily_digest"
	every day at "09:00" in "UTC"
	run .research_topic
	with
		topic: "AI news"

# Tests
test "research completes successfully"
	given
		workflow :research_topic
		input
			topic: "quantum computing"

	expect
		completes within 30s
		calls web_search once
		returns results
```

---

## 16. Syntax Reference

### 16.1 Symbols and References

- **Symbol**: `:name` - Named constant reference
- **Reference**: `.name` - Reference to workflow/agent/tool

### 16.2 Operators

- **Arrow**: `->` - Action binding
- **Range**: `..` - Numeric range (e.g., `5..8`)
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`

### 16.3 Data Types

- `text` - String
- `number` - Numeric
- `boolean` - True/false
- `list` - Array
- `map` - Key-value object
- `any` - Any type

### 16.4 Duration Format

- `5s` - 5 seconds
- `10m` - 10 minutes
- `2h` - 2 hours
- `1d` - 1 day

### 16.5 Memory Size Format

- `256kb` - Kilobytes
- `128mb` - Megabytes
- `2gb` - Gigabytes

---

## 17. Status & Roadmap

**Current Status:** v0.1 (Not Stable)

This specification defines the minimal, functional, immutable, temporal foundation of A22.

**Next Steps:**
- Reference runtime implementation
- Standard library of tools and agents
- Tooling (LSP, formatter, linter)
- Examples and templates
- Production deployment guides

---

## 18. License

Apache 2.0

---

**End of Specification**
