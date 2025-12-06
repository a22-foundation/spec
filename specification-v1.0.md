# A22 Specification v1.0

**A Functional, Immutable, Temporal Language for Agentic Systems**

**Status:** Stable
**License:** Apache 2.0
**Last Updated:** 2024-12-07

---

## Table of Contents

1. [Overview](#1-overview)
2. [Core Semantic Model](#2-core-semantic-model)
3. [Event System](#3-event-system)
4. [Context & Querying](#4-context--querying)
5. [Type System](#5-type-system)
6. [Agents](#6-agents)
7. [Tools](#7-tools)
8. [Workflows](#8-workflows)
9. [Multi-Agent Coordination](#9-multi-agent-coordination)
10. [Streaming & Real-Time](#10-streaming--real-time)
11. [Error Handling & Recovery](#11-error-handling--recovery)
12. [Policies & Security](#12-policies--security)
13. [Providers](#13-providers)
14. [Human-in-the-Loop](#14-human-in-the-loop)
15. [Scheduling](#15-scheduling)
16. [Standard Library](#16-standard-library)
17. [Observability](#17-observability)
18. [Versioning & Migration](#18-versioning--migration)
19. [Deployment](#19-deployment)
20. [Testing](#20-testing)
21. [Runtime Model](#21-runtime-model)
22. [Complete Examples](#22-complete-examples)
23. [Syntax Reference](#23-syntax-reference)

---

## 1. Overview

A22 is a **declarative, functional, immutable, temporal language** for defining agentic systems as **pure dataflow graphs over time**.

### 1.1 Core Philosophy

A22 treats all system behavior as:
- **Events** flowing through time
- **Pure functions** transforming inputs to outputs
- **Immutable context** preserving complete history
- **Deterministic execution** guaranteeing reproducibility

### 1.2 Design Principles

1. **Functional** — Pure functions, no side effects in core logic
2. **Immutable** — Append-only events, never mutate
3. **Temporal** — Context evolves through time
4. **Declarative** — Describe intent, not execution
5. **Natural** — Reads like structured thought
6. **Safe** — Policies, sandboxing, validation
7. **Portable** — Vendor-neutral specification
8. **Deterministic** — Same input → same output
9. **Composable** — Build complex from simple
10. **Auditable** — Every action is an event

### 1.3 Indentation-Based Syntax

A22 uses Python/YAML-style indentation (tabs or 4 spaces):

```a22
agent "assistant"
	can chat, search
	use model: :gpt4

	when user.message
		-> respond
```

---

## 2. Core Semantic Model

A22 has exactly **five irreducible primitives**:

### 2.1 Event

```
event = {
  type: qualified_name,
  time: timestamp_utc,
  agent_id: string,
  correlation_id: uuid,
  causation_id: uuid,
  metadata: {
    version: "1.0",
    source: agent_or_tool_id,
    tags: [string]
  },
  data: map,
  signature: optional_hash
}
```

Events are:
- **Immutable** — Never change after creation
- **Timestamped** — ISO 8601 with microsecond precision
- **Causally linked** — correlation_id and causation_id track relationships
- **Verifiable** — Optional cryptographic signature

### 2.2 Context

```
context_t = [event_0, event_1, ..., event_t]
```

Context is:
- **Append-only** — Events only added, never removed or modified
- **Shared** — All components read from same context
- **The source of truth** — Everything derives from events
- **Queryable** — Supports filtering, projection, aggregation

### 2.3 Agent

```
agent(context, input) -> event
```

Agents are pure functions that:
- Read only from context (immutable)
- Take input
- Produce a new event
- Cannot mutate state

### 2.4 Workflow

```
workflow(context, input) -> [event]
```

Workflows are temporal DAGs where:
- Each step is a pure function
- Steps run when inputs available
- Steps produce events
- Events trigger next steps

### 2.5 Tool

```
tool(input) -> event
```

Tools are externalized side effects, pure from A22's perspective:
- Input → Tool execution → Output event
- Side effects happen "outside" the functional core
- Events are what enter the context

---

## 3. Event System

### 3.1 Standard Event Types

#### 3.1.1 Core Events

```a22
# Message events
message.incoming        # User input received
message.outgoing        # Agent output sent

# Agent events
agent.invoked          # Agent called
agent.thinking         # Agent processing (optional observability)
agent.output           # Agent produced result
agent.error            # Agent execution failed

# Tool events
tool.invoked           # Tool called
tool.output            # Tool completed successfully
tool.error             # Tool execution failed

# Workflow events
workflow.started       # Workflow began
workflow.step          # Step executed
workflow.completed     # Workflow finished successfully
workflow.failed        # Workflow error
```

#### 3.1.2 Control Events

```a22
# Human-in-loop
hil.request            # Human input needed
hil.response           # Human provided decision
hil.timeout            # Human response timed out

# Policy events
policy.checked         # Policy validation occurred
policy.violated        # Policy validation failed

# Schedule events
schedule.triggered     # Scheduled event fired
```

#### 3.1.3 State Events

```a22
state.snapshot         # Periodic state checkpoint
context.compacted      # Context optimization occurred
session.started        # New session began
session.ended          # Session closed
```

#### 3.1.4 Error Events

```a22
error.validation       # Input validation failed
error.execution        # Execution error
error.timeout          # Operation timed out
error.resource         # Resource limit exceeded
error.policy           # Policy violation
error.network          # Network error
error.authentication   # Auth failure
```

### 3.2 Event Schema Definition

Define custom event types:

```a22
event_type "analysis.completed"
	schema
		confidence: number
		categories: list<string>
		metadata: map
		processing_time_ms: number

	derives_from: "tool.output"

	validates
		confidence
			range: 0.0..1.0
		categories
			min_length: 1
```

### 3.3 Event Correlation

Track related events:

```a22
# Events automatically linked via correlation_id
event {
  type: "workflow.started",
  correlation_id: "abc-123",
  causation_id: "message-456",
  ...
}

event {
  type: "tool.invoked",
  correlation_id: "abc-123",  # Same workflow
  causation_id: "workflow-step-789",
  ...
}
```

Query correlated events:

```a22
query context
	correlation_id: event.correlation_id
	order_by time asc
```

---

## 4. Context & Querying

### 4.1 Query Language

Filter and retrieve events from context:

```a22
# Basic filtering
query context
	filter type: "message.*"
	limit 50

# Conditional filtering
query context
	filter type: "agent.output"
	where data.confidence > 0.8
	order_by time desc
	limit 10

# Temporal queries
query context
	filter type: "tool.*"
	between 2024-01-01T00:00:00Z and 2024-01-31T23:59:59Z

# Correlation queries
query context
	correlation_id: workflow.correlation_id
	order_by time asc

# Complex queries
query context
	filter type: "error.*"
	where data.severity == "critical"
	and agent_id == "production_agent"
	group_by type
	limit 100
```

### 4.2 Projections

Compute state as views over context:

```a22
projection "conversation_history"
	from context
	filter type: "message.*"

	transform
		user_messages = filter(events, type: "message.incoming")
		agent_messages = filter(events, type: "message.outgoing")
		interleaved = interleave(user_messages, agent_messages)

	output
		messages: interleaved
		count: len(messages)
		last_updated: max(events, by: time)
		participants: unique(events, by: data.user_id)

# Use projection in agent
agent "chatbot"
	state :memory
		conversation = projection("conversation_history")
		recent = take(conversation.messages, 50)
```

### 4.3 Aggregations

Compute summaries from events:

```a22
projection "performance_metrics"
	from context
	filter type: "workflow.completed"

	aggregate
		total_workflows: count(events)
		avg_duration: average(events, field: "data.duration_ms")
		p95_duration: percentile(events, field: "data.duration_ms", p: 95)
		success_rate: count(events where data.status == "success") / count(events)
```

### 4.4 Context Compaction

Manage context size while preserving semantics:

```a22
compaction_policy
	when context.size > 1gb
	strategy :snapshot_and_truncate
		keep_recent: 10000 events
		snapshot_every: 1000 events
		preserve_snapshots: 10

		# Never compact critical events
		preserve_events
			types: [error.*, policy.violated, audit.*]

		# Compact low-priority events aggressively
		compact_events
			types: [debug.*, trace.*]
			ttl: 1d
```

---

## 5. Type System

### 5.1 Basic Types

```a22
# Primitive types
text                   # String
number                 # Numeric (int or float)
boolean                # true/false
timestamp              # ISO 8601 datetime
duration               # Time interval (5s, 10m, 2h)
uuid                   # UUID string

# Collection types
list<T>                # Array of type T
map<K, V>              # Key-value map
set<T>                 # Unique set of T
optional<T>            # T or null

# Special types
any                    # Any type (use sparingly)
never                  # No valid value (for exhaustiveness)
```

### 5.2 Structured Types

Define custom data structures:

```a22
type UserProfile
	fields
		id: uuid
		name: text
		email: text
		preferences: map<text, any>
		created_at: timestamp
		verified: boolean

	validates
		email
			pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
		name
			min_length: 1
			max_length: 100

type SearchResult
	fields
		url: text
		title: text
		snippet: text
		relevance_score: number
		timestamp: timestamp

	validates
		relevance_score
			range: 0.0..1.0
		url
			pattern: "^https?://.*"

# Use in declarations
tool "user_lookup"
	input
		user_id: uuid
	output
		profile: UserProfile

tool "search"
	input
		query: text
		max_results: number
	output
		results: list<SearchResult>
```

### 5.3 Union Types

Support multiple possible types:

```a22
type Result<T, E>
	union
		success: T
		failure: E

type AnalysisOutcome
	union
		high_confidence
			result: text
			confidence: number
		low_confidence
			suggestions: list<text>
			reasons: list<text>
		error
			error_code: text
			error_message: text

# Pattern matching
workflow "handle_analysis"
	steps
		outcome = analyze input

		branch outcome
			when outcome is high_confidence
				-> publish outcome.result
			when outcome is low_confidence
				-> request_human_review outcome.suggestions
			when outcome is error
				-> log_error outcome.error_message
```

### 5.4 Generic Types

Parameterized types:

```a22
type Result<T, E>
	union
		ok: T
		error: E

type List<T>
	items: [T]
	length: number

type Pair<A, B>
	fields
		first: A
		second: B

# Usage
tool "safe_operation"
	input
		data: text
	output
		result: Result<ProcessedData, ValidationError>

workflow "map_reduce"
	input
		items: List<InputItem>
	output
		result: List<OutputItem>
```

### 5.5 Type Aliases

Create readable type names:

```a22
type UserId = uuid
type Email = text
type Timestamp = timestamp

type UserMap = map<UserId, UserProfile>
type EmailList = list<Email>
```

---

## 6. Agents

### 6.1 Basic Agent

```a22
agent "assistant"
	can chat, search, remember
	use model: :gpt4
	use tools: [web_search]

	prompt :system
		"You are a helpful AI assistant."

	when user.message
		-> respond
```

### 6.2 Capabilities

Declare what an agent can do:

```a22
agent "researcher"
	can search, analyze, summarize, cite_sources, fact_check

	# Capabilities checked by policies
	# Can be queried by other agents
```

### 6.3 Model Configuration

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

	# Strategies:
	# :failover - Try in order until success
	# :cost_optimized - Choose cheapest
	# :latency_optimized - Choose fastest
	# :round_robin - Distribute load
```

### 6.4 State Management

State as projections over context:

```a22
agent "assistant"
	state :persistent
		backend :redis
		ttl 24h

	remembers
		conversation: last 50 messages
		preferences: always
		session_context: current_session
		user_facts: last 100 facts

# State is computed from events, not stored mutably
# Runtime queries context to build state views
```

### 6.5 Multi-Prompt System

```a22
agent "adaptive"
	prompt :system
		when user.expertise == "expert"
			-> "Provide technical details and code examples."
		when user.expertise == "beginner"
			-> "Use simple, clear language with examples."
		otherwise
			-> "Balance technical depth with clarity."

	prompt :user_template
		"Please analyze: {{input.text}}"
```

### 6.6 Agent Roles

Define agent responsibilities in multi-agent systems:

```a22
agent "supervisor"
	role :coordinator
	can delegate, review, approve, escalate

	delegates_to
		agents: [researcher, writer, editor]
		max_concurrent: 3
		strategy: :load_balanced

	when task.assigned
		-> .delegate_workflow

agent "researcher"
	role :worker
	reports_to: supervisor
	can search, analyze, summarize

	when task.delegated
		-> .research_task
```

### 6.7 Complete Agent Example

```a22
agent "research_assistant"
	can search, analyze, summarize, cite_sources
	use model
		primary :gpt4 from :openai
		fallback [:claude from :anthropic]
		strategy :cost_optimized

	use tools: [web_search, arxiv_search, citation_tool]
	has policy: :safe_research_mode

	prompt :system
		"You are a research assistant that finds and analyzes academic information.
		Always cite sources and verify facts."

	state :persistent
		backend :redis
		ttl 86400

	remembers
		research_history: last 100 queries
		findings: last 200 facts
		preferences: always

	when user.query
		-> .research_workflow

	when user.follow_up
		-> .follow_up_workflow
```

---

## 7. Tools

### 7.1 Basic Tool

```a22
tool "web_search"
	endpoint "https://api.search.com/v1"
	runtime :http
	auth env.SEARCH_KEY

	input
		query: text
		max_results: number
		filters: optional<map<text, any>>

	output
		results: list<SearchResult>
		total_found: number
		search_time_ms: number

	sandbox
		timeout: 10s
		network: limited
		rate_limit: 100/min
```

### 7.2 Input Validation

```a22
tool "send_email"
	input
		to: text
		subject: text
		body: text
		attachments: optional<list<text>>

	validates
		to
			pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
		subject
			min_length: 1
			max_length: 200
			deny_patterns: ["<script", "javascript:"]
		body
			max_length: 100000
			deny_patterns: ["<script", "javascript:", "data:"]
		attachments
			max_items: 10
			each
				max_size: 10mb
```

### 7.3 Sandbox Configuration

```a22
tool "execute_code"
	runtime :python
	handler "sandbox/execute.py"

	sandbox
		timeout: 30s
		memory: 512mb
		cpu_limit: 1core
		network: none
		filesystem
			readonly: ["/app/lib"]
			readwrite: ["/tmp"]
			deny: ["/etc", "/root"]

		environment
			PATH: "/usr/local/bin:/usr/bin"
			PYTHONPATH: "/app/lib"

		resource_limits
			max_processes: 10
			max_file_descriptors: 100
```

### 7.4 Runtime Types

```a22
tool "python_analyzer"
	runtime :python
	handler "analyzers/sentiment.py"
	version "3.11"

tool "js_processor"
	runtime :javascript
	handler "processors/transform.js"
	version "node:20"

tool "http_api"
	runtime :http
	endpoint "https://api.example.com/v1/process"
	method POST
	headers
		Content-Type: "application/json"
		Authorization: "Bearer {{env.API_KEY}}"

tool "native_binary"
	runtime :native
	handler "/usr/local/bin/custom-tool"
	args: ["--mode", "production"]

tool "docker_container"
	runtime :container
	image: "my-org/analyzer:v1.2"
	command: ["python", "analyze.py"]
```

### 7.5 Tool with Types

```a22
type AnalysisInput
	fields
		text: text
		language: text
		options: map<text, boolean>

type AnalysisOutput
	fields
		sentiment: text
		confidence: number
		entities: list<Entity>
		keywords: list<text>

tool "advanced_analysis"
	input: AnalysisInput
	output: Result<AnalysisOutput, AnalysisError>

	runtime :python
	handler "ml/analyze.py"

	sandbox
		timeout: 30s
		memory: 1gb
		gpu: optional
```

---

## 8. Workflows

### 8.1 Sequential Steps

```a22
workflow "research"
	steps
		search = web_search
			query: input.topic
			max_results: 10

		analysis = agent "analyst"
			message: "Analyze these search results"
			context: search.results

		summary = summarize
			content: analysis.output

		return summary
```

### 8.2 Parallel Execution

```a22
workflow "multi_source_research"
	steps
		parallel
			web = web_search query: input.topic
			arxiv = arxiv_search query: input.topic
			news = news_search query: input.topic

		# All three run concurrently
		# Next step waits for all to complete

		synthesis = agent "synthesizer"
			sources: [web.results, arxiv.results, news.results]

		return synthesis
```

### 8.3 Branching

```a22
workflow "quality_gate"
	steps
		draft = generate_content topic: input.topic
		quality = evaluate_quality draft: draft.content

		branch quality.score
			when >8
				-> publish draft
			when 5..8
				-> improve_and_publish draft
			when <5
				-> regenerate_with_feedback
					topic: input.topic
					feedback: quality.issues

		return result
```

### 8.4 Loops

```a22
workflow "iterative_improvement"
	steps
		draft = initial_draft topic: input.topic
		iteration = 0

		loop max: 5
			iteration = iteration + 1

			quality = evaluate draft: draft

			# Break conditions
			when quality.score >8
				-> break

			when iteration >=5
				-> break

			# Improve and continue
			draft = improve
				content: draft
				feedback: quality.feedback

		return draft
```

### 8.5 Error Handling

```a22
workflow "resilient_process"
	steps
		result = risky_operation input: input.data

	on_error
		when error.type == "error.timeout"
			retry
				max_attempts: 3
				backoff: exponential
				initial_delay: 1s
				max_delay: 30s

		when error.type == "error.validation"
			fallback
				use_default_value()

		when error.type == "error.resource"
			circuit_breaker
				failure_threshold: 5
				timeout: 30s
				half_open_after: 60s

		otherwise
			compensate
				rollback_steps: [previous_operation]
				notify: admin
				log_error: error
```

### 8.6 Saga Pattern

Long-running distributed transactions:

```a22
workflow "distributed_transaction"
	saga
		steps
			inventory = reserve_inventory
				item_id: input.item_id
				quantity: input.quantity

			payment = charge_payment
				user_id: input.user_id
				amount: input.amount

			shipment = schedule_shipment
				order_id: input.order_id
				address: input.address

		compensations
			inventory -> release_inventory inventory.reservation_id
			payment -> refund_payment payment.transaction_id
			shipment -> cancel_shipment shipment.shipment_id

		on_failure
			rollback_from: failed_step
			notification: admin.email
```

### 8.7 Human-in-Loop Integration

```a22
workflow "content_approval"
	steps
		draft = generate_article topic: input.topic

		approval = human_in_loop
			show: draft.content
			ask: "Approve for publishing?"
			options: [approve, reject, revise]
			timeout: 2h
			default: reject

		branch approval
			when "approve"
				-> publish draft
			when "reject"
				-> archive draft
			when "revise"
				-> .revision_workflow draft

		return result
```

---

## 9. Multi-Agent Coordination

### 9.1 Agent Communication

```a22
agent "researcher"
	can search, analyze

	do
		findings = search_web topic: input.query

		# Send to another agent
		analysis = agent "analyst"
			message: "Please analyze these findings"
			context: findings
			wait_for: response

		return analysis

# Analyst receives via event
agent "analyst"
	when message.from_agent
		-> analyze_and_respond
```

### 9.2 Multi-Agent Workflows

```a22
workflow "collaborative_research"
	steps
		# Parallel agent execution
		parallel
			web_research = agent "web_researcher"
				task: input.topic
				focus: "current news and trends"

			academic_research = agent "academic_researcher"
				task: input.topic
				focus: "peer-reviewed papers"

			industry_research = agent "industry_researcher"
				task: input.topic
				focus: "industry reports and analysis"

		# Coordinator synthesizes
		synthesis = agent "synthesizer"
			role: coordinator
			sources: [web_research, academic_research, industry_research]
			task: "Create comprehensive report"

		# Editor polishes
		final = agent "editor"
			content: synthesis.output
			task: "Polish and format"

		return final
```

### 9.3 Delegation Pattern

```a22
agent "supervisor"
	role :coordinator
	can delegate, monitor, approve

	workflow "delegate_task"
		steps
			# Analyze task
			breakdown = analyze_task input.task

			# Assign to workers
			parallel
				for subtask in breakdown.subtasks
					result = delegate_to_worker
						agent: select_best_agent(subtask.type)
						task: subtask
						deadline: subtask.deadline

			# Review results
			review = review_results
				results: parallel.results

			# Approve or request revisions
			decision = evaluate_quality review

			branch decision
				when decision.approved
					-> finalize_and_deliver
				when decision.needs_revision
					-> request_revisions decision.feedback

			return result

agent "worker"
	role :worker
	reports_to: supervisor

	when task.assigned
		-> execute_assigned_task
```

### 9.4 Agent Discovery

```a22
workflow "dynamic_collaboration"
	steps
		# Find agents with required capabilities
		analysts = discover_agents
			capabilities: [analyze, summarize]
			availability: online
			max_load: <80%

		# Use discovered agents
		parallel
			for agent in analysts
				result = invoke_agent
					agent: agent
					task: input.subtask

		return results
```

---

## 10. Streaming & Real-Time

### 10.1 Event Streams

```a22
workflow "real_time_monitoring"
	input_stream: sensor_events
		filter type: "sensor.reading"

	steps
		stream
			reading = next_event()

			# Immediate action on threshold
			when reading.data.temperature > 100
				-> alert_critical reading

			# Windowed aggregation
			stats = window
				size: 100 events
				slide: 10 events
				compute
					avg_temp: average(events, field: "data.temperature")
					max_temp: max(events, field: "data.temperature")
					event_count: count(events)

			when stats.avg_temp > 80
				-> alert_warning stats
```

### 10.2 Time Windows

```a22
workflow "time_based_analysis"
	input_stream: activity_events

	steps
		stream
			# Tumbling window (non-overlapping)
			hourly_stats = tumbling_window
				duration: 1h
				compute
					total_events: count(events)
					unique_users: count_distinct(events, field: "user_id")
					avg_duration: average(events, field: "duration_ms")

			# Sliding window (overlapping)
			moving_avg = sliding_window
				duration: 5m
				slide: 1m
				compute
					avg_value: average(events, field: "value")

			# Session window (gap-based)
			sessions = session_window
				gap: 30m
				compute
					session_duration: max(time) - min(time)
					actions_per_session: count(events)
```

### 10.3 Reactive Agents

```a22
agent "anomaly_detector"
	watches
		stream: "metrics.performance"
		window: 5m

	when stream.anomaly_detected
		-> investigate_and_alert

	workflow "investigate_and_alert"
		steps
			# Get recent context
			recent_events = query context
				filter type: "metrics.performance"
				last: 1000 events

			# Analyze
			analysis = analyze_anomaly
				current: stream.current_window
				historical: recent_events

			# Alert if severe
			when analysis.severity == "critical"
				-> send_alert analysis
```

### 10.4 Stream Processing

```a22
workflow "event_processing_pipeline"
	input_stream: raw_events

	steps
		stream
			# Filter
			filtered = filter_events
				condition: event.data.priority == "high"

			# Transform
			transformed = transform_event
				event: filtered
				schema: StandardSchema

			# Enrich
			enriched = enrich_event
				event: transformed
				lookup: user_database

			# Aggregate
			aggregated = aggregate
				group_by: event.data.category
				window: 1m
				compute
					count: count(events)
					sum_value: sum(events, field: "value")

			# Output
			when aggregated.count > threshold
				-> emit_alert aggregated
```

---

## 11. Error Handling & Recovery

### 11.1 Error Events

Errors are events in the system:

```a22
# Error event structure
error_event {
  type: "error.execution",
  time: timestamp,
  data: {
    error_code: "TIMEOUT",
    error_message: "Tool execution exceeded 30s",
    error_category: "resource",
    severity: "warning" | "error" | "critical",
    stack_trace: optional,
    context_snapshot: optional,
    recovery_suggestions: [actions],
    failed_component: {
      type: "tool" | "agent" | "workflow",
      id: component_id,
      step: optional_step_id
    }
  }
}
```

### 11.2 Retry Strategies

```a22
workflow "resilient_operation"
	steps
		result = unstable_operation input: data

	on_error
		retry
			max_attempts: 5
			backoff: exponential
			initial_delay: 1s
			max_delay: 60s
			multiplier: 2
			jitter: 0.1

			# Only retry specific errors
			retry_on: [error.timeout, error.network, error.rate_limit]

			# Don't retry these
			give_up_on: [error.validation, error.authentication]
```

### 11.3 Fallback Pattern

```a22
workflow "with_fallbacks"
	steps
		result = primary_operation input: data

	on_error
		fallback_chain
			# Try primary
			-> primary_operation input: data

			# If fails, try secondary
			on_error -> secondary_operation input: data

			# If fails, try cached
			on_error -> cached_result input: data.cache_key

			# If all fail, use default
			on_error -> default_result input: data.type
```

### 11.4 Circuit Breaker

```a22
workflow "protected_operation"
	steps
		result = fragile_service input: data

	on_error
		circuit_breaker
			failure_threshold: 5        # Open after 5 failures
			success_threshold: 2        # Close after 2 successes
			timeout: 30s                # Stay open for 30s
			half_open_max_calls: 3      # Test with 3 calls when half-open

			# What to do when circuit is open
			on_open
				-> fallback_service input: data
```

### 11.5 Compensation (Saga Pattern)

```a22
workflow "complex_transaction"
	saga
		steps
			step1 = operation1 input: data
			step2 = operation2 input: step1.result
			step3 = operation3 input: step2.result

		compensations
			step1 -> compensate_operation1 step1.transaction_id
			step2 -> compensate_operation2 step2.transaction_id
			step3 -> compensate_operation3 step3.transaction_id

		on_failure
			# Rollback all completed steps in reverse order
			rollback_from: failed_step
			rollback_order: reverse

			# Log compensation
			log_compensation: true

			# Notify on failure
			notification
				channel: :email
				recipient: admin
				include: compensation_log
```

### 11.6 Deadline Propagation

```a22
workflow "time_bounded"
	deadline: 5m

	steps
		step1 = operation1 input: data
			deadline: remaining_time * 0.3

		step2 = operation2 input: step1.result
			deadline: remaining_time * 0.5

		step3 = operation3 input: step2.result
			deadline: remaining_time

	on_timeout
		-> partial_result_handler
			completed_steps: completed
			remaining_steps: remaining
```

---

## 12. Policies & Security

### 12.1 Basic Policy

```a22
policy :safe_mode
	allow
		tools: [web_search, email_send, calendar_access]
		capabilities: [chat, search, schedule]
		data_access
			level: :public, :internal
			not: :confidential, :secret

	deny
		tools: [system_commands, file_delete, db_write]
		operations
			write_to: [production_db]
			delete_from: [any]
		data_export
			to: external_networks

	limits
		rate
			requests_per_second: 10
			tokens_per_minute: 100000
		resource
			max_memory: 2gb
			max_execution_time: 300s
			max_concurrent_workflows: 5
		cost
			max_usd_per_hour: 10
			max_usd_per_day: 100
```

### 12.2 Data Privacy (GDPR)

```a22
privacy
	pii_fields
		mark
			user.email: :email
			user.name: :name
			user.phone: :phone
			user.location: :geo_location
			user.ip_address: :ip_address

	retention
		events_with_pii
			ttl: 90d
			after_deletion: anonymize

		audit_events
			ttl: 7y
			immutable: true

	right_to_be_forgotten
		on_request: user_id
			delete_all_events
				where data.user_id == user_id
			anonymize_events
				where metadata.mentions_user_id == user_id
			create_deletion_certificate
				user_id: user_id
				timestamp: now()
				events_deleted: count

	consent
		required_for: [email, phone, location]
		store_consent_events: true
		validate_consent_before: data_processing
```

### 12.3 Audit Trail

```a22
audit
	enabled: true

	events
		include: all
		exclude: [debug.*, trace.*]

		# Always audit these
		critical_events: [
			policy.violated,
			error.critical,
			admin.action,
			data.exported,
			permission.changed
		]

	integrity
		hash_algorithm: :sha256
		chain_events: true
		periodic_checkpoint: 1000 events
		tamper_detection: enabled

	storage
		backend: :append_only_log
		replicas: 3
		geo_distributed: true
		retention: 7y
		encryption_at_rest: true

	compliance
		standards: [soc2, gdpr, hipaa, pci_dss]
		export_format: :cef
		real_time_monitoring: true
```

### 12.4 Input Sanitization

```a22
policy :input_validation
	sanitize
		text_inputs
			max_length: 10000
			deny_patterns: [
				"<script",
				"javascript:",
				"data:text/html",
				"onerror=",
				"onclick="
			]
			strip_html: true
			normalize_unicode: true

		file_uploads
			max_size: 10mb
			allowed_types: [".pdf", ".txt", ".docx"]
			scan_for_malware: true

		urls
			allowed_protocols: ["https"]
			deny_domains: [blocked_domains]
			validate_ssl: true
```

### 12.5 Sandbox Policies

```a22
policy :strict_sandbox
	sandbox
		default_timeout: 10s
		default_memory: 256mb
		default_network: none

		allowed_binaries: [
			"/usr/bin/python3",
			"/usr/bin/node"
		]

		denied_system_calls: [
			"fork", "exec", "kill", "ptrace"
		]

		filesystem
			readonly: ["/usr", "/lib"]
			readwrite: ["/tmp"]
			deny: ["/etc", "/root", "/home"]

		resource_limits
			max_processes: 5
			max_file_descriptors: 50
			max_disk_usage: 100mb
```

---

## 13. Providers

### 13.1 LLM Provider

```a22
provider :openai
	type :llm
	auth env.OPENAI_API_KEY

	config
		endpoint "https://api.openai.com/v1"
		timeout 60s
		retry_policy
			max_attempts: 3
			backoff: exponential

	limits
		requests_per_minute: 3500
		tokens_per_minute: 90000
		tokens_per_day: 10000000

	models
		:gpt4
			name: "gpt-4-turbo"
			context_window: 128000
			max_tokens: 4096
		:gpt35
			name: "gpt-3.5-turbo"
			context_window: 16385
			max_tokens: 4096
```

### 13.2 Multiple Providers

```a22
provider :anthropic
	type :llm
	auth env.ANTHROPIC_API_KEY

	config
		endpoint "https://api.anthropic.com/v1"
		timeout 60s

	limits
		requests_per_minute: 1000
		tokens_per_minute: 100000

	models
		:claude
			name: "claude-3-opus-20240229"
			context_window: 200000
			max_tokens: 4096

provider :google
	type :llm
	auth env.GOOGLE_API_KEY

	models
		:gemini
			name: "gemini-pro"
			context_window: 32000
```

### 13.3 Embedding Provider

```a22
provider :openai_embeddings
	type :embedding
	auth env.OPENAI_API_KEY

	config
		endpoint "https://api.openai.com/v1/embeddings"

	models
		:text_embed_3
			name: "text-embedding-3-large"
			dimensions: 3072
```

### 13.4 Custom Provider

```a22
provider :custom_llm
	type :llm

	config
		endpoint "https://custom-llm.company.com/v1"
		auth
			type: :bearer_token
			token: env.CUSTOM_TOKEN

		headers
			X-Custom-Header: "value"

		request_format: :openai_compatible
		response_format: :openai_compatible
```

---

## 14. Human-in-the-Loop

### 14.1 Basic HIL

```a22
workflow "approval_flow"
	steps
		draft = generate_content topic: input.topic

		decision = human_in_loop
			show: draft.content
			ask: "Approve this draft?"
			options: [approve, reject, revise]
			timeout: 2h
			default: reject

		branch decision
			when "approve" -> publish draft
			when "reject" -> archive draft
			when "revise" -> .revision_workflow draft
```

### 14.2 Rich HIL

```a22
human_in_loop "review_analysis"
	show
		title: "Analysis Results"
		content: analysis.output
		visualizations: analysis.charts
		metadata
			confidence: analysis.confidence
			sources: analysis.sources

	ask: "How should we proceed?"

	options
		approve
			label: "Approve and Continue"
			style: :primary

		request_more_data
			label: "Request More Data"
			style: :secondary
			requires_input: text
			prompt: "What additional data do you need?"

		reject
			label: "Reject Analysis"
			style: :danger
			requires_input: text
			prompt: "Why are you rejecting this?"

	timeout: 4h
	default: reject

	notifications
		channels: [:email, :slack]
		recipients: [admin, assigned_reviewer]
		reminder_after: 1h
```

### 14.3 Multi-Stage HIL

```a22
workflow "multi_review_process"
	steps
		draft = generate_proposal topic: input.topic

		# First review
		initial_review = human_in_loop
			role: :reviewer
			show: draft
			ask: "Initial review decision?"
			options: [approve_for_manager, request_changes, reject]

		branch initial_review
			when "approve_for_manager"
				# Manager approval
				manager_review = human_in_loop
					role: :manager
					show: draft
					context: initial_review.feedback
					ask: "Final approval?"
					options: [approve, send_back_to_reviewer, reject]
					timeout: 24h

				branch manager_review
					when "approve" -> finalize_and_publish draft
					when "send_back_to_reviewer" -> .reviewer_workflow
					when "reject" -> archive_with_reason

			when "request_changes"
				-> .revision_workflow draft, initial_review.requested_changes

			when "reject"
				-> archive_with_reason initial_review.reason
```

---

## 15. Scheduling

### 15.1 Interval-Based

```a22
schedule "hourly_sync"
	every 1h
	run .sync_workflow
	with
		mode: :incremental

schedule "daily_report"
	every 1d at "09:00" in "America/New_York"
	run .generate_daily_report
	with
		recipients: [admin, manager]
		include_metrics: true
```

### 15.2 Cron-Based

```a22
schedule "weekly_cleanup"
	cron "0 2 * * 1"  # Every Monday at 2 AM
	run .cleanup_workflow
	with
		mode: :full
		notify_on_completion: true

schedule "monthly_archive"
	cron "0 0 1 * *"  # First day of month at midnight
	run .archive_old_data
	with
		retention_days: 90
```

### 15.3 Event-Based

```a22
schedule "on_data_arrival"
	when event "data.updated"
	run .process_new_data
	with
		priority: :high

schedule "on_error_spike"
	when
		query context
			filter type: "error.*"
			window: 5m
			having count(*) > 100
	run .investigate_errors
```

### 15.4 Complex Scheduling

```a22
schedule "business_hours_monitoring"
	when
		time_range: "09:00-17:00"
		timezone: "America/New_York"
		days: [monday, tuesday, wednesday, thursday, friday]
	run .active_monitoring

	paused_during
		holidays: us_federal_holidays
		maintenance_windows: scheduled_maintenance

schedule "conditional_report"
	every 1h
	when
		query context
			filter type: "transaction.completed"
			window: 1h
			having sum(data.amount) > 10000
	run .high_value_report
```

---

## 16. Standard Library

### 16.1 Core Tools

```a22
# HTTP Client
tool stdlib.http_get
	input
		url: text
		headers: optional<map<text, text>>
		timeout: optional<duration>
	output
		status: number
		body: text
		headers: map<text, text>

tool stdlib.http_post
	input
		url: text
		body: text
		headers: optional<map<text, text>>
	output
		status: number
		body: text

# JSON Processing
tool stdlib.json_parse
	input
		json_string: text
	output
		parsed: map<text, any>

tool stdlib.json_stringify
	input
		data: any
		pretty: optional<boolean>
	output
		json: text

# Text Processing
tool stdlib.text_extract
	input
		text: text
		pattern: text  # regex
	output
		matches: list<text>

tool stdlib.text_replace
	input
		text: text
		pattern: text
		replacement: text
	output
		result: text

# Time/Date
tool stdlib.now
	output
		timestamp: timestamp

tool stdlib.format_time
	input
		timestamp: timestamp
		format: text
		timezone: optional<text>
	output
		formatted: text

# Crypto
tool stdlib.hash
	input
		data: text
		algorithm: text  # sha256, sha512, md5
	output
		hash: text

tool stdlib.uuid
	output
		uuid: uuid
```

### 16.2 Core Agents

```a22
agent stdlib.classifier
	can classify

	input
		text: text
		categories: list<text>
		examples: optional<map<text, list<text>>>
	output
		category: text
		confidence: number
		reasoning: optional<text>

agent stdlib.summarizer
	can summarize

	input
		text: text
		max_length: optional<number>
		style: optional<text>  # brief, detailed, bullet_points
	output
		summary: text
		key_points: list<text>

agent stdlib.translator
	can translate

	input
		text: text
		source_language: text
		target_language: text
	output
		translated: text
		confidence: number

agent stdlib.sentiment_analyzer
	can analyze_sentiment

	input
		text: text
	output
		sentiment: text  # positive, negative, neutral
		confidence: number
		aspects: optional<list<SentimentAspect>>
```

### 16.3 Core Workflows

```a22
workflow stdlib.retry_with_backoff
	input
		operation: callable
		max_attempts: number
		backoff: :exponential | :linear | :constant
		initial_delay: duration

	steps
		attempt = 0

		loop max: input.max_attempts
			attempt = attempt + 1

			result = try operation()

			when result.success
				-> break

			delay = calculate_backoff
				strategy: input.backoff
				attempt: attempt
				initial: input.initial_delay

			wait delay

		return result

workflow stdlib.map_parallel
	input
		items: list<any>
		operation: callable
		max_concurrency: optional<number>

	steps
		results = []

		parallel max: input.max_concurrency ?? 10
			for item in input.items
				result = operation item
				results.append(result)

		return results

workflow stdlib.reduce
	input
		items: list<any>
		operation: callable
		initial: any

	steps
		accumulator = input.initial

		for item in input.items
			accumulator = operation accumulator, item

		return accumulator
```

---

## 17. Observability

### 17.1 Distributed Tracing

Automatic trace generation from events:

```a22
# Tracing automatically enabled
# correlation_id tracks related events

# View trace
trace
	trace_id: correlation_id
	visualization: waterfall

	spans
		for_each event in context
			where event.correlation_id == trace_id

			span
				name: event.type
				start_time: event.time
				duration: next_event.time - event.time
				attributes: event.data
				tags: event.metadata.tags

# Manual trace points
agent "processor"
	do
		trace_span "data_processing"
			tags: [critical, user_facing]
			attributes
				user_id: input.user_id
				data_size: len(input.data)

		result = complex_operation input.data

		trace_span "processing_complete"
			tags: [success]
			attributes
				result_size: len(result)
				processing_time_ms: elapsed_time

		return result
```

### 17.2 Metrics

Auto-generated from events:

```a22
metrics
	# Counter metrics
	agent_invocations
		type: counter
		from_events: "agent.invoked"
		labels: [agent_id, model]

	# Histogram metrics
	workflow_duration
		type: histogram
		from_events: ["workflow.started", "workflow.completed"]
		labels: [workflow_id]
		buckets: [0.1, 0.5, 1, 5, 10, 30, 60]
		unit: seconds

	# Gauge metrics
	active_workflows
		type: gauge
		from_events: ["workflow.started", "workflow.completed"]
		compute: count(started) - count(completed)

	# Custom business metrics
	business_metric "revenue_per_hour"
		from_events: "transaction.completed"
		compute: sum(data.amount)
		window: 1h
		labels: [currency, product_type]
```

### 17.3 Time-Travel Debugging

```a22
debug_session
	# Load production context snapshot
	context: production_snapshot_abc123

	# Replay from specific point
	replay_from: event_id_xyz789

	# Set breakpoints
	breakpoints
		on_event: "agent.invoked"
			where agent_id == "problematic_agent"

		on_condition: error_count > 5

		on_data_value: input.user_id == "test_user_123"

	# Enable detailed logging
	trace_level: :verbose

	# Step through execution
	step_mode: true

	# Inspect variables at each step
	variable_inspection: true

	# Modify context for testing
	context_modifications
		add_event
			type: "test.override"
			data: {test_mode: true}
```

### 17.4 Logging

```a22
logging
	level: :info  # :debug, :info, :warn, :error, :critical

	structured: true
	format: :json

	outputs
		console
			enabled: true
			level: :info

		file
			enabled: true
			path: "/var/log/a22/app.log"
			rotation
				max_size: 100mb
				max_files: 10

		remote
			enabled: true
			endpoint: "https://logs.company.com/ingest"
			batch_size: 100
			flush_interval: 10s

	# Automatic logging from events
	log_events
		include: [error.*, warning.*, audit.*]
		exclude: [debug.*, trace.*]

		enrichment
			add_context_snapshot: for_errors
			add_stack_trace: for_errors
			add_user_context: true
```

---

## 18. Versioning & Migration

### 18.1 Specification Version

```a22
spec_version "1.0"

# Version constraints for dependencies
requires
	spec: ">=1.0, <2.0"
	stdlib: "^1.0.0"
	providers
		openai: "^1.2.0"
		anthropic: "^0.5.0"
```

### 18.2 Context Migration

Migrate contexts across versions:

```a22
migration "0.1_to_1.0"
	from_version: "0.1"
	to_version: "1.0"

	# Transform events
	transform_events
		when event.type == "message.user"
			-> new_event
				type: "message.incoming"
				time: event.time
				data: event.data
				metadata
					migrated: true
					original_type: event.type

		when event.type == "agent.response"
			-> new_event
				type: "agent.output"
				time: event.time
				data: event.data

	# Add required metadata
	add_metadata
		events: all
		metadata
			migrated_at: now()
			migration_version: "0.1_to_1.0"
			schema_version: "1.0"

	# Validate migration
	validate
		all_events_have: metadata.schema_version
		no_unknown_event_types: true
		context_size_within: 10% of original
```

### 18.3 Backward Compatibility

```a22
# Support old event types
compatibility
	legacy_event_types
		"message.user" -> "message.incoming"
		"agent.response" -> "agent.output"
		"tool.call" -> "tool.invoked"

	# Graceful degradation
	when_unknown_event_type
		action: :log_warning
		fallback: :treat_as_generic_event

	# Feature flags
	features
		new_context_querying
			enabled: true
			fallback: :legacy_context_access

		enhanced_error_handling
			enabled: true
			fallback: :basic_error_handling
```

---

## 19. Deployment

### 19.1 Deployment Package

```a22
deployment "my_agent_system"
	version: "1.0.0"
	spec_version: "1.0"

	includes
		agents: [research_assistant, analyst, editor]
		workflows: [research_workflow, review_workflow]
		tools: [web_search, custom_analyzer]
		policies: [safe_mode, production_policy]

	dependencies
		stdlib: "^1.0"
		providers
			openai: "^1.2"
			anthropic: "^0.5"

	environment
		requires
			redis: ">=7.0"
			postgres: ">=14.0"

		optional
			elasticsearch: ">=8.0"

		secrets
			openai_key: env.OPENAI_API_KEY
			db_password: secrets.DB_PASSWORD
			redis_url: secrets.REDIS_URL

	resources
		memory: 4gb
		cpu: 2 cores
		storage: 20gb
		gpu: optional

	health_check
		endpoint: "/health"
		interval: 30s
		timeout: 5s
		retries: 3
```

### 19.2 Configuration Management

```a22
config :development
	providers
		openai
			model: :gpt-3.5-turbo
			max_tokens: 1000

	state
		backend: :memory

	policies: :permissive
	debug: true
	trace_level: :verbose

config :staging
	providers
		openai
			model: :gpt-4-turbo
			max_tokens: 2000
		anthropic
			model: :claude-3-sonnet

	state
		backend: :redis
		url: env.STAGING_REDIS_URL

	policies: :moderate
	debug: true

config :production
	providers
		openai
			model: :gpt-4-turbo
			max_tokens: 4000
		anthropic
			model: :claude-3-opus

	state
		backend: :postgres
		url: env.PROD_DB_URL
		replicas: 3

	policies: :strict
	debug: false
	audit: true

	rate_limiting
		global: 1000/min
		per_user: 100/min

	monitoring
		enabled: true
		export_to: ["prometheus", "datadog"]
```

### 19.3 Scaling Configuration

```a22
deployment "my_agent_system"
	scaling
		auto_scaling
			enabled: true
			min_instances: 2
			max_instances: 10

			metrics
				cpu_threshold: 70%
				memory_threshold: 80%
				request_queue_depth: 100

			scale_up
				instances: +2
				cooldown: 5m

			scale_down
				instances: -1
				cooldown: 10m

		load_balancing
			strategy: :least_connections
			health_check_interval: 30s
			session_affinity: optional

	high_availability
		replicas: 3
		distribution: multi_az
		failover_time: <30s
```

---

## 20. Testing

### 20.1 Agent Tests

```a22
test "agent responds to greeting"
	given
		agent :assistant
		context []
		input "Hello"

	expect
		produces event
			type: "agent.output"
			data.content contains "hello" or "hi"

		completes within 5s
		calls model once

		no errors

test "agent uses tools correctly"
	given
		agent :researcher
		context []
		input "Search for quantum computing"

	expect
		calls tool :web_search
			with query contains "quantum computing"

		produces event
			type: "agent.output"
			data.sources is_not empty

		completes within 30s
```

### 20.2 Workflow Tests

```a22
test "research workflow succeeds"
	given
		workflow :research
		context []
		input
			topic: "artificial intelligence"

	when
		execute workflow

	expect
		workflow.completed within 60s

		calls tool :web_search once
		calls agent :analyst once

		produces event
			type: "workflow.completed"
			data.result is_not empty
			data.result.summary length >100

		no errors

test "workflow handles errors gracefully"
	given
		workflow :resilient_process
		context []
		input
			data: invalid_data

	when
		execute workflow

	expect
		produces error event
			type: "error.validation"

		retry attempted 3 times

		fallback executed

		workflow.completed with fallback_result
```

### 20.3 Integration Tests

```a22
test "multi-agent collaboration"
	given
		agents: [supervisor, researcher, analyst]
		workflow :collaborative_research
		input
			topic: "climate change"

	expect
		supervisor delegates to [researcher, analyst]

		parallel execution of
			- researcher.search
			- analyst.analyze

		supervisor.synthesize called with both results

		workflow.completed within 2m

		produces comprehensive_report
			sections: ["summary", "findings", "analysis"]

test "end-to-end user interaction"
	given
		agent :assistant
		context []

	scenario
		# User asks question
		user_input "What is quantum computing?"

		expect
			agent.invoked
			tool.web_search called
			agent.output produced within 10s

		# Follow-up question
		user_input "How does it differ from classical computing?"

		expect
			agent uses conversation_context
			agent.output references previous_response
			completes within 10s
```

### 20.4 Property-Based Tests

```a22
test "projection consistency"
	given
		projection :conversation_history

	property
		for_all context in generated_contexts
			messages = projection(context).messages

			assert count(messages) == count(context, type: "message.*")
			assert messages is_ordered_by time
			assert all(messages, has_required_fields)

test "determinism guarantee"
	given
		agent :chatbot
		context sample_context
		input "Hello"

	property
		result1 = execute agent
		result2 = execute agent

		assert result1 == result2
		assert result1.event.type == result2.event.type
		assert result1.event.data == result2.event.data
```

---

## 21. Runtime Model

### 21.1 Event Loop

```
loop:
  1. Receive input or trigger
  2. Component(context, input) → new_event
  3. Validate via policies
  4. Append event to context
  5. Update projections
  6. Trigger next steps / workflows
  7. Emit metrics/traces
```

### 21.2 Pure Function Semantics

```
# Agents
agent(context, input) -> event
  where ∀context, input: agent(context, input) = agent(context, input)
  (referential transparency)

# Workflows
workflow(context, input) -> [event]
  where events are produced in causal order
  and each step is pure

# Tools (from A22 perspective)
tool(input) -> event
  where side effects are external
  and event is deterministic for given input
```

### 21.3 Context Evolution

```
context_0 = []
context_1 = context_0 + [user.message]
context_2 = context_1 + [agent.thinking]
context_3 = context_2 + [agent.output]

# State views computed on demand
state_t = projection(context_t)
```

### 21.4 Execution Guarantees

1. **Determinism**: Same context + same input → same event
2. **Immutability**: Events never change after creation
3. **Causality**: Events strictly ordered by time
4. **Atomicity**: Event appends are atomic operations
5. **Durability**: Events persisted before acknowledgment
6. **Observability**: All actions produce traceable events

---

## 22. Complete Examples

### 22.1 Simple Chatbot

```a22
spec_version "1.0"

provider :openai
	type :llm
	auth env.OPENAI_API_KEY

agent "chatbot"
	can chat
	use model: :gpt4 from :openai

	prompt :system
		"You are a helpful AI assistant."

	state :memory
		conversation = projection("conversation_history")
		recent = take(conversation.messages, 50)

	when user.message
		-> respond

test "chatbot responds"
	given
		agent :chatbot
		input "Hello"

	expect
		produces agent.output within 5s
		output contains greeting
```

### 22.2 Research Assistant

```a22
spec_version "1.0"

# Providers
provider :openai
	type :llm
	auth env.OPENAI_API_KEY

# Tools
tool "web_search"
	endpoint "https://api.search.com/v1"
	runtime :http
	auth env.SEARCH_KEY

	input
		query: text
		max_results: number
	output
		results: list<SearchResult>

# Agents
agent "researcher"
	can search, analyze
	use model: :gpt4
	use tools: [web_search]

	when user.query
		-> .research_workflow

agent "analyst"
	can analyze, summarize
	use model: :gpt4

	when analysis.request
		-> analyze_and_summarize

# Workflows
workflow "research_workflow"
	steps
		# Search
		search_results = web_search
			query: input.query
			max_results: 10

		# Analyze
		analysis = agent "analyst"
			message: "Analyze these search results"
			context: search_results

		# Quality check
		quality = evaluate_quality analysis

		when quality.score <7
			# Improve with loop
			loop max: 2
				feedback = generate_feedback quality
				analysis = agent "analyst"
					message: "Improve based on feedback"
					context: [search_results, feedback]
				quality = evaluate_quality analysis
				when quality.score >=7
					-> break

		return analysis
```

### 22.3 Multi-Agent System

```a22
spec_version "1.0"

# Agents with roles
agent "supervisor"
	role :coordinator
	can delegate, review, approve
	use model: :gpt4

	delegates_to
		agents: [researcher, writer, editor]
		max_concurrent: 3

	workflow "content_creation"
		steps
			# Plan
			outline = create_outline topic: input.topic

			# Delegate research
			research = agent "researcher"
				task: "Research " + input.topic
				outline: outline

			# Delegate writing
			draft = agent "writer"
				task: "Write article"
				research: research
				outline: outline

			# Delegate editing
			edited = agent "editor"
				task: "Edit and polish"
				draft: draft

			# Review
			review = evaluate_content edited

			branch review.quality
				when >8 -> approve_and_publish edited
				when 5..8 -> request_revisions review.feedback
				when <5 -> restart_workflow

agent "researcher"
	role :worker
	reports_to: supervisor
	can search, analyze, fact_check
	use model: :gpt4
	use tools: [web_search, arxiv_search]

agent "writer"
	role :worker
	reports_to: supervisor
	can write, structure, cite
	use model: :gpt4

agent "editor"
	role :worker
	reports_to: supervisor
	can edit, proofread, optimize
	use model: :gpt4
```

### 22.4 Production System with Full Features

```a22
spec_version "1.0"

# Providers
provider :openai
	type :llm
	auth env.OPENAI_API_KEY
	limits
		requests_per_minute: 3500
		tokens_per_minute: 90000

provider :anthropic
	type :llm
	auth env.ANTHROPIC_API_KEY
	limits
		requests_per_minute: 1000

# Types
type UserQuery
	fields
		text: text
		user_id: uuid
		priority: text
		context: optional<map<text, any>>

type AnalysisResult
	fields
		summary: text
		confidence: number
		sources: list<text>
		timestamp: timestamp

# Policies
policy :production_safe
	allow
		tools: [web_search, database_query]
		capabilities: [search, analyze]

	deny
		tools: [system_commands]
		operations
			write_to: [production_db]

	limits
		requests_per_second: 10
		tokens_per_minute: 50000
		max_execution_time: 300s

	audit
		enabled: true
		log_all_events: true

privacy
	pii_fields
		mark
			user_id: :user_identifier
			user.email: :email

	retention
		events_with_pii
			ttl: 90d
			after_deletion: anonymize

# Tools
tool "web_search"
	endpoint "https://api.search.com/v1"
	runtime :http
	auth env.SEARCH_KEY

	input
		query: text
		max_results: number
	output
		results: list<SearchResult>

	sandbox
		timeout: 10s
		network: limited
		rate_limit: 100/min

# Agents
agent "production_assistant"
	can search, analyze, summarize

	use model
		primary :gpt4 from :openai
		fallback [:claude from :anthropic]
		strategy :cost_optimized

	use tools: [web_search]
	has policy: :production_safe

	state :persistent
		backend :postgres
		ttl 24h

	remembers
		queries: last 100
		results: last 50

	when user.query
		-> .production_workflow

# Workflows
workflow "production_workflow"
	deadline: 5m

	steps
		# Validate input
		validated = validate_input input

		when validated.invalid
			-> return_error validated.errors

		# Search with retry
		search_results = web_search
			query: validated.query
			max_results: 10

	on_error
		retry
			max_attempts: 3
			backoff: exponential
		fallback
			-> cached_results validated.query

	steps
		# Analyze
		analysis = agent "production_assistant"
			message: "Analyze results"
			context: search_results

		# Quality gate
		quality = evaluate_quality analysis

		when quality.score <7
			loop max: 2
				analysis = improve_analysis
					original: analysis
					feedback: quality.feedback
				quality = evaluate_quality analysis
				when quality.score >=7
					-> break

		# Audit log
		audit_log
			event_type: "workflow.completed"
			user_id: input.user_id
			result: analysis

		return analysis

# Metrics
metrics
	workflow_duration
		type: histogram
		from_events: ["workflow.started", "workflow.completed"]
		labels: [workflow_id]
		buckets: [1, 5, 10, 30, 60]

	error_rate
		type: counter
		from_events: "error.*"
		labels: [error_type, component]

# Observability
logging
	level: :info
	structured: true
	outputs
		console: true
		file: "/var/log/a22/app.log"
		remote: "https://logs.company.com"

# Deployment
deployment "production"
	version: "1.0.0"

	resources
		memory: 4gb
		cpu: 2cores
		storage: 20gb

	scaling
		auto_scaling
			min_instances: 2
			max_instances: 10
			cpu_threshold: 70%

	high_availability
		replicas: 3
		distribution: multi_az

# Tests
test "production workflow succeeds"
	given
		workflow :production_workflow
		input
			query: "test query"
			user_id: "test_user"

	expect
		completes within 60s
		produces analysis_result
		audit_logged
		no_pii_leaked
```

---

## 23. Syntax Reference

### 23.1 Keywords

```
# Top-level
agent, tool, workflow, policy, provider, capability
schedule, test, import, type, projection
event_type, migration, deployment, config
metrics, logging, privacy, audit

# Statements
can, use, do, has, is, when, needs
steps, parallel, branch, loop, return
break, continue
from, as
given, expect
```

### 23.2 Symbols and References

```
:name          # Symbol (provider, model, capability)
.name          # Reference (workflow, agent, tool)
->             # Arrow (action binding)
..             # Range operator (5..10)
```

### 23.3 Operators

```
# Comparison
==, !=, <, >, <=, >=

# Logical
and, or, not

# Arithmetic
+, -, *, /, %

# Membership
in, not in, contains
```

### 23.4 Data Types

```
# Primitives
text, number, boolean, timestamp, duration, uuid

# Collections
list<T>, map<K,V>, set<T>, optional<T>

# Special
any, never
```

### 23.5 Duration Format

```
5s      # 5 seconds
10m     # 10 minutes
2h      # 2 hours
1d      # 1 day
1w      # 1 week
```

### 23.6 Size Format

```
256kb   # Kilobytes
128mb   # Megabytes
2gb     # Gigabytes
1tb     # Terabytes
```

---

## Appendices

### A. Event Type Reference

See Section 3.1 for complete event type catalog.

### B. Standard Library Reference

See Section 16 for complete stdlib documentation.

### C. Migration Guide from v0.1

See Section 18 for migration procedures.

### D. Implementation Conformance Tests

Implementations must pass conformance test suite covering:
- All five primitives
- Event system
- Context querying
- Error handling
- Determinism guarantees
- Security policies

### E. Formal Semantics

See whitepaper for complete formal foundations including:
- Event algebra
- Pure function semantics
- Temporal logic
- Determinism proofs

---

**End of Specification**

**A22 v1.0 — Stable**
**© 2024 A22 Foundation**
**License: Apache 2.0**
