# Verifiable Data Registry Tools (VDR Tools)

Evernym VDR Tools is a Rust library that makes it easy to perform SSI related ledger operations across a variety of ledgers. The library implements [Hyperledger Aries](http://github.com/hyperledger/aries-rfcs) in a format suitable for reuse in many scenarios, thus increasing consistency, compatibility, and correctness.

Key features include:

* Support for cryptographic operations to sign credentials with multiple signature types.
* Key storage in SQLite or an external database.
* Common functions for Aries protocols such as message packing.
* A flexible threading model.
* Payment API for including token based payments as part of exchange.
* Built for multiple platforms and includes API wrappers in additional languages.
* Apache licensed.

## Evolution

Hyperledger Aries is creating a standard way to interact with distributed ledgers as Verifiable Data Registries (VDRs) and a standard approach to key storage. VDR Tools is an evolution of Hyperledger Indy SDK which remains compatible with the Aries ecosystem.

Evernym was a key maintainer of the [Hyperledger Indy SDK](https://github.com/hyperledger/indy-sdk), but over time our needs diverged from the broader Hyperledger community. We realized that we need a library tailored to the unique requirements of our products which we can iterate quickly without disrupting other members of the ecosystem. We do not expect to replace other Aries libraries. The goal of this project is to provide an alternative specific to the architectural requirements of our products.

## Roadmap

* Internal renaming from Indy SDK / LibIndy to Evernym VDR Tools.
* Removal of legacy components: Indy CLI, LibVCX, and unused wrappers.
* Support for multiple ledgers, including Indy ledgers like Sovrin and IDUnion.
* Support for additional signature types such as BBS+.


# VDR Tools SDK

## Items included in this SDK

### libindy

The major artifact of the SDK is a C-callable library that provides the basic building blocks for
the creation of applications on the top of [Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy).
It is available for most popular desktop, mobile and server platforms.

### Libindy wrappers

A set of libindy wrappers for developing Indy-based applications in your favorite programming language.
Indy SDK provides libindy wrappers for the following programming languages and platforms:

* [Java](wrappers/java/README.md)
* [Python](wrappers/python/README.md)
* [NodeJS](wrappers/nodejs/README.md)
* [.Net](wrappers/dotnet/README.md)
* [Rust](wrappers/rust/README.md)

### Libnullpay

[Libnullpay](/libnullpay/README.md) is a libindy plugin that can be used for development of applications that use the Payments API of Indy SDK.

## How-To Tutorials
Short, simple tutorials that demonstrate how to accomplish common tasks
are also available. See the [docs/how-tos](docs/how-tos) folder.

1. [Write a DID and Query Its Verkey](docs/how-tos/write-did-and-query-verkey/README.md)
2. [Rotate a Key](docs/how-tos/rotate-key/README.md)
3. [Save a Schema and Cred Def](docs/how-tos/save-schema-and-cred-def/README.md)
4. [Issue a Credential](docs/how-tos/issue-credential/README.md)
5. [Negotiate a Proof](docs/how-tos/negotiate-proof/README.md)
6. [Send a Secure Message](docs/how-tos/send-secure-msg/README.md)

## Installing the SDK
### Release channels
The Indy SDK release process defines the following release channels:

* `master` - development builds for each push to master branch.
* `rc` - release candidates.
* `stable` - stable releases.

Please refer to our [release workflow](docs/contributors/release-workflow.md) for more details.

### Ubuntu based distributions (Ubuntu 16.04 and 18.04)
It is recommended to install the SDK packages with APT:

    sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb (xenial|bionic) {release channel}"
    sudo apt-get update
    sudo apt-get install -y {library}

* {library} must be replaced with libindy, libnullpay, libvcx or indy-cli.
* (xenial|bionic) xenial for 16.04 Ubuntu and bionic for 18.04 Ubuntu.
* {release channel} must be replaced with master, rc or stable to define corresponded release channel.
Please See the section "Release channels" above for more details.

### Windows

1. Download last version of library.
2. Unzip archives to the directory where you want to save working library.
3. After unzip you will get next structure of files:

* `Your working directory for libindy`
    * `include`
        * `...`
    * `lib`
        * `indy.dll`
        * `libeay32md.dll`
        * `libsodium.dll`
        * `libzmq.dll`
        * `ssleay32md.dll`

`include` contains c-header files which contains all necessary declarations
that may be need for your applications.

`lib` contains all necessary binaries which contains libindy and all it's dependencies.
 `You must add to PATH environment variable path to lib`. It's necessary for dynamic linkage
 your application with libindy.

{release channel} must be replaced with master, rc or stable to define corresponded release channel.
See section "Release channels" for more details.

{library} must be replaced with libindy, libnullpay, libvcx or indy-cli.

### MacOS

1. Download the latest version of library.
2. Unzip archives to the directory where you want to save working library.
3. After unzip you will get next structure of files:

* `Your working directory`
    * `include` - contains c-header files which contains all necessary declarations that may be need for your applications.
        * `...`
    * `lib` - contains library binaries (static and dynamic).
        * `library.a`
        * `library.dylib`
    
4. Install dependent libraries: libsodium, zeromq, openssl. The dependent libraries should match the version with what you can find from ``otool -L libindy.dylib``.

You need add the path to lib folder to LIBRARY_PATH environment variable. 
    
{library} must be replaced with libindy, libnullpay, libvcx or indy-cli to define corresponded library.

{release channel} must be replaced with master, rc or stable to define corresponded release channel.
    
## How to build Indy SDK from source

* [Ubuntu based distributions (Ubuntu 16.04)](docs/build-guides/ubuntu-build.md)
* [RHEL based distributions (Centos)](docs/build-guides/rhel-build.md)
* [Windows](docs/build-guides/windows-build.md)
* [MacOS](docs/build-guides/mac-build.md)
* [Android](docs/build-guides/android-build.md)

**Note:**
By default `cargo build` produce debug artifacts with a large amount of run-time checks.
It's good for development, but this build can be in 100+ times slower for some math calculation.
If you would like to analyse CPU performance of libindy for your use case, you have to use release artifacts (`cargo build --release`).

## How to start local nodes pool with docker
To test the SDK codebase with a virtual Indy node network, you can start a pool of local nodes using docker:

**Note: If you are getting a PoolLedgerTimeout error it's because the IP addresses in
cli/docker_pool_transactions_genesis and the pool configuration don't match.
Use method 3 to configure the IPs of the docker containers to match the pool.**

### 1) Starting the test pool on localhost
Start the pool of local nodes on `127.0.0.1:9701-9708` with Docker by running:

```
docker build -f ci/indy-pool.dockerfile -t indy_pool .
docker run -itd -p 9701-9708:9701-9708 indy_pool
```

### 2) Starting the test pool on a specific IP address
 Dockerfile `ci/indy-pool.dockerfile` supports an optional pool_ip param that allows
 changing ip of pool nodes in generated pool configuration.

 You can start the pool with e.g. with the IP address of your development machine's WIFI interface
 so that mobile apps in the same network can reach the pool.

 ```
 # replace 192.168.179.90 with your wifi IP address
 docker build --build-arg pool_ip=192.168.179.90 -f ci/indy-pool.dockerfile -t indy_pool .
 docker run -itd -p 192.168.179.90:9701-9708:9701-9708 indy_pool
 ```
 To connect to the pool the IP addresses in /var/lib/indy/sandbox/pool_transactions_genesis (in docker) and
 the pool configuration you use in your mobile app must match.

### 3) Starting the test pool on a docker network
 The following commands allow to start local nodes pool in custom docker network and access this pool
 by custom ip in docker network:

 ```
 docker network create --subnet 10.0.0.0/8 indy_pool_network
 docker build --build-arg pool_ip=10.0.0.2 -f ci/indy-pool.dockerfile -t indy_pool .
 docker run -d --ip="10.0.0.2" --net=indy_pool_network indy_pool
 ```
 Note that for Windows and MacOS this approach has some issues. Docker for these OS run in
 their virtual environment. First command creates network for container and host can't
 get access to that network because container placed on virtual machine. You must appropriate set up
 networking on your virtual environment. See the instructions for MacOS below.

### Docker port mapping on MacOS

If you use some Docker distribution based on Virtual Box you can use Virtual Box's
port forwarding future to map 9701-9709 container ports to local 9701-9709 ports.

If you use VMWare Fusion to run Docker locally, follow the instructions from
[this article](https://medium.com/@tuweizhong/how-to-setup-port-forward-at-vmware-fusion-8-for-os-x-742ad6ca1344)
and add the following lines to _/Library/Preferences/VMware Fusion/vmnet8/nat.conf_:

```
# Use these with care - anyone can enter into your VM through these...
# The format and example are as follows:
#<external port number> = <VM's IP address>:<VM's port number>
#8080 = 172.16.3.128:80
9701 = <your_docker_ip>:9701
9702 = <your_docker_ip>:9702
9703 = <your_docker_ip>:9703
9704 = <your_docker_ip>:9704
9705 = <your_docker_ip>:9705
9706 = <your_docker_ip>:9706
9707 = <your_docker_ip>:9707
9708 = <your_docker_ip>:9708
9709 = <your_docker_ip>:9709
```
where <your_docker_ip> is your Docker host IP.

Docker machine needs to be rebooted after these changes.

## Wrappers documentation

The following [wrappers](docs/architecture/language-bindings.md) are tested and complete. 

## How to migrate
The documents that provide necessary information for Libindy migrations.
 
* [v1.14.0 → v1.15.x](docs/migration-guides/migration-guide-1.14.0-1.15.0.md)

## How to Contribute
We love Merge Requests. During the MR review process, you will be asked to agree to the Evernym CLA.

#### Notes
* Libindy implements multithreading approach based on **mpsc channels**.
If your application needs to use Libindy from multiple processes you should keep in mind the following restrictions:
    * Fork - duplicates only the main thread. So, child threads will not be duplicated.
      If any out-of-process requirements are possible, the caller must fork first **before any calls to Libindy**
      (otherwise the command from a child thread will hang). Fork is only available on Unix.
    * Popen - spawns a new OS level process which will create its own child threads. Popen is cross-platform.
