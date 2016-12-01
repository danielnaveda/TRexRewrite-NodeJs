var operation = require('./operations');

operation.initialize();

var express = require('express')
var bodyParser = require('body-parser');

var app = express()

app.use(bodyParser.json());

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

app.listen(3000, function () {
  console.log('Listening on port 3000');
})
