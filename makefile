BIN=$(PWD)/.bin
DATABASE_URL=postgres://postgres:postgres@localhost:5432/soves
MIGRATIONS_DIR = ./migrations/

.PHONY: create-db
create-db: ## - Create the database
	@echo "Creating database"
	@docker-compose exec postgres psql -U postgres -c "CREATE DATABASE todo;"
	@echo "Created database"

.PHONY: download-tools
download-tools: ## - Build sqlx and move it to bin directory
	@echo "Downloading tools"
	@mkdir -p $(BIN)
	@curl -L --output $(BIN)/sqlx.tar.gz  "https://github.com/launchbadge/sqlx/archive/refs/tags/v0.8.2.tar.gz"
	@echo "Downloaded sqlx"
	@tar -xzvf $(BIN)/sqlx.tar.gz -C $(BIN)
	@rm -rf $(BIN)/sqlx.tar.gz
	@echo "Extracted sqlx"
	@cargo build --release --manifest-path $(BIN)/sqlx-0.8.2/sqlx-cli/Cargo.toml
	@echo "Built sqlx"
	@mv $(BIN)/sqlx-0.8.2/target/release/sqlx $(BIN)/sqlx
	@echo "Moved sqlx to bin"
	@rm -rf $(BIN)/sqlx-0.8.2
	@echo "Removed sqlx-0.8.2"
	@echo "Downloaded tools"

.PHONY: run-mig
run-mig: ## - Run database migrations, e.g., `make run-mig`
	$(info $(M) running DB migrations...)
	@$(BIN)/sqlx migrate run --source "$(MIGRATIONS_DIR)" --database-url "$(DATABASE_URL)"

.PHONY: add-migration
add-mig: ## - Create a new database migration file, e.g., `make db-create-migration name_of_migration`
	$(info $(M) creating DB migration...)
	@$(BIN)/sqlx migrate add -r $(filter-out $@,$(MAKECMDGOALS))

.PHONY: help
help: ## - Show this help message
	@printf "\033[32mUsage: make [target]\n\n\033[0m"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: help
%: help
	@:
