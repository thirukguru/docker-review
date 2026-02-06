.PHONY: build build-all clean test run

# Build for current platform
build:
	go build -o docker-review ./cmd/docker-review

# Build for all platforms
build-all: clean
	@mkdir -p bin
	GOOS=linux GOARCH=amd64 go build -ldflags="-s -w" -o bin/docker-review-linux-amd64 ./cmd/docker-review
	GOOS=linux GOARCH=arm64 go build -ldflags="-s -w" -o bin/docker-review-linux-arm64 ./cmd/docker-review
	GOOS=darwin GOARCH=amd64 go build -ldflags="-s -w" -o bin/docker-review-darwin-amd64 ./cmd/docker-review
	GOOS=darwin GOARCH=arm64 go build -ldflags="-s -w" -o bin/docker-review-darwin-arm64 ./cmd/docker-review
	GOOS=windows GOARCH=amd64 go build -ldflags="-s -w" -o bin/docker-review-windows-amd64.exe ./cmd/docker-review
	@echo "Built binaries for all platforms:"
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
