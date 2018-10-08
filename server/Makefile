.PHONY: db

DB_DIR=${PWD}/run-data
DB_USER="dev-control"

# db starts a local postgres database
db:
	docker run \
		-it \
		--rm \
		--net host \
		-v "${DB_DIR}:/var/lib/postgresql/data" \
		-e POSTGRES_USER="${DB_USER}" \
		postgres
