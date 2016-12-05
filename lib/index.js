var express = require('express')
var bodyParser = require('body-parser');
// var operation = require('./operations');
var trex = require('./operations');
// var trex = require('./trex.js');
var colors = require('colors/safe'); // does not alter string prototype

// var morgan = require('morgan')

var app = express()


trex.initialize();

app.use(bodyParser.json());

// app.use(morgan('combined'))

// Middleware function to debug
app.use(function (req, res, next) {
  // console.log(colors.black("|--------------- New Request ---------------|"));
  // console.log(colors.red("|--------------- New Request ---------------|"));
  console.log(colors.green("|--------------- New Request ---------------|"));
  // console.log(colors.yellow("|--------------- New Request ---------------|"));
  // console.log(colors.blue("|--------------- New Request ---------------|"));
  // console.log(colors.magenta("|--------------- New Request ---------------|"));
  // console.log(colors.cyan("|--------------- New Request ---------------|"));
  // console.log(colors.white("|--------------- New Request ---------------|"));
  // console.log(colors.gray("|--------------- New Request ---------------|"));
  // console.log(colors.grey("|--------------- New Request ---------------|"));
  console.log('Request:', req.method, req.url);
  console.log('Body:', req.body);
  // console.log(req.url);
  next()
})

/*
app.post('/trexrewrite', function (req, res) {

  response = {
     result:"Error",
     description:"Internal errors"
  };

  response = JSON.stringify(response);

  switch (req.body.operation){
    //TODO: Implement a proper call to these functions
    // case 'declare':
    //   operation.declare(req, res);
    // break;
    // case 'define':
    //   operation.define(req, res);
    // break;
    case 'subscribe':
      operation.subscribe(req, res);
    break;
    // case 'unsubscribe':
    //   operation.unsubscribe(req, res);
    // break;
    case 'publish':
      operation.publish(req, res);
      response = {result:"Ok"};response = JSON.stringify(response);
    break;
    case 'get_notification':
      var response = operation.get_notification(req, res);
    break;
    default:
      console.log("Error: Function not supported");
  }

  res.end(response);
})
*/

app.get('/connections', function(req, res, next) {
    // console.log("--------------------------------");
    // console.log(req.body);
    // console.log("GET /connections");
    res.json(trex.getConnection());
    // res.send('Hello World');
    // return res.send('Under construction');
    // console.log('Response Body:', res.json);
    next()
});

app.post('/subscriptions/:connID', function(req, res, next) {
    // console.log("--------------------------------");
    // console.log(req.body);
    // console.log("POST /subscriptions");
  //   console.log('\tclient '+req.params.connID+' sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getConnection(req.params.connID);
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   conn.subscribe(req.body);
  //   res.status(200);
  //   res.json(true);

  // var conn = subscribe(req.params.connID);
  res.json(trex.subscribe(req.params.connID));
  next();
  // return res.send('Under construction');
});

app.delete('/subscriptions/:connID', function(req, res) {
    // console.log("--------------------------------");
    // console.log("DELETE /subscriptions");
  //   console.log('\tclient '+req.params.connID+' sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getConnection(req.params.connID);
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   conn.unsubscribe(req.body);
  //   res.status(200);
  //   res.json(true);
  return res.send('Under construction');
});

app.get('/events/:connID', function(req, res) {
    // console.log("--------------------------------");
    // console.log("GET /events/:connID");
  //   console.log('\tclient '+req.params.connID+' sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getConnection(req.params.connID);
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   if(typeof events[req.params.connID] === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   if(events[req.params.connID].length==0) {
	// res.status(204);
	// return res.send();
  //   }
  //   res.status(200);
  //   res.json(events[req.params.connID].shift());

  // return res.send('Under construction');
  res.json(trex.getNotification(req.params.connID));
});

app.post('/events/:connID', function(req, res) {
    // console.log("--------------------------------");
    // console.log("POST /events/:connID");
  //   console.log('\tclient '+req.params.connID+' sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getConnection(req.params.connID);
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   conn.publish(req.body);
  //   res.status(200);
  //   res.json(true);

  // return res.send('Under construction');
  res.json(trex.publish(req.params.connID, req.body.value));
});

////////
// Special publish service for anonymous publishers
////////
app.post('/events', function(req, res) {
    // console.log("--------------------------------");
    // console.log("POST /events");
  //   console.log('\tanonymous client sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getDefaultConnection();
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No default connection found');
  //   }
  //   conn.publish(req.body);
  //   res.status(200);
  //   res.json(true);
  return res.send('Under construction');
});

app.get('/status', function(req, res) {
    // console.log("--------------------------------");
    // console.log("GET /status");
    trex.status();
    return res.send('Ok');
});


app.listen(8888, function () {
  console.log('Listening on port 8888');
})
