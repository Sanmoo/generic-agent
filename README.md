# Introduction

Generic Agent is a small web application written in RUST that was built in order to expose a REST API over a CLI. Once the user is authenticated, they can request that a command is made in the host machine and they can ask to be notified back when the command has finished.

Observation: This is a side project specifically built for me to train my Rust skills. This is not intended to be used in production.

# TODO

* Understand the web framework minimally.
* Build the first API: POST /commands. Do nothing yet.
* Build the second API GET /commands/{id}. Do nothing yet.
* Specifiy the contract for the APIs by versioning an openapi file.
* Add toml configuration file and implement first logic in POST /commands.