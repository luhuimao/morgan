## Minimal Morgan Docker image
This image is automatically updated by CI

https://hub.docker.com/r/morganlabs/morgan/

### Usage:
Run the latest beta image:
```bash
$ docker run --rm -p 10099:10099 morganlabs/morgan:beta
```

Run the latest edge image:
```bash
$ docker run --rm -p 10099:10099 morganlabs/morgan:edge
```

Port *10099* is the JSON RPC port, which is used by clients to communicate with the network.
