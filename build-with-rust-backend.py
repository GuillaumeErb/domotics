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
subprocess.run(["cargo", "build", "--release"],
               cwd="domotics-backend")

print(colors.BOLD + colors.PURPLE + "➡️  Building frontend ..." + colors.ENDC)
subprocess.run(["npm", "run", "build"],
               cwd="domotics-frontend")

print(colors.BOLD + colors.PURPLE + "➡️  Creating artifact ..." + colors.ENDC)
if os.path.exists("artifact") and os.path.isdir("artifact"):
    shutil.rmtree("artifact")
os.mkdir("artifact")
shutil.copy("domotics-backend/target/release/domotics-backend", "artifact")
shutil.copytree("domotics-frontend/build", "artifact/www")

print(colors.BOLD + colors.PURPLE + "➡️  Done !" + colors.ENDC)
