.DEFAULT_GOAL := run

migration-1:
	sqlx database create

migration-2: migration-1
	sqlx migrate run

run: migration-2
	./target/release/app_server
