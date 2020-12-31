# tabu

A simple banned word management system to showcase how CRUD operations can
be done with [warp][1], [tokio][2], and [mobc][3] .

## Requirements

- docker
- docker-compose
- curl

## How to Run

1. Instantiate a local cluster with the following command

```
$ docker-compose up -d
```

2. Insert some entries with `curl`. For example,

```
$ curl -X POST -H 'Content-Type: application/json' \
 --data-raw '{"words":"republican","reason":"dissent"}' \
 'http://localhost:8080/'

$ curl -X POST -H 'Content-Type: application/json' \
 --data-raw '{"words":"abuse","reason":"violence"}' \
 'http://localhost:8080/'
```

3. Query all existing entries with `curl`.

```
$ curl -X GET 'http://localhost:8080/'
```

4. Existing entries can be deleted by sending requests like this

```
$ curl -X DELETE -H 'Content-Type: application/json' \
 --data-raw '{"words":"abuse"}' \
 'http://localhost:8080/'
```

5. You could also update existing entries in the following fashion

```
$ curl -X PUT -H 'Content-Type: application/json' \
 --data-raw '{"words":"republican","reason":"dissent,obstruction"}' \
 'http://localhost:8080/'
```

Have fun!

---
[1]:https://docs.rs/warp/
[2]:https://docs.rs/tokio/
[3]:https://docs.rs/mobc/
