var express = require('express')
var bodyParser = require('body-parser');
var trex = require('./trex');
var colors = require('colors/safe');

var app = express()

process.argv.forEach(function (val, index, array) {
  if (index == 2 && val == "testing"){
    trex.init_examples();//Initialize smoke, temp and fire events; plus the fire rule
  }
});

app.use(bodyParser.json());

// Middleware function to debug
app.use(function(req, res, next) {
    console.log(colors.green('|--------- Request:', req.method, req.url, '---------|'));
    console.log('Body:', req.body);
    next()
})



app.get('/connections', function(req, res) {
    res.json(JSON.parse(trex.get_connection()));
});

app.post('/subscriptions/:connID/:eventType', function(req, res) {
    res.json(JSON.parse(trex.subscribe(req.params.connID, parseInt(req.params.eventType))));
});

app.delete('/subscriptions/:connID/:subsID', function(req, res) {
    res.json(JSON.parse(trex.unsubscribe(req.params.connID, parseInt(req.params.subsID))));
});

app.get('/events/:connID', function(req, res) {
    res.json(trex.get_notification(req.params.connID));
});

app.post('/events/:connID', function(req, res) {
    res.json(JSON.parse(trex.publish(req.params.connID, req.body.type, req.body.area)));
});

// Special publish service for anonymous publishers
app.post('/events', function(req, res) {
    res.json(JSON.parse(trex.unknown_publish(JSON.stringify(req.body))));
});

app.post('/declare-event', function(req, res) {
    res.json(JSON.parse(trex.declare_event(JSON.stringify(req.body))));
});

app.post('/define-rule', function(req, res) {
    res.json(JSON.parse(trex.define_rule(JSON.stringify(req.body))));
});



// Extra functionality to get the status of TRex
app.get('/status', function(req, res) {
    res.json(JSON.parse(trex.status()));
});

app.listen(8888, function() {
    console.log('Listening on port 8888');
})
