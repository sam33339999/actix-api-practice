.PHONY: migrate-up migrate-down gen-schema

migrate-up:
	@echo "Migrating up..."
	@diesel migration run

migrate-down:
	@echo "Migrating down..."
	@diesel migration redo

gen-schema:
	@source ~/.zshrc
	@echo "Generating schema.rs..."
	@diesel print-schema > src/schema.rs
	@echo "Done"
