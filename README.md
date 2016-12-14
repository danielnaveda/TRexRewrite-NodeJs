# trex-rewrite-nodejs

Implement an HTTP proxy using NodeJs and Neon for TRexRewrite.

## Installation

Make sure you have installed neon:

https://github.com/neon-bindings/neon

To build the project, execute:
```
git clone https://github.com/daniel2121/TRexRewrite-NodeJs.git

cd trex-rewrite-nodejs

neon build
```

## Run Server
Once installed and in the project folder, run:
```
node lib/index.js [testing]
```

There are some dependencies in the node.js code. If you face any errors for missing dependency, install it by typing:
```
npm install <name of the dependency>
```

## Run test
Using a different terminal, run:

```
cd scripts

./testing_fire.sh
```

## Reference

* https://github.com/dippi/TRexRewrite

* https://github.com/deib-polimi/TRex

* https://github.com/neon-bindings/neon
