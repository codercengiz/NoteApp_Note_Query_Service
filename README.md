# NoteApp_Note_Query_Service
A query microservice for note domain of NoteApp. Rust, Mongodb, Kafka  

## Install docker of eventstore and kafka
```console
make generate-all
```

## Run microservice
```console
make run
```

## TODO
- [x] Settings 
- [x] Web Server with warp
- [x] Mongodb Service 
- [x] Kafka Consumer
- [x] Note Create event catch and apply to mongodb
- [x] Note Parent Change event catch and apply to mongodb
- [x] Note Basic Info Change event catch and apply to mongodb
- [ ] File upload event catch and apply to mongodb
- [ ] Image upload event catch and apply to mongodb
- [ ] File Delete event catch and apply to mongodb
- [ ] Image delete event catch and apply to mongodb
- [ ] Note Delete event catch and apply to mongodb
- [ ] Healthcheck
- [ ] Api version mechanism
- [ ] Docker file

## HTTP-API
### Get Notes for a User
