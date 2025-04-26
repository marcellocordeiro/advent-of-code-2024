#!/usr/bin/env python3

from argparse import ArgumentParser
import subprocess
import shlex


parser = ArgumentParser()

parser.add_argument("command")
parser.add_argument("day")
parser.add_argument("part")
parser.add_argument("test")

args = parser.parse_args()

test = "::".join([args.part, "tests", args.test])

command = [
    "cargo",
    f"{args.command}",
    "--release",
    "--package",
    f"{args.day}",
    test,
    "--",
    "--exact",
    "--nocapture",
]

print(shlex.join(command))
subprocess.run(command)
