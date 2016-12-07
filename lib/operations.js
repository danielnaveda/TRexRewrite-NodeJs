// operations.js
// ========
var trex_engine = require('../native');

module.exports = {

  getConnection: function () {
    // process.stdout.write("Get Connection: ");
    // console.log("GetConnection");

    var result = trex_engine.getConnection();
    return result;
  },
  // subscribe: function (connID) {
  subscribe: function () {
    // process.stdout.write("Subscribe: ");
    // console.log("Subscribe");

    var result = trex_engine.subscribe();
    return result;
  },
  unsubscribe: function (connID) {
    // process.stdout.write("Subscribe: ");
    // console.log("Subscribe");

    var result = trex_engine.unsubscribe(connID);
    return result;
  },
  publish: function (connID, type, area) {
    // process.stdout.write("Publish: ");
    // console.log("Publish");

    var result = trex_engine.publish(connID, type, area);
    return result;
  },
  unknown_publish: function (type, area) {
    // process.stdout.write("Publish: ");
    // console.log("Publish");

    var result = trex_engine.unknown_publish(type, area);
    return result;
  },
  getNotification: function (connID) {
    // process.stdout.write("Get Notification: ");
    // console.log("GetNotification");

    var result = trex_engine.getNotification(connID);
    return result;
  },
  status: function () {
    // process.stdout.write("System's Status: ");
    // console.log("Status");

    trex_engine.status();
  },




  initialize: function () {
    // console.log("Initializing...");
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
/*
  subscribe: function (req, res) {
    process.stdout.write("Subscribe: ");
    console.log(req.body);

    var result = trex_engine.subscribe();
  },
*/
  // unsubscribe: function (req, res) {
  //   process.stdout.write("Unsubscribe: ");
  //   console.log(req.body);
  //
  //   var result = trex_engine.unsubscribe();
  // },
/*
  publish: function (req, res) {
    process.stdout.write("Publish: ");
    console.log(req.body);

    var result = trex_engine.publish(req.body.event_id);
  },
*/
  get_notification: function (req, res) {
    process.stdout.write("Get Notification: ");
    console.log(req.body);

    var result = trex_engine.get_notification();
    return result;
    // var result = trex_engine.publish();
  },

};
