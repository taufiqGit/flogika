FROM node:20-alpine AS web_builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM golang:1.22-alpine AS go_builder
WORKDIR /app
COPY go-server ./go-server
RUN cd go-server && go mod download
RUN cd go-server && CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o /app/server ./main.go

FROM alpine:3.20
WORKDIR /app
RUN adduser -D appuser
COPY --from=go_builder /app/server /usr/local/bin/server
COPY --from=web_builder /app/dist ./dist
USER appuser
EXPOSE 8081
CMD ["server"]
