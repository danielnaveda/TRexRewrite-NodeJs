var express = require('express')
var bodyParser = require('body-parser');
var trex = require('./operations');
var colors = require('colors/safe'); // does not alter string prototype

var app = express()

trex.initialize();

app.use(bodyParser.json());

// Middleware function to debug
app.use(function (req, res, next) {
  // console.log(colors.black("|--------------- New Request ---------------|"));
  // console.log(colors.red("|--------------- New Request ---------------|"));
  // console.log(colors.yellow("|--------------- New Request ---------------|"));
  // console.log(colors.blue("|--------------- New Request ---------------|"));
  // console.log(colors.magenta("|--------------- New Request ---------------|"));
  // console.log(colors.cyan("|--------------- New Request ---------------|"));
  // console.log(colors.white("|--------------- New Request ---------------|"));
  // console.log(colors.gray("|--------------- New Request ---------------|"));
  // console.log(colors.grey("|--------------- New Request ---------------|"));
  console.log(colors.green('|--------- Request:', req.method, req.url, '---------|'));
  // console.log('Request:', req.method, req.url);
  console.log('Body:', req.body);
  next()
})

//This function seems to be unnecessary under the new TRex
app.get('/connections', function(req, res) {
    res.json(
      {"result": "Ok",
       "value": trex.getConnection()}
    );
});

// app.post('/subscriptions/:connID', function(req, res) {
//   res.json(
//     {"result": "Ok",
//      "value": trex.subscribe(req.params.connID)}
//   );
// });
app.post('/subscriptions', function(req, res) {
  res.json(
    {"result": "Ok",
     "value": trex.subscribe()}
  );
});

app.delete('/subscriptions/:connID', function(req, res) {
  // res.json(
  //   {"result": "Error",
  //    "description": "Operation under construction"}
  // );
  res.json(
    {"result": "Ok",
     "value": trex.unsubscribe(parseInt(req.params.connID))}
  );
});

app.get('/events/:connID', function(req, res) {
  // res.json(trex.getNotification(req.params.connID));
  res.json(
    {"result": "Ok",
    //  "value": trex.getNotification(req.params.connID)}
     "value": trex.getNotification(parseInt(req.params.connID))}
  );
});

app.post('/events/:connID', function(req, res) {
  // res.json(trex.publish(req.params.connID, req.body.value));
  res.json(
    {"result": "Ok",
     "value": trex.unknown_publish(req.params.connID, req.body.type, req.body.area)}
  );
});

// Special publish service for anonymous publishers
app.post('/events', function(req, res) {
  // res.json(
  //   {"result": "Error",
  //    "description": "Operation under construction"}
  // );
  res.json(
    {"result": "Ok",
     "value": trex.unknown_publish(req.body.type, req.body.area)}
  );
});

app.get('/status', function(req, res) {
    trex.status();
    // return res.send('Ok');
    res.json(
      {"result": "Ok"}
    );
});

app.listen(8888, function () {
  console.log('Listening on port 8888');
})
