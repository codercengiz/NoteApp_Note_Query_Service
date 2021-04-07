SHELL := /bin/bash

.PHONY: generate-all generate-mongodb generate-kafka delete-all

generate-all:generate-network generate-mongodb generate-kafka

delete-all: delete-mongodb delete-kafka delete-networks

generate-mongodb:
	@docker-compose -f docker-compose-for-mongo.yaml up -d

generate-kafka:
	@docker-compose -f docker-compose-for-kafka.yaml up -d

generate-network:
	@docker network create note-app || true

delete-mongodb:
	@docker-compose -f docker-compose-for-mongo.yaml down
	@docker volume rm $(docker volume ls --filter name=noteApp_note_query_service -q) || true
	
delete-kafka:
	@docker-compose -f docker-compose-for-kafka.yaml down
	@docker volume ls --filter name=noteApp_note_query_service_zookeeper -q | xargs docker volume rm || true
	@docker volume ls --filter name=noteApp_note_query_service_kafka -q | xargs docker volume rm ||true

delete-networks:
	@docker network ls --filter name=note-app -q | xargs docker network rm