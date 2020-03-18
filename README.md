# The SAFE API

| [MaidSafe website](https://maidsafe.net) | [SAFE Dev Forum](https://forum.safedev.org) | [SAFE Network Forum](https://safenetforum.org) |
|:----------------------------------------:|:-------------------------------------------:|:----------------------------------------------:|

## Table of contents

1. [Description](#description)
2. [The SAFE API](#the-safe-api-safe-api)
3. [The FFI layer](#the-ffi-layer-safe-ffi)
4. [The SAFE CLI](#the-safe-cli)
5. [The Authenticator daemon](#the-authenticator-daemon)
6. [JSON-RPC and QUIC](#json-rpc-and-quic)
7. [Further Help](#further-help)
8. [License](#license)
9. [Contributing](#contributing)

## Description

In this repository you'll find all that's needed by any application which intends to connect and read/write data on [The SAFE Network](https://safenetwork.tech).

A Rust SAFE application can make use of the `safe-api` crate to be able to not only read/write data on the SAFE Network but also to send/receive authorisation requests to the SAFE Authenticator (see https://hub.safedev.org/discover for additional info of the Authenticator).

![SAFE app authorisation flow](misc/auth-flow-diagram.png)

In addition to the `safe-api` crate to be used by Rust applications, this repository contains the [safe-ffi](safe-ffi) library and a couple of applications ([safe-authd](safe-authd) and [safe-cli](safe-cli)) which are required depending on the type of SAFE application you are developing, use case, and/or if you are just a user of the SAFE Network willing to interact with it using a simple command line interface.

The following diagram depicts how each of the artifacts of this repository fit in the SAFE applications ecosystem. You can find more information about each of them further below in the next section of this document.

![SAFE API ecosystem](misc/safe-api-ecosystem.png)

## The SAFE API ([safe-api](safe-api))

The [safe-api](safe-api) is a Rust crate which exposes the SAFE API with all the functions needed to communicate with the SAFE Network and the SAFE Authenticator. If you are developing a Rust application for SAFE, this is all you need as a dependency from your app.

## The FFI layer ([safe-ffi](safe-ffi))

The [safe-ffi](safe-ffi) is a Rust crate exposing the same functions as the SAFE API (`safe-api`) but in the form of an interface which can be consumed from other programming languages like C, this is achieved by the use of the [Rust FFI feature](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code).

Therefore, if you are developing a SAFE application using a different programming language than Rust, this is the crate you need to access the SAFE API. This crate also provides scripts to automatically generate the binding libraries for some languages like Java and C#.

## The SAFE CLI

The [safe-cli](safe-cli) is a Rust application which implements a CLI (Command Line Interface) for the SAFE Network.

![SAFE CLI](misc/safe-cli-animation.svg)

The SAFE CLI provides all the tools necessary to interact with the SAFE Network, including storing and browsing data of any kind, following links that are contained in the data and using their addresses on the network, using safecoin wallets, and much more. Using the CLI users have access to any type of operation that can be made on the SAFE Network and the data stored on it.

If you are just a SAFE user, or a system engineer creating automated scripts, this application provides you with all you need to interact with the SAFE Network. Please refer to [The SAFE CLI User Guide](safe-cli/README.md) to learn how to start using it.

## The Authenticator daemon

The [safe-authd](safe-authd) is a SAFE Authenticator implementation which runs in the background a daemon on Linux and Mac, or as a service in Windows platforms.

The SAFE Authenticator gives complete control over the type of access and permissions that are granted to the applications used by the SAFE users. Any application that is intending to write data on the Network on behalf of the user needs to get credentials which are authorised by the user, and the SAFE Authenticator is the component which facilitates such mechanism.

This application is normally shipped as part of the package of an Authenticator GUI, like the [SAFE Network Application](), and therefore SAFE users and SAFE app developers don't need it or worry about since the SAFE API already provides functions to interact with the `safe-authd`, and the SAFE CLI also has commands to do so.

## JSON-RPC and QUIC

One last crate found in this repository is the [jsonrpc-quic](jsonrpc-quic). This crate provides the implementation of [JSON-RPC](https://www.jsonrpc.org/) over [QUIC](https://en.wikipedia.org/wiki/QUIC), which is required by the Authenticator daemon communication protocol.

This crate exposes a minimised set of functions which are used by other crates to implement the Authenticator daemon communication protocol. On one hand the `safe-api` makes use of it to be able to send JSON-RPC messages to the `authd` over QUIC, and on the other hand the `safe-authd` makes use of it to accept those requests from clients, generating and sending back a JSON-RPC response over QUIC. Please refer to the [safe-authd README](safe-authd/README.md) to see some examples of these type of requests/responses.

## Further Help

You can discuss development-related questions on the [SAFE Dev Forum](https://forum.safedev.org/).
If you are just starting to develop an application for the SAFE Network, it's very advisable to visit the [SAFE Network Dev Hub](https://hub.safedev.org) where you will find a lot of relevant information.

## License

This SAFE Network library is dual-licensed under the Modified BSD ([LICENSE-BSD](LICENSE-BSD) https://opensource.org/licenses/BSD-3-Clause) or the MIT license ([LICENSE-MIT](LICENSE-MIT) https://opensource.org/licenses/MIT) at your option.

## Contributing

Want to contribute? Great :tada:

There are many ways to give back to the project, whether it be writing new code, fixing bugs, or just reporting errors. All forms of contributions are encouraged!

For instructions on how to contribute, see our [Guide to contributing](https://github.com/maidsafe/QA/blob/master/CONTRIBUTING.md).
