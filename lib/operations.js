// operations.js
// ========
var trex_engine = require('../native');

module.exports = {
    getConnection: function() {
        var result = trex_engine.getConnection();
        return result;
    },
    subscribe: function() {
        var result = trex_engine.subscribe();
        return result;
    },
    unsubscribe: function(connID) {
        var result = trex_engine.unsubscribe(connID);
        return result;
    },
    publish: function(connID, type, area) {
        var result = trex_engine.publish(connID, type, area);
        return result;
    },
    // unknown_publish: function (type, area) {
    unknown_publish: function(str_event) {
        // var result = trex_engine.unknown_publish(type, area);
        var result = trex_engine.unknown_publish(str_event);
        return result;
    },
    getNotification: function(connID) {
        var result = trex_engine.getNotification(connID);
        return result;
    },
    status: function() {
        trex_engine.status();
    },
    init_examples: function() {
        trex_engine.init_examples();
    },
    // declare: function (req, res) {
    //   process.stdout.write("Declare: ");
    //   console.log(req.body);
    //
    //   var result = trex_engine.declareEvent(
    //     req.body.event_id,
    //     req.body.event_name
    //   );
    // },
    // define: function (req, res) {
    //   process.stdout.write("Define: ");
    //   console.log(req.body);
    //
    //   var result = trex_engine.defineRule();
    // },
    // get_notification: function (req, res) {
    //   process.stdout.write("Get Notification: ");
    //   console.log(req.body);
    //
    //   var result = trex_engine.get_notification();
    //   return result;
    // },
};
