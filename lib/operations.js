// operations.js
// ========
var trex_engine = require('../native');

module.exports = {

  initialize: function () {
    console.log("Initializing...");
    trex_engine.initialize();
  },

  declare: function (req, res) {
    process.stdout.write("Declare: ");
    console.log(req.body);

    var result = trex_engine.declareEvent(
      req.body.event_id,
      req.body.event_name
    );
  },

  define: function (req, res) {
    process.stdout.write("Define: ");
    console.log(req.body);

    var result = trex_engine.defineRule();
  },

  subscribe: function (req, res) {
    process.stdout.write("Subscribe: ");
    console.log(req.body);

    var result = trex_engine.subscribe();
  },

  unsubscribe: function (req, res) {
    process.stdout.write("Unsubscribe: ");
    console.log(req.body);

    var result = trex_engine.unsubscribe();
  },

  publish: function (req, res) {
    process.stdout.write("Publish: ");
    console.log(req.body);

    var result = trex_engine.publish(req.body.event_id);
  },

  get_notification: function (req, res) {
    process.stdout.write("Get Notification: ");
    console.log(req.body);

    var result = trex_engine.get_notification();
    return result;
    // var result = trex_engine.publish();
  },

};
