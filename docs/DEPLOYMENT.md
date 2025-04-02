# Deplying backend and frontend (docker)

## Create docker network for backend <-> frontend communication

```sh
docker network create liberland-vote-scope
```

## Build and deploy backend

1. Clone repository:

```sh
git clone https://github.com/rbochenek/liberland-vote-scope
```

2. Build docker image

```sh
docker build -t liberland-vote-scope .
```

3. Run

```sh
docker run -d --network liberland-vote-scope --name liberland-vote-scope --restart unless-stopped liberland-vote-scope
```

## Build and deploy frontend

1. Clone repository:

```sh
git clone https://github.com/rbochenek/liberland-vote-scope-frontend.git
```

2. Build docker image

```sh
docker build -t liberland-vote-scope-frontend .
```

3. Run

```sh
docker run -d --network liberland-vote-scope --name liberland-vote-scope-frontend --restart unless-stopped -p 3000:3000 liberland-vote-scope-frontend
```
