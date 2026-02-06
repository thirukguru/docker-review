.PHONY: build build-all clean test run

# Version info - auto-detect from git tag, fallback to "dev"
VERSION := $(shell git describe --tags --abbrev=0 2>/dev/null || echo "dev")
BUILD_TIME := $(shell date -u '+%Y-%m-%d_%H:%M:%S')
LDFLAGS := -ldflags="-s -w -X main.Version=$(VERSION) -X main.BuildTime=$(BUILD_TIME)"

# Build for current platform
build:
	go build $(LDFLAGS) -o docker-review ./cmd/docker-review

# Build for all platforms
build-all: clean
	@echo "Building version: $(VERSION)"
	@mkdir -p bin
	GOOS=linux GOARCH=amd64 go build $(LDFLAGS) -o bin/docker-review-$(VERSION)-linux-amd64 ./cmd/docker-review
	GOOS=linux GOARCH=arm64 go build $(LDFLAGS) -o bin/docker-review-$(VERSION)-linux-arm64 ./cmd/docker-review
	GOOS=darwin GOARCH=amd64 go build $(LDFLAGS) -o bin/docker-review-$(VERSION)-darwin-amd64 ./cmd/docker-review
	GOOS=darwin GOARCH=arm64 go build $(LDFLAGS) -o bin/docker-review-$(VERSION)-darwin-arm64 ./cmd/docker-review
	GOOS=windows GOARCH=amd64 go build $(LDFLAGS) -o bin/docker-review-$(VERSION)-windows-amd64.exe ./cmd/docker-review
	@echo "Built binaries for all platforms ($(VERSION)):"
	@ls -lh bin/

# Clean build artifacts
clean:
	rm -rf docker-review bin/

# Run tests
test:
	go test -v ./...

# Run the tool
run:
	go run ./cmd/docker-review $(ARGS)

# Install locally
install: build
	sudo cp docker-review /usr/local/bin/

# Show current version
version:
	@echo $(VERSION)
