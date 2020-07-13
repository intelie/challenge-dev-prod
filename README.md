# Convert a CSV stream to JSONL text

In this challenge you will process a CSV-like data stream through TCP and append it in realtime to a file as [JSON Lines](http://jsonlines.org/).

## Requirements
 - Linux: Ubuntu 18+ recommended (can be a virtual machine).
 - Docker + compose: [Install Docker Engine on Ubuntu](https://docs.docker.com/engine/install/ubuntu/), [Install Docker Compose](https://docs.docker.com/compose/install/)

## Setup

To start the datasource with random data:
```bash
docker-compose up
```

Wait until the code compiles and the service shows `Listening at tcp://0.0.0.0:9999`

Then you can test it with:
```bash
nc localhost 9999
```

Example:
```
leomar@intelie:dev-challenge$ nc localhost 9999
2020-07-13T15:06:58.872-03:00,0455,0.996195,"(a=0110110011010110,b=8d567ba0-002d0bd4::8d567ba0fd2f2bf6;c=8d476ab1)"
2020-07-13T15:06:59.374-03:00,0456,0.994522,"(a=0001111110110000,b=c26f1757-00d5a6c4::c26f175774f7a6e6;c=c27e0646)"
...
```

## Challenge

The csv-like stream generates 2 rows per second with the following columns:
1. **timestamp** - [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) timestamp to be converted to **milliseconds** of [unix time](https://en.wikipedia.org/wiki/Unix_time) ([example](https://currentmillis.com/));
2. **index** - An integer that increases at each event;
3. **signalwave** - A floating point with a [sine wave](https://en.wikipedia.org/wiki/Sine) from a sensor;
4. **metadata** - key/value list that needs to be converted to json map objects (*hint: you can use regular expressions*).

If you receive these rows:
```csv
2020-07-13T15:06:58.872-03:00,0455,0.996195,"(a=0110110011010110,b=8d567ba0-002d0bd4::8d567ba0fd2f2bf6;c=8d476ab1)"
2020-07-13T15:06:59.374-03:00,0456,0.994522,"(a=0001111110110000,b=c26f1757-00d5a6c4::c26f175774f7a6e6;c=c27e0646)"
2020-07-13T18:24:19.921Z,1676,-0.173648,"(a=1100010101110110,b=1d6c2fab-00d1e736::1d6c2fab2bf3e736;c=1d7d3eba)"
```
Your code should **append** the following lines to a JSON file:
```json
{"timestamp": 1594663618872, "index": 455, "signalwave": 0.996195, "metadata": { "a": "0110110011010110", "b": ["8d567ba0", "002d0bd4", "8d567ba0fd2f2bf6"],  "c": "8d476ab1"}}
{"timestamp": 1594663619374, "index": 456, "signalwave": 0.994522, "metadata": { "a": "0001111110110000", "b": ["c26f1757", "00d5a6c4", "c26f175774f7a6e6"],  "c": "c27e0646"}}
{"timestamp": 1594664659921, "index": 1676, "signalwave": -0.173648, "metadata": { "a": "1100010101110110", "b": ["1d6c2fab", "00d1e736", "1d6c2fab2bf3e736"],  "c": "1d7d3eba"}}
```

You can use the language you want: C/C++, Go, Java, JavaScript (Node.js/Deno), Python 3.6+, Rust, etc, but you need to send us the code and instructions so we are able to deploy in a clean Ubuntu VM.

## Bonus

1. Create a container to compile/run your code and add it to the **docker-compose.yml**.
2. After appending the object to the json file, send it to a [mongodb container](https://hub.docker.com/_/mongo) so anyone can see it with [mongo-express web interface](https://hub.docker.com/_/mongo-express).

Also remember to add the instructions how to use it.

## Solve this challenge

To solve this challenge, you may fork this repository, then send us a link with your implementation. Alternatively, if you do not want to have this repo on your profile (we totally get it), send us a [git patch file](https://www.devroom.io/2009/10/26/how-to-create-and-apply-a-patch-with-git/) with your changes.

If you are already in the hiring process, you may send it to whoever is your contact at Intelie. If you wish to apply for a job at Intelie, please send your solution to [trabalhe@intelie.com.br](mailto:trabalhe@intelie.com.br).
