format:
	cargo fmt

lint:
	cargo clippy -- -Dclippy::all

fix:
	cargo fix

test:
	cargo test

create-migration:
	diesel migration generate --diff-schema=src/tables.rs $(name)

migrate:
	diesel migration run

migration-rollback:
	diesel migration revert

migration-redo:
	diesel migration redo