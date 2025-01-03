# Missions-Systems
[Missions-Systems](#missions-systems)
- [OverView](#overview)
- [Features](#features)
    - [Application server](#application-server)
    - [Resetter](#resetter)
- [Architecture](#architecture)
- [Run](#run)
    - [Set mysql_config](#set-mysql_config)
    - [Config .env file](#config-env-file)
    - [Docker Container run and migration](#docker-container-run-and-migration)
    - [Build application](#build-application)
    - [Start app_server](#start-app_server)
    - [Start mission_resetter](#start-mission_resetter)
- [API](#api)
    - [Create user](#create-user)
    - [Login](#login)
    - [Get user information](#get-user-information)
    - [Update user name](#update-user-name)
    - [Delete user](#delete-user)
    - [Create DailyMission](#create-dailymission)
    - [Get all DailyMission](#get-all-dailymission)
    - [Get specific DailyMission](#get-specific-dailymission)
    - [Update DailyMission](#update-dailymission)
    - [Delete DailyMission](#delete-dailymission)
    - [Set Complete a DailyMission "complete" true](#set-complete-a-dailymission-complete-true)
    - [Get user's experience points and level](#get-users-experience-points-and-level)
- [MissionResetter](#missionresetter)

## OverView
This repository contains the server-side implementation of an application called "Missions."
## Features
### Application server
- User registration
- Authentication
- Registering, reading, updating, and deleting ```DailyMission```
- Retrieving and increasing experience points
### Resetter
- Resetting the completion status of daily missions  
The update query for all users is implemented in ```./mission_resetter```.

## Architecture
It is built based on Domain-Driven Design (DDD).
![img](./docs/img/architecture.png)

## Run
### Set mysql_config
```bash
touch mysql_config
```
in mysql_config file
```
MYSQL_ROOT_PASSWORD=example_password
MYSQL_USER=example
MYSQL_PASSWORD=example_pass
MYSQL_DATABASE=missions_db
MYSQL_ALLOW_EMPTY_PASSWORD=false
```
### Config .env file
```bash
touch .env
```
you need to set 3 environment  
ALLOW_ORIGIN, DATABASE_URL and FILE_PATH of CSV file
```
ALLOW_ORIGIN=http://localhost:3000
DATABASE_URL=mysql://example:example_pass@localhost:13306/missions_db
FILE_PATH=/home/my_user/develop/missions-systems/exp_table.csv
```
### Docker Container run and migration
```bash
docker compose up
```
Install sqlx_cli
```
cargo install sqlx-cli
```
```bash
sqlx database create
```
```bash
sqlx migrate run
```
### Build application
```bash
cargo build --release
```
### Start app_server
```bash
./target/release/app_server
```
### Start mission_resetter
```
./target/release/mission_resetter
```

## API
### Create user  
method: ```POST```  
url: http://localhost:8080/user  
payload:
```json
{
    "userName": "Example",
    "email": "example@mail.com",
    "password": "examplePass"
}
```
### Login  
method: ```POST```  
url: http://localhost:8080/login  
request-body:
```json
{
    "email": "example@mail.com",
    "password": "examplePass"
}
```
### Get user information  
method: ```GET```  
url: http://localhost:8080/user  
**response**-body:
```json
{
	"userId": "2c639af8-12bb-4140-a3c4-f3aa7e1c6913",
	"userName": "Example"
}
```
### Update user name
method: ```PUT```  
url: http://localhost:8080/user?user_name=update_name
### Delete user
method: ```DELETE```  
url: http://localhost:8080/user
### Create DailyMission
method: ```POST```  
url: http://localhost:8080/daily  
request-body:
```json
{
	"title": "Read book",
	"description": "Read WebAPI books"
}
```
### Get all DailyMission
method: ```GET```  
url: http://localhost/daily  
**response**-body:
```json
[
	{
		"userId": "2c639af8-12bb-4140-a3c4-f3aa7e1c6913",
		"missionId": "8b21e122-8411-42f2-9c6f-0415fe824ee1",
		"title": "Read book",
		"description": "Read WebAPI books",
		"isComplete": false
	},
	{
		"userId": "2c639af8-12bb-4140-a3c4-f3aa7e1c6913",
		"missionId": "ed78957a-70b5-46db-88e2-b9f1be89e934",
		"title": "Cooking",
		"description": null,
		"isComplete": false
	},
]
```
### Get specific DailyMission
method: ```GET```  
url: http://localhost:8080/daily/:id  
```:id``` is dynamic path. It is **missionId**.  
**response**-body:  
if url is ```http://localhost:8080/daily/8b21e122-8411-42f2-9c6f-0415fe824ee1```
```json
{
    "userId": "2c639af8-12bb-4140-a3c4-f3aa7e1c6913",
    "missionId": "8b21e122-8411-42f2-9c6f-0415fe824ee1",
    "title": "Read book",
    "description": "Read WebAPI books",
    "isComplete": false
}
```
### Update DailyMission
method: ```PUT```  
url: http://localhost:8080/daily/:id  
```:id``` is dynamic path. It is **missionId**.  
request-body:  
if you would like to update a daily mission, you need to set url like this  
```http://localhost:8080/daily/8b21e122-8411-42f2-9c6f-0415fe824ee1```and set body
```json
{
	"title": "update daily mission!",
	"description": "update!"
}
```
### Delete DailyMission
method: ```DELETE```  
url: http://localhost:8080/daily/:id  
```:id``` is dynamic path. It is **missionId**.   
if you would like to delete a daily mission, you need to set url like this  
```http://localhost:8080/daily/8b21e122-8411-42f2-9c6f-0415fe824ee1```
### Set Complete a DailyMission "complete" true
Set DailyMission's complete true.  
**Setting "complete" to true will increase the user's experience points (by 2 points) behind the scenes**
```json
{
    "userId": "2c639af8-12bb-4140-a3c4-f3aa7e1c6913",
    "missionId": "8b21e122-8411-42f2-9c6f-0415fe824ee1",
    "title": "Read book",
    "description": "Read WebAPI books",
    "isComplete": true
    //           ++++++ set true from false
}
```
method: ```PUT```  
url: http://localhost:8080/daily/complete/:id  
if you would like to set ```complete=true``` a daily mission, you need to set url like this  
```http://localhost:8080/daily/complete/8b21e122-8411-42f2-9c6f-0415fe824ee1```
### Get user's experience points and level
method: ```GET```
url: http://localhost:8080/exp  
**response-body**: 
```json
{
	"userId": "2c639af8-12bb-4140-a3c4-f3aa7e1c6913",
	"experiencePoints": 8,
	"level": 0
}
```
Experience point is defined at [CSV file](./exp_table.csv)

## MissionResetter
At 20:00 UTC (5:00 AM Japan time), an UPDATE statement is issued for all users' DailyMissions to reset isComplete to false.
