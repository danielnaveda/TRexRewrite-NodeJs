var trex_engine = require('../native');

module.exports = {
    getConnection: function() {
        var result = trex_engine.get_connection();
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
    unknown_publish: function(str_event) {
        var result = trex_engine.unknown_publish(str_event);
        return result;
    },
    getNotification: function(connID) {
        var result = trex_engine.get_notification(connID);
        return result;
    },
    status: function() {
        trex_engine.status();
    },
    init_examples: function() {
        trex_engine.init_examples();
    },
};
