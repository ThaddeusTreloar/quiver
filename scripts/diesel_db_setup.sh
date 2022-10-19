#!/bin/bash

diesel setup --database-url ./run/core.sqlite
diesel migration run --database-url ./run/core.sqlite