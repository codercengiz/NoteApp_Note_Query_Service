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
- [ ] Settings 
- [ ] Web Server with warp
- [ ] Eventstoredb 
- [ ] Kafka
- [ ] Note Create event
- [ ] Note Parent Change event
- [ ] Note Basic Info Change event
- [ ] File upload event
- [ ] Image upload event
- [ ] File Delete event
- [ ] Image delete event
- [ ] Note Delete event
- [ ] Healthcheck
- [ ] Api version mechanism
- [ ] Docker file

## HTTP-API
### Create Note
