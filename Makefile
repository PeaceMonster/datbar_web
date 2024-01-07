default: build

build:
	cd frontend; npm run build

dev: 
	cd frontend; npm run dev

server:
	flask --app app.py run