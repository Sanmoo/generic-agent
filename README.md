# Introduction

Generic Agent is a small web application written in RUST that proxies a local program's CLI through a Rest API. Once the user is authenticated, they can request that a command is made in the host machine and they can ask to be notified back when the command has finished.

Observation: This is a side project specifically built for me to train my Rust skills. This is not intended to be used in production.

# TODO

* In GET /commands/{id} show the datetimes with time information.
* Define unit test strategy and cover current code.
* Add proper logging.