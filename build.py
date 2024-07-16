import datetime, time, os, toml

version = toml.loads(open('Cargo.toml', 'r').read())['package']['version']

# get content of src directory
dir_content = os.listdir("./src")

try:
    os.mkdir("src/metadata")
except:
    pass
# Remove info files and main.rs from modules list
ignore_files = ['main.rs', 'metadata', 'utils.rs']

for item in ignore_files:
    try:
        dir_content.remove(item)
    except:
        continue

# remove file extensions
i = 0
for item in dir_content:
    item = os.path.splitext(item)
    dir_content[i] = item[0]
    i += 1

# write to modules
with open('src/metadata/modules', 'w') as f:
    f.write(",".join(dir_content))
    f.close()

with open('src/metadata/version', 'w') as f:
    f.write(version)
    f.close()

with open('src/metadata/build', 'w') as f:
    f.write(f"{datetime.datetime.now()} {time.tzname[0]}")
    f.close()
os.system("cargo build --release")
print("Executable is built!")
