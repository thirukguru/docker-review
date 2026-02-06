package types

import (
	"fmt"
)

// Severity levels for issues
type Severity int

const (
	Suggestion Severity = iota
	Warning
	Critical
)

func (s Severity) String() string {
	switch s {
	case Critical:
		return "Critical"
	case Warning:
		return "Warning"
	case Suggestion:
		return "Suggestion"
	default:
		return "Unknown"
	}
}

// ParseSeverity converts a string to Severity
func ParseSeverity(s string) Severity {
	switch s {
	case "critical", "Critical":
		return Critical
	case "warning", "Warning":
		return Warning
	case "suggestion", "Suggestion":
		return Suggestion
	default:
		return Suggestion
	}
}

// Issue represents a detected problem
type Issue struct {
	RuleID      string   `json:"rule_id"`
	Name        string   `json:"name"`
	Severity    Severity `json:"severity"`
	Line        int      `json:"line"`
	Description string   `json:"description"`
	Fix         string   `json:"fix"`
	Category    string   `json:"category"` // security, performance, maintainability
}

// Rule interface for all rules
type Rule interface {
	ID() string
	Name() string
	Severity() Severity
	Category() string
	Description() string
	Fix() string
	Explain() string
}

// DockerfileContext provides parsed Dockerfile data for rules
type DockerfileContext struct {
	Path            string
	Lines           []string
	Instructions    []Instruction
	HasDockerignore bool
}

// Instruction represents a Dockerfile instruction
type Instruction struct {
	Name      string
	Arguments string
	Line      int
	Raw       string
}

// ComposeContext provides parsed compose data for rules
type ComposeContext struct {
	Path     string
	Services map[string]Service
	Raw      map[string]interface{}
}

// Service represents a docker-compose service
type Service struct {
	Name        string
	Image       string
	Build       interface{}
	Privileged  bool
	Restart     string
	Environment map[string]string
	Deploy      *Deploy
	Raw         map[string]interface{}
}

// Deploy represents deploy configuration
type Deploy struct {
	Resources *Resources
}

// Resources represents resource limits
type Resources struct {
	Limits *ResourceSpec
}

// ResourceSpec represents CPU/memory limits
type ResourceSpec struct {
	CPUs   string
	Memory string
}

// DockerfileRule checks Dockerfile content
type DockerfileRule interface {
	Rule
	Check(dockerfile *DockerfileContext) []Issue
}

// ComposeRule checks docker-compose.yml content
type ComposeRule interface {
	Rule
	Check(compose *ComposeContext) []Issue
}

// BaseRule provides common implementation
type BaseRule struct {
	RuleID          string
	RuleName        string
	RuleSeverity    Severity
	RuleCategory    string
	RuleDescription string
	RuleFix         string
}

func (r BaseRule) ID() string          { return r.RuleID }
func (r BaseRule) Name() string        { return r.RuleName }
func (r BaseRule) Severity() Severity  { return r.RuleSeverity }
func (r BaseRule) Category() string    { return r.RuleCategory }
func (r BaseRule) Description() string { return r.RuleDescription }
func (r BaseRule) Fix() string         { return r.RuleFix }

func (r BaseRule) Explain() string {
	return fmt.Sprintf(`Rule: %s (%s)
Severity: %s
Category: %s

Description:
  %s

Recommended Fix:
  %s
`, r.RuleName, r.RuleID, r.RuleSeverity, r.RuleCategory, r.RuleDescription, r.RuleFix)
}
