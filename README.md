## MPD forwarder (sync version)

 A simple proxy that forwards TCP payload to an MPD server.

 This is the sync version, using basic networking I/O.

### Run with:

    $ cargo watch -x run

### Send cmd with:

    $ echo "status" | ncat 127.0.0.1 6601
