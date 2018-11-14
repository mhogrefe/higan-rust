#!/bin/bash
cd rust &&
bash build-and-test.sh &&
cd ../cpp &&
bash build-test-and-run.sh
