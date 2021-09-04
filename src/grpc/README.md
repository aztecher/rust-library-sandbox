# gRPC + Tonic Sandbox

Please refer to

* [https://github.com/hyperium/tonic](https://github.com/hyperium/tonic)
* [https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

## Example

In crate root.  start up server by this command.

```
cargo run --bin helloworld-server
```

and if your server was already installed `grpcurl`, you can test the ping-poing with grpc by bellow command.  

```
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' localhost:50051 helloworld.Greeter/SayHello
```

in my sandbox code, gRPC server bind the `localhost:50051` address/port.  

or, you can execute client call by src/grpc/client.rs's implementation

```
cargo run --bin helloworld-client
```

for more information, you can see in Cargo.toml
