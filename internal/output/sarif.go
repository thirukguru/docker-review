package output

import (
	"encoding/json"
	"path/filepath"

	"github.com/thirukguru/docker-review/internal/analyzer"
	"github.com/thirukguru/docker-review/internal/types"
)

// SARIF format structs
type SARIFReport struct {
	Schema  string     `json:"$schema"`
	Version string     `json:"version"`
	Runs    []SARIFRun `json:"runs"`
}

type SARIFRun struct {
	Tool    SARIFTool     `json:"tool"`
	Results []SARIFResult `json:"results"`
}

type SARIFTool struct {
	Driver SARIFDriver `json:"driver"`
}

type SARIFDriver struct {
	Name           string      `json:"name"`
	Version        string      `json:"version"`
	InformationURI string      `json:"informationUri"`
	Rules          []SARIFRule `json:"rules"`
}

type SARIFRule struct {
	ID               string             `json:"id"`
	Name             string             `json:"name"`
	ShortDescription SARIFDescription   `json:"shortDescription"`
	FullDescription  SARIFDescription   `json:"fullDescription"`
	DefaultConfig    SARIFDefaultConfig `json:"defaultConfiguration"`
}

type SARIFDescription struct {
	Text string `json:"text"`
}

type SARIFDefaultConfig struct {
	Level string `json:"level"`
}

type SARIFResult struct {
	RuleID    string           `json:"ruleId"`
	Level     string           `json:"level"`
	Message   SARIFDescription `json:"message"`
	Locations []SARIFLocation  `json:"locations"`
}

type SARIFLocation struct {
	PhysicalLocation SARIFPhysicalLocation `json:"physicalLocation"`
}

type SARIFPhysicalLocation struct {
	ArtifactLocation SARIFArtifactLocation `json:"artifactLocation"`
	Region           SARIFRegion           `json:"region"`
}

type SARIFArtifactLocation struct {
	URI string `json:"uri"`
}

type SARIFRegion struct {
	StartLine int `json:"startLine"`
}

// FormatSARIF converts a report to SARIF format
func FormatSARIF(report *analyzer.Report, version string) string {
	// Build unique rules
	ruleMap := make(map[string]SARIFRule)
	for _, issue := range report.Issues {
		if _, exists := ruleMap[issue.RuleID]; !exists {
			ruleMap[issue.RuleID] = SARIFRule{
				ID:               issue.RuleID,
				Name:             issue.Name,
				ShortDescription: SARIFDescription{Text: issue.Name},
				FullDescription:  SARIFDescription{Text: issue.Description},
				DefaultConfig: SARIFDefaultConfig{
					Level: severityToSARIF(issue.Severity),
				},
			}
		}
	}

	var rules []SARIFRule
	for _, rule := range ruleMap {
		rules = append(rules, rule)
	}

	// Build results
	var results []SARIFResult
	for _, issue := range report.Issues {
		results = append(results, SARIFResult{
			RuleID:  issue.RuleID,
			Level:   severityToSARIF(issue.Severity),
			Message: SARIFDescription{Text: issue.Description},
			Locations: []SARIFLocation{{
				PhysicalLocation: SARIFPhysicalLocation{
					ArtifactLocation: SARIFArtifactLocation{
						URI: filepath.Base(report.FilePath),
					},
					Region: SARIFRegion{StartLine: issue.Line},
				},
			}},
		})
	}

	sarif := SARIFReport{
		Schema:  "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
		Version: "2.1.0",
		Runs: []SARIFRun{{
			Tool: SARIFTool{
				Driver: SARIFDriver{
					Name:           "docker-review",
					Version:        version,
					InformationURI: "https://github.com/thirukguru/docker-review",
					Rules:          rules,
				},
			},
			Results: results,
		}},
	}

	data, _ := json.MarshalIndent(sarif, "", "  ")
	return string(data)
}

func severityToSARIF(sev types.Severity) string {
	switch sev {
	case types.Critical:
		return "error"
	case types.Warning:
		return "warning"
	default:
		return "note"
	}
}
