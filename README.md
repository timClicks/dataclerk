> Disclaimer: `dataclerk` is currently alpha quality software. There
> are big ambitions for this small tool though, so you are very welcome
> to monitor the project.

# `dataclerk`

**A fast, reliable, small data remote logging tool with a very small footprint.
`dataclerk` is similar to a log file, but you can send it messages over the web.**

To start, we create a server that's listening locally:

```bash
$ dataclerk localhost:4499 iot-logs.sqlite
```

Its primary user interface is `curl` (or perhaps `httpie` 🦀). Assuming
a `dataclerk` server is live at `clerk.example.com:4499`, we're able to
register channels and begin to record entries:

```bash
$ curl -X PUT localhost:4499/v1/channel/mesh
$ curl localhost:4499/+/mesh -d unit=borg-h1a42 -d status=ok
```

On the backend, a new table has been created within Sqlite database at `iot-logs.sqlite` called "mesh".
The second line has created a new row within that within that table:

<table>
<tr>
<th>id
<th>created_at
<th>uuid
<th>data
</tr>
<tr>
<td>1
<td>2019-04-09 08:37:42
<td>641c8210-5aa2-11e9-8b61-674d210d2b16
<td>{"unit":"borg-h1a42","status":"ok"}
</tr>
</table>

Adding data to `dataclerk` should be very fast. You should expect a response within 10ms, although the system may be constrained by how long things take to be stored onto physical storage media. You can see from the logs generated from this session that that we used around 3-4ms to make changes:


```plain
$ dataclerk localhost:4499 iot-logs.sqlite
[2019-04-09T09:04:48Z INFO  dataclerk] Hello!
[2019-04-09T09:04:48Z INFO  actix_server::builder] Starting 12 workers
[2019-04-09T09:04:48Z INFO  actix_server::builder] Starting server on 127.0.0.1:4499
[2019-04-09T09:05:14Z INFO  dataclerk] registering channel "mesh"
[2019-04-09T09:05:14Z INFO  actix_web::middleware::logger]  "PUT /v1/channel/mesh HTTP/1.1" 201 0 "-" "curl/7.61.0" 0.033640
[2019-04-09T09:05:46Z DEBUG dataclerk] recv: channel:"mesh", data: {"status": "ok", "unit": "borg-h1a42"}
[2019-04-09T09:05:46Z INFO  actix_web::middleware::logger]  "POST /+/mesh HTTP/1.1" 204 0 "-" "curl/7.61.0" 0.04191
^C[2019-04-09T09:07:53Z INFO  actix_server::builder] SIGINT received, exiting
[2019-04-09T09:07:53Z INFO  dataclerk] Goodbye
```

## usage

`dataclerk` is a command-line tool. To run it, open a the terminal/console prompt and change into the directory where you've downloaded the `dataclerk` tool.

### getting help

To access help, at any time, run `dataclerk --help`. You will see usage instructions appear:

```plain
dataclerk v0.1
Tim McNmamara <dataclerk@timmcnamara.co.nz>
HTTP data logger

USAGE:
    dataclerk <address> <database>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <address>     Hostname/IP address and port pair for the server
                  to listen to. [default: localhost:4499]
    <database>    Database file to connect to. Will be appended to
                  if it already exists. Use :memory: for an in-
                  memory database [default: ./dataclerk.sqlite]

dataclerk receives data via HTTP POST and stores it in a sqlite
database for later analysis. All entries are stored as
well-formed JSON and tagged with a timestamp and UUID.
```

If you are having trouble that can't be answered,
by that material, you are welcome to register an issue,
send me an email or a [tweet](https://twitter.com/timClicks).

### receiving messages

To get up-and-running, provide an address for the server to listen on `<address>` and a file name `<database>` to store messages:

```bash
$ dataclerk localhost:4499 iot-logs.sqlite
            ~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~
               \             \
              <address>     <database>
```

Now, from another console, you need to 1) register a channel with a HTTP PUT request and then you can  begin to send messages by sending HTTP POST data.

## roadmap

- Enable client functionality within `dataclerk` so that sending messages can occur through a single tool and people no longer need to remember URL paths.
- As `dataclerk` grows, the intention is for the registration command to accept a schema.
  Right now, everything is stored as a JSON object that has strings as keys as values.
- Query functionality. At the moment, the analysis API is SQL queries.





## jargon

- "channel" is analogous to an [AMQP topic][], a [syslog][] facility

[syslog]: https://en.wikipedia.org/wiki/Syslog
[amqp topic]: https://www.rabbitmq.com/tutorials/tutorial-five-python.html

