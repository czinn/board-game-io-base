"use strict";
exports.__esModule = true;
exports.BoardGameClient = void 0;
function get_token(room) {
    return window.localStorage.getItem("reconnect_token:" + room);
}
function set_token(room, token) {
    window.localStorage.setItem("reconnect_token:" + room, token);
}
var Writeable = /** @class */ (function () {
    function Writeable(value, on_set) {
        this.value = value;
        this.subscribers = [];
        this.on_set = on_set || null;
    }
    Writeable.prototype.subscribe = function (subscription) {
        var _this = this;
        subscription(this.value);
        this.subscribers.push(subscription);
        return function () { return _this.subscribers.splice(_this.subscribers.indexOf(subscription), 1); };
    };
    Writeable.prototype.value_internal = function () {
        return this.value;
    };
    Writeable.prototype.set_internal = function (value) {
        this.value = value;
        for (var _i = 0, _a = this.subscribers; _i < _a.length; _i++) {
            var subscription = _a[_i];
            subscription(value);
        }
    };
    Writeable.prototype.set = function (value) {
        if (this.on_set === null || this.on_set(value)) {
            this.set_internal(value);
        }
    };
    return Writeable;
}());
var Readable = /** @class */ (function () {
    function Readable(value) {
        this.writeable = new Writeable(value);
    }
    Readable.prototype.subscribe = function (subscription) {
        return this.writeable.subscribe(subscription);
    };
    Readable.prototype.set_internal = function (value) {
        this.writeable.set_internal(value);
    };
    Readable.prototype.value_internal = function () {
        return this.writeable.value_internal();
    };
    return Readable;
}());
var BoardGameClient = /** @class */ (function () {
    function BoardGameClient(addr) {
        var _this = this;
        this.ws = new WebSocket(addr);
        this.ws.onmessage = function (event) { return _this.handle_server_message(event); };
        this.users = new Readable([]);
        this.config = new Writeable(null, function (config) { return _this.handle_config_update(config); });
        this.view = new Readable(null);
    }
    BoardGameClient.prototype.handle_config_update = function (config) {
        for (var _i = 0, _a = this.users.value_internal(); _i < _a.length; _i++) {
            var user = _a[_i];
            if (user.id === this.user_id) {
                if (user.leader) {
                    this.update_config(config);
                    return true;
                }
                else {
                    return false;
                }
            }
        }
        return false;
    };
    BoardGameClient.prototype.handle_server_message = function (event) {
        var data = JSON.parse(event.data);
        if (data.type === "error") {
            console.log("Error: " + data.message);
        }
        else if (data.type === "join_response") {
            this.room_id = data.room_id;
            window.history.pushState("", "", "/" + data.room_id);
            set_token(data.room_id, data.token);
            this.user_id = data.user_id;
            this.username = data.username;
        }
        else if (data.type === "user_info") {
            this.users.set_internal(data.users);
        }
        else if (data.type === "room_info") {
            this.config.set_internal(data.config);
        }
        else if (data.type === "game_info") {
            this.view.set_internal(data.view);
        }
        else if (data.type === "invalid_action") {
            console.log("Invalid action: " + data.message);
        }
    };
    BoardGameClient.prototype.send_message = function (data) {
        this.ws.send(JSON.stringify(data));
    };
    BoardGameClient.prototype.join_room = function (username, room) {
        this.send_message({ type: "join_room", username: username, room: room });
    };
    BoardGameClient.prototype.rejoin_room = function (room) {
        var token = get_token(room);
        if (token === null) {
            return false;
        }
        this.send_message({ type: "rejoin_room", token: token, room: room });
        return true;
    };
    BoardGameClient.prototype.update_config = function (config) {
        this.send_message({ type: "update_config", config: config });
    };
    BoardGameClient.prototype.start_game = function (player_mapping) {
        this.send_message({ type: "start_game", player_mapping: player_mapping || null });
    };
    BoardGameClient.prototype.do_action = function (action) {
        this.send_message({ type: "do_action", action: action });
    };
    return BoardGameClient;
}());
exports.BoardGameClient = BoardGameClient;
