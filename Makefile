docker:
	sudo docker run --rm --name sqlx-pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

dev:
	cargo watch -q -c -w ./src/ -w ./.cargo/ -x "run"

test:
	cargo test model::task::tests -- --nocapture

quick_dev:
	cargo watch -q -c -w examples/ -x "run --example quick_dev"
