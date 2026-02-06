package parser

import (
	"os"
	"path/filepath"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
	"gopkg.in/yaml.v3"
)

// ParseCompose parses a docker-compose.yml file
func ParseCompose(path string) (*types.ComposeContext, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	var raw map[string]interface{}
	if err := yaml.Unmarshal(data, &raw); err != nil {
		return nil, err
	}

	ctx := &types.ComposeContext{
		Path:     path,
		Services: make(map[string]types.Service),
		Raw:      raw,
	}

	if services, ok := raw["services"].(map[string]interface{}); ok {
		for name, svc := range services {
			if svcMap, ok := svc.(map[string]interface{}); ok {
				service := parseService(name, svcMap)
				ctx.Services[name] = service
			}
		}
	}

	return ctx, nil
}

func parseService(name string, svcMap map[string]interface{}) types.Service {
	service := types.Service{
		Name:        name,
		Environment: make(map[string]string),
		Raw:         svcMap,
	}

	if image, ok := svcMap["image"].(string); ok {
		service.Image = image
	}

	if build, ok := svcMap["build"]; ok {
		service.Build = build
	}

	if privileged, ok := svcMap["privileged"].(bool); ok {
		service.Privileged = privileged
	}

	if restart, ok := svcMap["restart"].(string); ok {
		service.Restart = restart
	}

	if env, ok := svcMap["environment"].(map[string]interface{}); ok {
		for k, v := range env {
			if vs, ok := v.(string); ok {
				service.Environment[k] = vs
			}
		}
	} else if envList, ok := svcMap["environment"].([]interface{}); ok {
		for _, e := range envList {
			if es, ok := e.(string); ok {
				parts := strings.SplitN(es, "=", 2)
				if len(parts) == 2 {
					service.Environment[parts[0]] = parts[1]
				}
			}
		}
	}

	if deploy, ok := svcMap["deploy"].(map[string]interface{}); ok {
		service.Deploy = &types.Deploy{}
		if resources, ok := deploy["resources"].(map[string]interface{}); ok {
			service.Deploy.Resources = &types.Resources{}
			if limits, ok := resources["limits"].(map[string]interface{}); ok {
				service.Deploy.Resources.Limits = &types.ResourceSpec{}
				if cpus, ok := limits["cpus"].(string); ok {
					service.Deploy.Resources.Limits.CPUs = cpus
				}
				if memory, ok := limits["memory"].(string); ok {
					service.Deploy.Resources.Limits.Memory = memory
				}
			}
		}
	}

	return service
}

// IsComposeFile checks if a file is a docker-compose file
func IsComposeFile(path string) bool {
	base := strings.ToLower(filepath.Base(path))
	return strings.Contains(base, "compose") && (strings.HasSuffix(base, ".yml") || strings.HasSuffix(base, ".yaml"))
}
