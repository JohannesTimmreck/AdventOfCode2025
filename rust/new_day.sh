#!/bin/bash

DayName=$1

cp -r ./Template ./${DayName}

echo """[package]
name = \"${DayName}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
""" > ./${DayName}/Cargo.toml

