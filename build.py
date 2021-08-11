#!/usr/bin/env python3

import subprocess
import os
import shutil


class colors:
    PURPLE = '\033[95m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


print(colors.BOLD + colors.PURPLE + "➡️  Building backend ..." + colors.ENDC)
subprocess.run(["npm", "run", "build"],
               cwd="domotics-backend-node")

print(colors.BOLD + colors.PURPLE + "➡️  Building frontend ..." + colors.ENDC)
subprocess.run(["npm", "run", "build"],
               cwd="domotics-frontend")

print(colors.BOLD + colors.PURPLE + "➡️  Done !" + colors.ENDC)
