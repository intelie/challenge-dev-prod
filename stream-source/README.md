## To compile / build image
```bash
docker build --tag stream-source .
```

## To test
- start the container
```bash
docker run --rm --name stream-source -p 9999:9999 stream-source
```

- do the tests
```bash
nc localhost 9999
```

- stop the container
```bash
docker stop stream-source
```