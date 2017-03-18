# trex-rewrite-nodejs

Implement an HTTP proxy using NodeJs and Neon for TRexRewrite.

## Installation

Make sure to have installed these three main dependencies:

* [neon](https://github.com/neon-bindings/neon)
* [Rust](https://www.rust-lang.org)
* [node](https://nodejs.org)


To build the project, execute:
```
git clone https://github.com/daniel2121/TRexRewrite-NodeJs.git

cd trex-rewrite-nodejs

neon build
```

### Detailed Installation procedure on Ubuntu/Mint:
Node Js:
```
  sudo apt-get install nodejs
  sudo ln -s `which nodejs` /usr/bin/node
  sudo apt-get install npm
```
Neon:
```
  sudo npm install -g neon-cli
```

Rust:
```
  curl -sf -L https://static.rust-lang.org/rustup.sh | sh
```

Sqlite:
```
sudo apt-get install libsqlite3-dev
```

TRex:
```
  git clone https://github.com/daniel2121/TRexRewrite-NodeJs.git

  cd TRexRewrite-NodeJs/

  neon build
```

NodeJs dependencies:
```
npm install express
npm install body-parser
npm install colors
```



## Run Server
Once installed and in the project folder, run:
```
node lib/index.js [testing]
```
Note: to run the test use the optional parameter "testing" (i.e. "node lib/index.js testing")

## Run test
Using a different terminal, run:

```
cd scripts/tools/

./testing_fire.sh
```

## Reference

* https://github.com/dippi/TRexRewrite

* https://github.com/deib-polimi/TRex

* https://github.com/neon-bindings/neon

* https://www.rust-lang.org
