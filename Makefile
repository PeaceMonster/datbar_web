default: build server

build:
	cd frontend; npm run build

dev: 
	cd src/frontend; npm run dev

server:
	flask --app app.py run