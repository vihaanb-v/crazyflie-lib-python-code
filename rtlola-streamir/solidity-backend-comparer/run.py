#!/bin/env python3

import json
import subprocess

subprocess.call(["cargo", "build", "--release"])

tests = json.load(open("tests.json", "rb"))
for test in tests:
	spec = test["spec"]
	traces = test["trace"]
	for trace in traces:
		if "streams" in test:
			args = ["../target/release/solidity_test_generator", spec, trace]
			for stream in test["streams"]:
				args.extend(["--output-streams", stream])
			subprocess.call(args)
		else:
			subprocess.call(["../target/release/solidity_test_generator", spec, trace])
