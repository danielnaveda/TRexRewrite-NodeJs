var express = require('express')
var bodyParser = require('body-parser');
// var operation = require('./operations');
var trex = require('./operations');
// var trex = require('./trex.js');
var app = express()


trex.initialize();

app.use(bodyParser.json());
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

app.get('/connections', function(req, res) {
    console.log("GET /connections");
    // res.json(getConnection());
    return res.send('Under construction');
});

app.post('/subscriptions/:connID', function(req, res) {
    console.log("POST /subscriptions");
  //   console.log('\tclient '+req.params.connID+' sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getConnection(req.params.connID);
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   conn.subscribe(req.body);
  //   res.status(200);
  //   res.json(true);
  return res.send('Under construction');
});

app.delete('/subscriptions/:connID', function(req, res) {
    console.log("DELETE /subscriptions");
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
    console.log("GET /events");
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
  return res.send('Under construction');
});

app.post('/events/:connID', function(req, res) {
    console.log("POST /events");
  //   console.log('\tclient '+req.params.connID+' sent "'+JSON.stringify(req.body)+'" data');
  //   var conn = getConnection(req.params.connID);
  //   if(typeof conn === 'undefined') {
	// res.status(404);
	// return res.send('Error 404: No connection ID found');
  //   }
  //   conn.publish(req.body);
  //   res.status(200);
  //   res.json(true);
  return res.send('Under construction');
});

////////
// Special publish service for anonymous publishers
////////
app.post('/events', function(req, res) {
    console.log("POST /events");
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

app.listen(8888, function () {
  console.log('Listening on port 8888');
})
