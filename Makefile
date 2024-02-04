.PHONY: setup run-vercel run-localtunnel

# Default port used by Vercel Dev
PORT = 3000

# Check if localtunnel is installed and install it if not
setup-localtunnel:
	@echo "Checking localtunnel..."
	@if ! command -v lt > /dev/null; then \
	echo "localtunnel not found, installing..."; \
		npm install -g localtunnel; \
	else \
		echo "localtunnel is already installed."; \
	fi

# Command to run setup and then start vercel dev
run-vercel: setup-localtunnel
	@echo "Starting Vercel Dev..."
	@vercel dev &

# Command to start the tunnel
run-localtunnel:
	@echo "Starting localtunnel..."
	@lt --port $(PORT)

# Command to start everything
dev: run-vercel run-localtunnel