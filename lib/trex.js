var trex_engine = require('../native');

module.exports = {
    get_connection: function() {
        return trex_engine.get_connection();
    },
    subscribe: function(connID, eventType) {
        return trex_engine.subscribe(connID, eventType);
    },
    unsubscribe: function(connID, subsID) {
        return trex_engine.unsubscribe(connID, subsID);
    },
    get_notification: function(connID) {
        return trex_engine.get_notification(connID);
    },
    publish: function(connID, type, area) {
        return trex_engine.publish(connID, type, area);
    },
    unknown_publish: function(str_event) {
        return trex_engine.unknown_publish(str_event);
    },
    declare_event: function(str_event) {
        return trex_engine.declare_event(str_event);
    },
    define_rule: function(str_rule) {
        return trex_engine.define_rule(str_rule);
    },
    status: function() {
        return trex_engine.status();
    },
    init_examples: function() {
        return trex_engine.init_examples();
    },
    measure_time: function() {
        var hrstart = process.hrtime();
        trex_engine.measure_time();//Dummy function to test
        hrend = process.hrtime(hrstart);
        console.info("Execution time (hr): %ds %dms", hrend[0], hrend[1]/1000000);
        return 0;
    },
};
