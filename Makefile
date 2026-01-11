.PHONY: demo stop
demo:
	docker compose up --build -d
	@echo "=========================================="
	@echo "  API docs : http://localhost:5000/docs"
	@echo "  Front    : http://localhost:5173"
	@echo "  MQTT     : localhost:1883  (no auth)"
	@echo "=========================================="

stop:
	docker compose down --volumes --remove-orphans
	@echo "All containers removed."
