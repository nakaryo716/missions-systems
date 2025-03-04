.DEFAULT_GOAL := migration-2

migration-1:
	sqlx database create

migration-2: migration-1
	sqlx migrate run
