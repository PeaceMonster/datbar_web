# Datbar Web 

## Setup

### Backend

Sørg for at du har den nyeste version af cargo.

Installer dependencies med

```
cargo install
```

### Frontend
Cd ind i src/frontend og installer packages:

```
npm install
```


## Building

Brug makefilen, ellers for manuel

### Byg frontend

```
cd frontend && npm run build
```

### Køre backend

```
cargo run
```

### Køre frontend i dev mode (Hot reload)

Kan være at server skal køre i baggrunden baseret på feature der skal testes

```
cd frontend && npm run dev
```

## Guideline for contributions


### Frontend

For frontend skal hver ny side have en mappe med snake_case navn for side id (Giv et ordenligt og klart navn) under `frontend/src/(navn_her)/`
I den mappe skal alt logik og svelte komponenter ligge for den side og kun den side.
Svelte komponenter skrives med PascalCase, og scripts skrives med camelCase.

Hvis noget logik kan genbruges mellem sider, læg det i `frontend/src/lib/`

### Backend

WIP