var operation = require('./operations');

operation.initialize();

var express = require('express')
var bodyParser = require('body-parser');

var app = express()

app.use(bodyParser.json());

app.post('/trexrewrite', function (req, res) {
  switch (req.body.operation){
    case 'declare':
      operation.declare(req, res)
    break;
    case 'define':
      operation.define(req, res)
    break;
    case 'subscribe':
      operation.subscribe(req, res)
    break;
    case 'unsubscribe':
      operation.unsubscribe(req, res)
    break;
    case 'publish':
      operation.publish(req, res)
    break;

    case 'get_notification':
      var aaagsafsa = operation.get_notification(req, res)
      // req.body.operation = "abc";
      // res.end(JSON.stringify(req.body));

      // Prepare output in JSON format
       response = {
          result:"OK",
          description:"The test was okay"
       };
      //  console.log(response);
      //  res.end(JSON.stringify(response));
      //  res.end(JSON.stringify(res));
      res.send(aaagsafsa)

      return;
    break;

    default:
      console.log("Error: Function not supported")
  }

  res.send('Ok')
})

app.listen(3000, function () {
  console.log('Listening on port 3000');
})
