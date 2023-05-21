# Voca API

Simple api for BARS (en to ru dictionary) based on *actix-web* with *diesel*. This project provide basic functionality of CRUD operations on database records.


## Getting Started

First of all you need to install [Diesel](https://github.com/diesel-rs/diesel) and [Actix](https://actix.rs/docs/getting-started)

Then you need to apply __diesel migration__:
```
$>diesel migration run
```

After you could run up the server, simply run it:
```
$>cargo run
```

## Endpoints

All endpoints with example documented in postman collection with environment file. You could find them [here](https://github.com/FedirM/Voca-API/tree/master/postman).

### Echo

| METHOD | POST |
| URL | {host}/echo |
| PARAMS | ANYTHING |

As soon as you ran the server you may want to test it. The simpliest way to do this is the **echo request**. This call return whatever you put into the request body.

### Get word\'s nest

| METHOD | GET |
| URL | {host}/word |
| PARAMS | word: STRING |

Get the nest of the word from dictionary.

### Add new word\'s nest

| METHOD | POST |
| URL | {host}/word |
| PARAMS | en: STRING; tr: STRING | 

PARAMS:
- **en** - english word or phrase.
- **tr** - translation for the word/phrase.

Add new item to dictionary.

### Edit word\'s nest

| METHOD | PATCH |
| URL | {host}/word |
| PARAMS | id: NUMBER; en: STRING; tr: STRING | 

PARAMS:
- **id** - nest identificator.
- **en** - english word or phrase.
- **tr** - translation for the word/phrase.

Update item into dictionary.