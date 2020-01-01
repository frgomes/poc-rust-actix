## Building this application

## Overview

This project starts with an OpenApi3 file which describes the conversation between server and client sides.

Ideally, it should be possible to generate all sources for this project, if we had a code generator which supports ``actix-web`` properly. We still don't have such thing. At the moment, part of the sources was automatically generated and part of the sources are edited by hand as part of our evaluation of the technology.

> At the moment, no code generation is required.

Since full code generation is not supported at the moment, you can simply skip the section below. We provide it for future reference only.

### Tooling: code generation

> At the moment, no code generation is required. You can simply skip this section.

For information purposes and future reference only, we describe below how the code generation works.

#### Install ``node``, ``npm`` and utilities by running the script below.

```bash
#!/bin/bash

wget https://raw.githubusercontent.com/frgomes/bash-scripts/master/user-install/install-node.sh
./install-node.sh
```
> This command will take a while, since various utiliy programs are installed.

By default, it will install ``node``, ``npm`` and utilities under ``${HOME}/tools/node``.

Make sure you add this location onto your ``$PATH``.

### Tooling: development environment 

> This step is required

#### Install ``rust``, ``cargo`` and utilities by running the script below.

```bash
#!/bin/bash

wget https://raw.githubusercontent.com/frgomes/bash-scripts/master/user-install/install-node.sh
./install-rust.sh
```

> This command will take a while, since various Cargo plugins are installed.

Make sure you run the command below so that the Rust environment is added to your ``$PATH``:

```bash
#!/bin/bash

source ${HOME}/.cargo/env
```


## OpenApi3 validation and code generation

> As explained in the section above, this step can be simply skipped.


Since full code generation is not supported at the moment, you can simply skip the commands below. We provide it for future reference only.

```bash
#!/bin/bash

openapi-generator validate -i openapi3.yaml && \
  openapi-generator generate -g rust --package-name poc_rust_actix -i openapi3.yaml -o .
```

__CAUTION__: This will __OVERWRITE__ all files in this folder.


## Building this application

```bash
#!/bin/bash

# check dependencies: make sure we avoid "package dependency hell"
cargo tree

# interactively test the application when sources are modified
cargo watch test
```
