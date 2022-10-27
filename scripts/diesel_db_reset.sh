#!/bin/bash

diesel migration revert --database-url ./run/core.sqlite
diesel migration run --database-url ./run/core.sqlite