.PHONY: db db-cli migrate

DB_DIR=${PWD}/run-data
DB_USER=dev-control
DB_NAME=dev-control

# db starts a local postgres database
db:
	docker run \
		-it \
		--rm \
		--net host \
		-v "${DB_DIR}:/var/lib/postgresql/data" \
		-e POSTGRES_USER="${DB_USER}" \
		-e POSTGRES_DB="${DB_NAME}" \
		postgres

# db-cli connects to the local database with the psql command line interface
db-cli:
	psql -h "localhost" -U "${DB_USER}" -d "${DB_NAME}"

# migrate runs database migrations
migrate:
	go run scripts/db-migrate.go
