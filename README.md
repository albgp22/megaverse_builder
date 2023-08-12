# Megaverse builder tool


## Usage

```{bash}
docker run --rm "$(docker build -q .)" --help
```

```{text}
Simple API client for building the MEGAVERSE!

Usage: megaverse-builder [OPTIONS]

Options:
  -p, --phase <PHASE>              Problem phase to automatically solve [default: 2]
  -c, --config-file <CONFIG_FILE>  Config file path. Doesn't matter wether absolute or relative [default: ./config.json]
  -h, --help                       Print help
  -V, --version                    Print version
```