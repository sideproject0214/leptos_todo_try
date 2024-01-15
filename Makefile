.PHONY: init-db check-db init quit-docker quit-check-db q 

init-db:
	@container_id=$$(docker ps -aqf "name=rust_todo_db" --filter "status=running"); \
	if [ -n "$$container_id" ]; then \
		echo "❌ DB is running. docker run pass!"; \
	else \
		container_exist=$$(docker ps -aqf "name=rust_todo_db"); \
		if [ -n "$$container_exist" ]; then \
			echo "✅ DB is not running, Docker Run starting..."; \
			docker start $$(docker ps -aqf "name=rust_todo_db"); \
		else \
			echo "✅ DB is not exist, building and starting..."; \
			cd ./db && docker build -t db:v1 -f Dockerfile.dev . && docker run -d -p 5432:5432 --name rust_todo_db db:v1; \
		fi; \
	fi;

check-db:
	@i=0;\
	while [ $$i -lt 30 ]; do \
		if [ $$(docker ps -qf "name=rust_todo_db" --filter "status=running") ]; then \
			echo "✅ Try $$(($$i+1)) : Database is ready!"; \
			exit 0; \
		else \
			echo "❌ Try $$i : Database is not ready yet. Retrying in 3 seconds..."; \
			sleep 3; \
			i=$$(($$i+1)); \
		fi; \
	done;


init: init-db check-db
	@cargo leptos watch &


quit-docker:
	@container_exist=$$(docker ps -aqf "name=rust_todo_db" --filter "status=running"); \
	if [ -z "$$container_exist" ]; then \
		echo "✅ DB already stop..."; \
	else \
		echo "✅ DB stop success"; \
		docker stop rust_todo_db; \
	fi;

quit-check-db:
	@i=0; \
	while [ $$i -lt 30 ]; do \
		if [ $$(docker ps -qf "name=rust_todo_db" --filter "status=running") ]; then \
			echo "❌ Try $$i : DB is running... Retrying in 1 seconds..."; \
			sleep 1; \
			i=$$(($$i+1)); \
		else \
			echo "✅ Try $$(($$i+1)) : DB stop"; \
			exit 0; \
		fi; \
	done;

q: quit-docker quit-check-db
	@pkill -f "leptos" & 
	@pkill -f "cargo-watch" & 

