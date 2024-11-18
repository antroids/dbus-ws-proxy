
    const schema = {
  "asyncapi": "3.0.0",
  "info": {
    "title": "DBus WebSocket proxy API",
    "version": "0.0.1",
    "description": "WebSocket API to interact with DBus IPC protocol",
    "license": {
      "name": "MIT",
      "url": "https://opensource.org/license/mit"
    },
    "externalDocs": {
      "description": "DBus Specification",
      "url": "https://dbus.freedesktop.org/doc/dbus-specification.html"
    }
  },
  "defaultContentType": "application/json",
  "servers": {
    "default": {
      "host": "{host}:{port}",
      "protocol": "ws",
      "variables": {
        "host": {
          "default": "127.0.0.1",
          "description": "WebSocket endpoint host"
        },
        "port": {
          "default": "2024",
          "description": "WebSocket endpoint port"
        }
      }
    }
  },
  "channels": {
    "webSocketV1": {
      "title": "WebSocket endpoint used to communicate with DBus server.",
      "address": "/ws/v1",
      "servers": [
        "$ref:$.servers.default"
      ],
      "bindings": {
        "ws": {
          "method": "GET",
          "query": {
            "type": "object",
            "properties": {
              "connection": {
                "title": "DBus connection target",
                "description": "D-Bus is designed for two specific use cases:| A \"system bus\" for notifications from the system| to user sessions, and to allow the system to request| input from user sessions.| A \"session bus\" used to implement desktop environments| such as GNOME and KDE.",
                "type": "string",
                "enum": [
                  "session",
                  "system"
                ]
              }
            }
          }
        }
      },
      "messages": {
        "callMethod": {
          "title": "DBus method call request",
          "name": "callMethod",
          "payload": {
            "type": "object",
            "required": [
              "CallMethod"
            ],
            "properties": {
              "CallMethod": {
                "type": "object",
                "required": [
                  "path",
                  "methodName"
                ],
                "properties": {
                  "requestId": {
                    "title": "Request id",
                    "description": "Unsigned 64-bytes number that allows to trace requests, and match them with results.",
                    "type": "number",
                    "minimum": 0,
                    "x-parser-schema-id": "requestId"
                  },
                  "destination": {
                    "title": "DBus bus name",
                    "description": "DBus bus name. DBus connections have one or more bus names associated with them.",
                    "externalDocs": {
                      "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus"
                    },
                    "type": "string",
                    "pattern": "^:?[A-Za-z0-9_\\-]+(\\.[A-Za-z0-9_\\-]+)+$",
                    "maxLength": 255,
                    "x-parser-schema-id": "busName"
                  },
                  "path": {
                    "title": "Object Path",
                    "externalDocs": {
                      "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path"
                    },
                    "type": "string",
                    "pattern": "^/([A-Za-z0-9_]+(/[A-Za-z0-9_]+)*)?$",
                    "x-parser-schema-id": "objectPathValue"
                  },
                  "interface": {
                    "title": "DBus interface name",
                    "description": "DBus interface name.",
                    "externalDocs": {
                      "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface"
                    },
                    "type": "string",
                    "pattern": "^[A-Za-z_]+[A-Za-z0-9_]*(\\.[A-Za-z_]+[A-Za-z0-9_]*)+$",
                    "maxLength": 255,
                    "x-parser-schema-id": "interfaceName"
                  },
                  "methodName": {
                    "title": "DBus member name",
                    "description": "DBus member name.",
                    "externalDocs": {
                      "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member"
                    },
                    "type": "string",
                    "pattern": "^[A-Za-z_]+[A-Za-z0-9_]*$",
                    "maxLength": 255,
                    "x-parser-schema-id": "memberName"
                  },
                  "args": {
                    "title": "Method arguments",
                    "type": "array",
                    "items": {
                      "title": "DBus value",
                      "externalDocs": {
                        "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#type-system"
                      },
                      "type": "object",
                      "discriminator": "type",
                      "required": [
                        "type"
                      ],
                      "properties": {
                        "type": {
                          "type": "string",
                          "enum": [
                            "u8",
                            "bool",
                            "i16",
                            "u16",
                            "i32",
                            "u32",
                            "i64",
                            "u64",
                            "f64",
                            "string",
                            "signature",
                            "objectPath",
                            "fd",
                            "variant",
                            "array",
                            "dict",
                            "struct"
                          ],
                          "x-parser-schema-id": "<anonymous-schema-4>"
                        }
                      },
                      "x-parser-schema-id": "value"
                    },
                    "x-parser-schema-id": "<anonymous-schema-3>"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-2>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-1>"
          },
          "examples": [
            {
              "name": "KDENotification",
              "summary": "Show KDE Notification popup",
              "payload": {
                "CallMethod": {
                  "requestId": 123,
                  "destination": "org.freedesktop.Notifications",
                  "path": "/org/freedesktop/Notifications",
                  "interface": "org.freedesktop.Notifications",
                  "methodName": "Notify",
                  "args": [
                    {
                      "type": "string",
                      "value": "test-app"
                    },
                    {
                      "type": "u32",
                      "value": 0
                    },
                    {
                      "type": "string",
                      "value": "dialog-information"
                    },
                    {
                      "type": "string",
                      "value": "A summary"
                    },
                    {
                      "type": "string",
                      "value": "Some body"
                    },
                    {
                      "type": "array",
                      "valueType": "string"
                    },
                    {
                      "type": "dict",
                      "keyType": "string",
                      "valueType": "variant"
                    },
                    {
                      "type": "i32",
                      "value": 5000
                    }
                  ]
                }
              }
            }
          ],
          "x-parser-unique-object-id": "callMethod"
        },
        "subscribeSignal": {
          "title": "DBus signal subscription request",
          "name": "subscribeSignal",
          "payload": {
            "type": "object",
            "required": [
              "SubscribeSignal"
            ],
            "properties": {
              "SubscribeSignal": {
                "allOf": [
                  {
                    "type": "object",
                    "x-parser-schema-id": "<anonymous-schema-7>"
                  },
                  {
                    "properties": {
                      "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId"
                    },
                    "x-parser-schema-id": "<anonymous-schema-8>"
                  },
                  {
                    "title": "Signal key",
                    "description": "An unique indentifyer for DBus signal",
                    "type": "object",
                    "required": [
                      "destination",
                      "path",
                      "interface",
                      "methodName"
                    ],
                    "properties": {
                      "destination": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.destination",
                      "path": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.path",
                      "interface": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.interface",
                      "methodName": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.methodName",
                      "args": {
                        "type": "array",
                        "title": "Signal arguments filter",
                        "description": "Signals can be filtered by arguments. Only string arguments are supported.",
                        "items": {
                          "type": "array",
                          "prefixItems": [
                            {
                              "type": "integer",
                              "title": "Argument index",
                              "format": "int32",
                              "minimum": 0,
                              "maximum": 255
                            },
                            {
                              "type": "string",
                              "title": "Argument value"
                            }
                          ],
                          "x-parser-schema-id": "<anonymous-schema-10>"
                        },
                        "x-parser-schema-id": "<anonymous-schema-9>"
                      }
                    },
                    "x-parser-schema-id": "signalKey"
                  }
                ],
                "x-parser-schema-id": "<anonymous-schema-6>"
              }
            },
            "examples": [
              {
                "name": "Subscribe for signal",
                "summary": "Subscribe for layout change signal",
                "payload": {
                  "SubscribeSignal": {
                    "requestId": 345,
                    "destination": "org.kde.keyboard",
                    "path": "/Layouts",
                    "interface": "org.kde.KeyboardLayouts",
                    "name": "layoutChanged"
                  }
                }
              }
            ],
            "x-parser-schema-id": "<anonymous-schema-5>"
          },
          "x-parser-unique-object-id": "subscribeSignal"
        },
        "unsubscribeSignal": {
          "title": "DBus signal unsubscription request",
          "name": "unsubscribeSignal",
          "payload": {
            "type": "object",
            "required": [
              "UnsubscribeSignal"
            ],
            "properties": {
              "UnsubscribeSignal": {
                "allOf": [
                  {
                    "type": "object",
                    "x-parser-schema-id": "<anonymous-schema-13>"
                  },
                  {
                    "properties": {
                      "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId"
                    },
                    "x-parser-schema-id": "<anonymous-schema-14>"
                  },
                  "$ref:$.channels.webSocketV1.messages.subscribeSignal.payload.properties.SubscribeSignal.allOf[2]"
                ],
                "x-parser-schema-id": "<anonymous-schema-12>"
              }
            },
            "examples": [
              {
                "name": "Unsubscribe from signal",
                "summary": "Unsubscribe from layout change signal",
                "payload": {
                  "UnsubscribeSignal": {
                    "requestId": 345,
                    "destination": "org.kde.keyboard",
                    "path": "/Layouts",
                    "interface": "org.kde.KeyboardLayouts",
                    "name": "layoutChanged"
                  }
                }
              }
            ],
            "x-parser-schema-id": "<anonymous-schema-11>"
          },
          "x-parser-unique-object-id": "unsubscribeSignal"
        },
        "methodReturn": {
          "title": "DBus method call result",
          "description": "DBus method call result will be sent if method was executed successfully",
          "name": "methodReturn",
          "payload": {
            "type": "object",
            "required": [
              "MethodReturn"
            ],
            "properties": {
              "MethodReturn": {
                "type": "object",
                "properties": {
                  "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId",
                  "args": {
                    "title": "Method result",
                    "type": "array",
                    "items": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
                    "x-parser-schema-id": "<anonymous-schema-17>"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-16>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-15>"
          },
          "x-parser-unique-object-id": "methodReturn"
        },
        "methodError": {
          "title": "DBus method call error",
          "description": "DBus method call error will be sent if method was executed with an error",
          "name": "methodError",
          "payload": {
            "type": "object",
            "required": [
              "MethodError"
            ],
            "properties": {
              "MethodError": {
                "type": "object",
                "properties": {
                  "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId",
                  "args": {
                    "title": "Method error parameters",
                    "type": "array",
                    "items": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
                    "x-parser-schema-id": "<anonymous-schema-20>"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-19>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-18>"
          },
          "x-parser-unique-object-id": "methodError"
        },
        "signal": {
          "title": "DBus signal",
          "description": "DBus signal received.",
          "name": "signal",
          "payload": {
            "type": "object",
            "required": [
              "Signal"
            ],
            "properties": {
              "Signal": {
                "type": "object",
                "properties": {
                  "key": "$ref:$.channels.webSocketV1.messages.subscribeSignal.payload.properties.SubscribeSignal.allOf[2]",
                  "args": {
                    "title": "Signal arguments values",
                    "type": "array",
                    "items": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
                    "x-parser-schema-id": "<anonymous-schema-23>"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-22>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-21>"
          },
          "x-parser-unique-object-id": "signal"
        },
        "success": {
          "title": "Success",
          "description": "A general success message can be received as result of operations without result payloads",
          "name": "success",
          "payload": {
            "type": "object",
            "required": [
              "Success"
            ],
            "properties": {
              "Success": {
                "type": "object",
                "properties": {
                  "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId"
                },
                "x-parser-schema-id": "<anonymous-schema-25>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-24>"
          },
          "x-parser-unique-object-id": "success"
        },
        "error": {
          "title": "Application error",
          "name": "error",
          "payload": {
            "type": "object",
            "required": [
              "Error"
            ],
            "properties": {
              "MethodError": {
                "type": "object",
                "required": [
                  "errorType",
                  "message"
                ],
                "properties": {
                  "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId",
                  "errorType": {
                    "title": "Error type",
                    "description": "Application error type",
                    "type": "string",
                    "enum": [
                      "DBusError",
                      "ServerError",
                      "UnsupportedFormat",
                      "JsonError",
                      "DBusFormatError",
                      "DBusValueError"
                    ],
                    "x-parser-schema-id": "<anonymous-schema-28>"
                  },
                  "message": {
                    "title": "Error message",
                    "description": "Application error message",
                    "type": "string",
                    "x-parser-schema-id": "<anonymous-schema-29>"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-27>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-26>"
          },
          "x-parser-unique-object-id": "error"
        }
      },
      "x-parser-unique-object-id": "webSocketV1"
    }
  },
  "operations": {
    "callMethod": {
      "title": "Call method",
      "summary": "Action to call DBus method.",
      "externalDocs": {
        "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types-method"
      },
      "channel": "$ref:$.channels.webSocketV1",
      "action": "send",
      "messages": [
        "$ref:$.channels.webSocketV1.messages.callMethod"
      ],
      "reply": {
        "channel": "$ref:$.channels.webSocketV1",
        "messages": [
          "$ref:$.channels.webSocketV1.messages.methodReturn",
          "$ref:$.channels.webSocketV1.messages.methodError",
          "$ref:$.channels.webSocketV1.messages.error"
        ]
      },
      "x-parser-unique-object-id": "callMethod"
    },
    "subscribeSignal": {
      "title": "Subscribe signal",
      "summary": "Subscribe as receiver for a DBus signals. Subscriptions will be stored for existing connection.",
      "channel": "$ref:$.channels.webSocketV1",
      "action": "send",
      "messages": [
        "$ref:$.channels.webSocketV1.messages.subscribeSignal"
      ],
      "reply": {
        "channel": "$ref:$.channels.webSocketV1",
        "messages": [
          "$ref:$.channels.webSocketV1.messages.success",
          "$ref:$.channels.webSocketV1.messages.error"
        ]
      },
      "x-parser-unique-object-id": "subscribeSignal"
    },
    "unsubscribeSignal": {
      "title": "Unsubscribe signal",
      "summary": "Unsubscribe as receiver for a DBus signals.",
      "channel": "$ref:$.channels.webSocketV1",
      "action": "send",
      "messages": [
        "$ref:$.channels.webSocketV1.messages.unsubscribeSignal"
      ],
      "reply": {
        "channel": "$ref:$.channels.webSocketV1",
        "messages": [
          "$ref:$.channels.webSocketV1.messages.success",
          "$ref:$.channels.webSocketV1.messages.error"
        ]
      },
      "x-parser-unique-object-id": "unsubscribeSignal"
    },
    "signal": {
      "title": "Signal",
      "summary": "DBus signal received",
      "channel": "$ref:$.channels.webSocketV1",
      "action": "receive",
      "messages": [
        "$ref:$.channels.webSocketV1.messages.signal"
      ],
      "x-parser-unique-object-id": "signal"
    },
    "error": {
      "title": "Application error",
      "summary": "Application error details",
      "channel": "$ref:$.channels.webSocketV1",
      "action": "receive",
      "messages": [
        "$ref:$.channels.webSocketV1.messages.error"
      ],
      "x-parser-unique-object-id": "error"
    }
  },
  "components": {
    "serverVariables": {
      "host": "$ref:$.servers.default.variables.host",
      "port": "$ref:$.servers.default.variables.port"
    },
    "messages": {
      "callMethod": "$ref:$.channels.webSocketV1.messages.callMethod",
      "subscribeSignal": "$ref:$.channels.webSocketV1.messages.subscribeSignal",
      "unsubscribeSignal": "$ref:$.channels.webSocketV1.messages.unsubscribeSignal",
      "methodReturn": "$ref:$.channels.webSocketV1.messages.methodReturn",
      "methodError": "$ref:$.channels.webSocketV1.messages.methodError",
      "signal": "$ref:$.channels.webSocketV1.messages.signal",
      "success": "$ref:$.channels.webSocketV1.messages.success",
      "error": "$ref:$.channels.webSocketV1.messages.error"
    },
    "schemas": {
      "requestId": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.requestId",
      "busName": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.destination",
      "interfaceName": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.interface",
      "memberName": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.methodName",
      "signalKey": "$ref:$.channels.webSocketV1.messages.subscribeSignal.payload.properties.SubscribeSignal.allOf[2]",
      "value": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
      "u8": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "8-bit unsigned integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int32",
                "minimum": 0,
                "maximum": 255,
                "x-parser-schema-id": "<anonymous-schema-31>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-30>"
          }
        ],
        "x-parser-schema-id": "u8"
      },
      "bool": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Boolean value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "boolean",
                "x-parser-schema-id": "<anonymous-schema-33>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-32>"
          }
        ],
        "x-parser-schema-id": "bool"
      },
      "i16": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "16-bit signed integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int32",
                "minimum": -32768,
                "maximum": 32767,
                "x-parser-schema-id": "<anonymous-schema-35>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-34>"
          }
        ],
        "x-parser-schema-id": "i16"
      },
      "u16": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "16-bit unsigned integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int32",
                "minimum": 0,
                "maximum": 65535,
                "x-parser-schema-id": "<anonymous-schema-37>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-36>"
          }
        ],
        "x-parser-schema-id": "u16"
      },
      "i32": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "32-bit signed integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int32",
                "x-parser-schema-id": "<anonymous-schema-39>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-38>"
          }
        ],
        "x-parser-schema-id": "i32"
      },
      "u32": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "32-bit unsigned integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int64",
                "minimum": 0,
                "maximum": 4294967295,
                "x-parser-schema-id": "<anonymous-schema-41>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-40>"
          }
        ],
        "x-parser-schema-id": "u32"
      },
      "i64": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "64-bit signed integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int64",
                "x-parser-schema-id": "<anonymous-schema-43>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-42>"
          }
        ],
        "x-parser-schema-id": "i64"
      },
      "u64": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "64-bit signed integer value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "integer",
                "format": "int64",
                "minimum": 0,
                "x-parser-schema-id": "<anonymous-schema-45>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-44>"
          }
        ],
        "x-parser-schema-id": "u64"
      },
      "f64": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "64-bit float value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "number",
                "format": "double",
                "x-parser-schema-id": "<anonymous-schema-47>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-46>"
          }
        ],
        "x-parser-schema-id": "f64"
      },
      "string": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "String value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "string",
                "x-parser-schema-id": "<anonymous-schema-49>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-48>"
          }
        ],
        "x-parser-schema-id": "string"
      },
      "signature": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Signature",
            "externalDocs": {
              "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-signature"
            },
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "string",
                "maxLength": 255,
                "pattern": "^[\\{\\}\\(\\)ybnqiuxtdsogavh].+$",
                "x-parser-schema-id": "<anonymous-schema-51>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-50>"
          }
        ],
        "x-parser-schema-id": "signature"
      },
      "objectPath": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Object Path",
            "externalDocs": {
              "url": "https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path"
            },
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.path"
            },
            "x-parser-schema-id": "<anonymous-schema-52>"
          }
        ],
        "x-parser-schema-id": "objectPath"
      },
      "objectPathValue": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.path",
      "fd": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "File Descriptor",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "number",
                "x-parser-schema-id": "<anonymous-schema-54>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-53>"
          }
        ],
        "x-parser-schema-id": "fd"
      },
      "variant": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Variant value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items"
            },
            "x-parser-schema-id": "<anonymous-schema-55>"
          }
        ],
        "x-parser-schema-id": "variant"
      },
      "array": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Array value",
            "type": "object",
            "oneOf": [
              {
                "required": [
                  "value"
                ],
                "properties": {
                  "value": {
                    "type": "array",
                    "items": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-57>"
              },
              {
                "required": [
                  "valueType"
                ],
                "properties": {
                  "valueType": {
                    "oneOf": [
                      {
                        "type": "string",
                        "enum": [
                          "u8",
                          "bool",
                          "i16",
                          "u16",
                          "i32",
                          "u32",
                          "i64",
                          "u64",
                          "f64",
                          "string",
                          "signature",
                          "objectPath",
                          "fd"
                        ],
                        "x-parser-schema-id": "primitiveValueType"
                      },
                      {
                        "oneOf": [
                          {
                            "type": "string",
                            "const": "variant",
                            "x-parser-schema-id": "variantValueType"
                          },
                          {
                            "type": "object",
                            "properties": {
                              "valueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType"
                            },
                            "x-parser-schema-id": "arrayValueType"
                          },
                          {
                            "type": "object",
                            "properties": {
                              "keyType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[0]",
                              "valueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType"
                            },
                            "x-parser-schema-id": "dictValueType"
                          },
                          {
                            "type": "object",
                            "required": [
                              "fields"
                            ],
                            "properties": {
                              "fields": {
                                "type": "array",
                                "items": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType",
                                "x-parser-schema-id": "<anonymous-schema-61>"
                              }
                            },
                            "x-parser-schema-id": "structValueType"
                          }
                        ],
                        "x-parser-schema-id": "containerValueType"
                      }
                    ],
                    "x-parser-schema-id": "valueType"
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-58>"
              }
            ],
            "x-parser-schema-id": "<anonymous-schema-56>"
          }
        ],
        "x-parser-schema-id": "array"
      },
      "dict": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Dictionary value",
            "type": "object",
            "oneOf": [
              {
                "required": [
                  "value"
                ],
                "properties": {
                  "value": {
                    "type": "object",
                    "patternProperties": {
                      ".": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items"
                    }
                  }
                },
                "x-parser-schema-id": "<anonymous-schema-60>"
              },
              "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[1].oneOf[2]"
            ],
            "x-parser-schema-id": "<anonymous-schema-59>"
          }
        ],
        "x-parser-schema-id": "dict"
      },
      "struct": {
        "allOf": [
          "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
          {
            "title": "Structure value",
            "type": "object",
            "required": [
              "value"
            ],
            "properties": {
              "value": {
                "type": "array",
                "items": "$ref:$.channels.webSocketV1.messages.callMethod.payload.properties.CallMethod.properties.args.items",
                "x-parser-schema-id": "<anonymous-schema-63>"
              }
            },
            "x-parser-schema-id": "<anonymous-schema-62>"
          }
        ],
        "x-parser-schema-id": "struct"
      },
      "valueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType",
      "primitiveValueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[0]",
      "containerValueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[1]",
      "variantValueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[1].oneOf[0]",
      "arrayValueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[1].oneOf[1]",
      "dictValueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[1].oneOf[2]",
      "structValueType": "$ref:$.components.schemas.array.allOf[1].oneOf[1].properties.valueType.oneOf[1].oneOf[3]"
    }
  },
  "x-parser-spec-parsed": true,
  "x-parser-api-version": 3,
  "x-parser-circular": true,
  "x-parser-spec-stringified": true
};
    const config = {"show":{"sidebar":true},"sidebar":{"showOperations":"byDefault"}};
    const appRoot = document.getElementById('root');
    AsyncApiStandalone.render(
        { schema, config, }, appRoot
    );
  