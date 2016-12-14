var trex_engine = require('../native');

module.exports = {
    get_connection: function() {
        return trex_engine.get_connection();
    },
    subscribe: function() {
        return trex_engine.subscribe();
    },
    unsubscribe: function(connID) {
        return trex_engine.unsubscribe(connID);
    },
    publish: function(connID, type, area) {
        return trex_engine.publish(connID, type, area);
    },
    unknown_publish: function(str_event) {
        return trex_engine.unknown_publish(str_event);
    },
    get_notification: function(connID) {
        return trex_engine.get_notification(connID);
    },
    status: function() {
        return trex_engine.status();
    },
    init_examples: function() {
        return trex_engine.init_examples();
    },
};
