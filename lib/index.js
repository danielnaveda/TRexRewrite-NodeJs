var express = require('express')
var bodyParser = require('body-parser');
var trex = require('./trex');
var colors = require('colors/safe');

var app = express()

trex.init_examples();//For testing purposes

app.use(bodyParser.json());

// Middleware function to debug
app.use(function(req, res, next) {
    console.log(colors.green('|--------- Request:', req.method, req.url, '---------|'));
    console.log('Body:', req.body);
    next()
})

//This function seems to be unnecessary under the new TRex
app.get('/connections', function(req, res) {
    res.json({
        "result": "Ok",
        "value": trex.getConnection()
    });
});

// app.post('/subscriptions/:connID', function(req, res) {
//   res.json(
//     {"result": "Ok",
//      "value": trex.subscribe(req.params.connID)}
//   );
// });

//TODO: This function should consider the :connID
app.post('/subscriptions', function(req, res) {
    res.json({
        "result": "Ok",
        "value": trex.subscribe()
    });
});

app.delete('/subscriptions/:connID', function(req, res) {
    res.json({
        "result": "Ok",
        "value": trex.unsubscribe(parseInt(req.params.connID))
    });
});

app.get('/events/:connID', function(req, res) {
    res.json({
        "result": "Ok",
        "value": trex.getNotification(parseInt(req.params.connID))
    });
});

app.post('/events/:connID', function(req, res) {
    res.json({
        "result": "Ok",
        "value": trex.publish(req.params.connID, req.body.type, req.body.area)
    });
});

// Special publish service for anonymous publishers
app.post('/events', function(req, res) {
    res.json({
        "result": "Ok",
        "value": trex.unknown_publish(JSON.stringify(req.body))
    });
});

app.get('/status', function(req, res) {
    trex.status();
    res.json({
        "result": "Ok"
    });
});

app.listen(8888, function() {
    console.log('Listening on port 8888');
})
