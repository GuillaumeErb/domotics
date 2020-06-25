# domotics

Project aiming ot build a website and a set of APIs to control Xiaomi Mi lights from a browser.
The backend is build in Rust, exposing APIs and serving the website using Rocket.
It communicates with the light bulbs over TCP and can discover them using UDP broadcast.
The frontend is a basic React app using Typescript.

To build and serve, run ./build.py, then execute in artifact ./domotics-backend

In a development context, run cargo run in the backend, and yarn install; yarn start in the frontent.
