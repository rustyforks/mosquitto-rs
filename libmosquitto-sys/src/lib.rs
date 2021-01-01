/* automatically generated by rust-bindgen */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]

pub type size_t = ::std::os::raw::c_ulong;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint32_t = ::std::os::raw::c_uint;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mosq_err_t {
    MOSQ_ERR_AUTH_CONTINUE = -4,
    MOSQ_ERR_NO_SUBSCRIBERS = -3,
    MOSQ_ERR_SUB_EXISTS = -2,
    MOSQ_ERR_CONN_PENDING = -1,
    MOSQ_ERR_SUCCESS = 0,
    MOSQ_ERR_NOMEM = 1,
    MOSQ_ERR_PROTOCOL = 2,
    MOSQ_ERR_INVAL = 3,
    MOSQ_ERR_NO_CONN = 4,
    MOSQ_ERR_CONN_REFUSED = 5,
    MOSQ_ERR_NOT_FOUND = 6,
    MOSQ_ERR_CONN_LOST = 7,
    MOSQ_ERR_TLS = 8,
    MOSQ_ERR_PAYLOAD_SIZE = 9,
    MOSQ_ERR_NOT_SUPPORTED = 10,
    MOSQ_ERR_AUTH = 11,
    MOSQ_ERR_ACL_DENIED = 12,
    MOSQ_ERR_UNKNOWN = 13,
    MOSQ_ERR_ERRNO = 14,
    MOSQ_ERR_EAI = 15,
    MOSQ_ERR_PROXY = 16,
    MOSQ_ERR_PLUGIN_DEFER = 17,
    MOSQ_ERR_MALFORMED_UTF8 = 18,
    MOSQ_ERR_KEEPALIVE = 19,
    MOSQ_ERR_LOOKUP = 20,
    MOSQ_ERR_MALFORMED_PACKET = 21,
    MOSQ_ERR_DUPLICATE_PROPERTY = 22,
    MOSQ_ERR_TLS_HANDSHAKE = 23,
    MOSQ_ERR_QOS_NOT_SUPPORTED = 24,
    MOSQ_ERR_OVERSIZE_PACKET = 25,
    MOSQ_ERR_OCSP = 26,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mosq_opt_t {
    MOSQ_OPT_PROTOCOL_VERSION = 1,
    MOSQ_OPT_SSL_CTX = 2,
    MOSQ_OPT_SSL_CTX_WITH_DEFAULTS = 3,
    MOSQ_OPT_RECEIVE_MAXIMUM = 4,
    MOSQ_OPT_SEND_MAXIMUM = 5,
    MOSQ_OPT_TLS_KEYFORM = 6,
    MOSQ_OPT_TLS_ENGINE = 7,
    MOSQ_OPT_TLS_ENGINE_KPASS_SHA1 = 8,
    MOSQ_OPT_TLS_OCSP_REQUIRED = 9,
    MOSQ_OPT_TLS_ALPN = 10,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_message {
    pub mid: ::std::os::raw::c_int,
    pub topic: *mut ::std::os::raw::c_char,
    pub payload: *mut ::std::os::raw::c_void,
    pub payloadlen: ::std::os::raw::c_int,
    pub qos: ::std::os::raw::c_int,
    pub retain: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mqtt5__property {
    _unused: [u8; 0],
}
pub type mosquitto_property = mqtt5__property;
extern "C" {
    pub fn mosquitto_lib_version(
        major: *mut ::std::os::raw::c_int,
        minor: *mut ::std::os::raw::c_int,
        revision: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_lib_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_lib_cleanup() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_new(
        id: *const ::std::os::raw::c_char,
        clean_session: bool,
        obj: *mut ::std::os::raw::c_void,
    ) -> *mut mosquitto;
}
extern "C" {
    pub fn mosquitto_destroy(mosq: *mut mosquitto);
}
extern "C" {
    pub fn mosquitto_reinitialise(
        mosq: *mut mosquitto,
        id: *const ::std::os::raw::c_char,
        clean_session: bool,
        obj: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_will_set(
        mosq: *mut mosquitto,
        topic: *const ::std::os::raw::c_char,
        payloadlen: ::std::os::raw::c_int,
        payload: *const ::std::os::raw::c_void,
        qos: ::std::os::raw::c_int,
        retain: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_will_set_v5(
        mosq: *mut mosquitto,
        topic: *const ::std::os::raw::c_char,
        payloadlen: ::std::os::raw::c_int,
        payload: *const ::std::os::raw::c_void,
        qos: ::std::os::raw::c_int,
        retain: bool,
        properties: *mut mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_will_clear(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_username_pw_set(
        mosq: *mut mosquitto,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        keepalive: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect_bind(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        keepalive: ::std::os::raw::c_int,
        bind_address: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect_bind_v5(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        keepalive: ::std::os::raw::c_int,
        bind_address: *const ::std::os::raw::c_char,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect_async(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        keepalive: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect_bind_async(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        keepalive: ::std::os::raw::c_int,
        bind_address: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect_srv(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        keepalive: ::std::os::raw::c_int,
        bind_address: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_reconnect(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_reconnect_async(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_disconnect(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_disconnect_v5(
        mosq: *mut mosquitto,
        reason_code: ::std::os::raw::c_int,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_publish(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        topic: *const ::std::os::raw::c_char,
        payloadlen: ::std::os::raw::c_int,
        payload: *const ::std::os::raw::c_void,
        qos: ::std::os::raw::c_int,
        retain: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_publish_v5(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        topic: *const ::std::os::raw::c_char,
        payloadlen: ::std::os::raw::c_int,
        payload: *const ::std::os::raw::c_void,
        qos: ::std::os::raw::c_int,
        retain: bool,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_subscribe(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        sub: *const ::std::os::raw::c_char,
        qos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_subscribe_v5(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        sub: *const ::std::os::raw::c_char,
        qos: ::std::os::raw::c_int,
        options: ::std::os::raw::c_int,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_subscribe_multiple(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        sub_count: ::std::os::raw::c_int,
        sub: *const *mut ::std::os::raw::c_char,
        qos: ::std::os::raw::c_int,
        options: ::std::os::raw::c_int,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_unsubscribe(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        sub: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_unsubscribe_v5(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        sub: *const ::std::os::raw::c_char,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_unsubscribe_multiple(
        mosq: *mut mosquitto,
        mid: *mut ::std::os::raw::c_int,
        sub_count: ::std::os::raw::c_int,
        sub: *const *mut ::std::os::raw::c_char,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_message_copy(
        dst: *mut mosquitto_message,
        src: *const mosquitto_message,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_message_free(message: *mut *mut mosquitto_message);
}
extern "C" {
    pub fn mosquitto_message_free_contents(message: *mut mosquitto_message);
}
extern "C" {
    pub fn mosquitto_loop_forever(
        mosq: *mut mosquitto,
        timeout: ::std::os::raw::c_int,
        max_packets: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_loop_start(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_loop_stop(mosq: *mut mosquitto, force: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_loop(
        mosq: *mut mosquitto,
        timeout: ::std::os::raw::c_int,
        max_packets: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_loop_read(
        mosq: *mut mosquitto,
        max_packets: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_loop_write(
        mosq: *mut mosquitto,
        max_packets: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_loop_misc(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_socket(mosq: *mut mosquitto) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_want_write(mosq: *mut mosquitto) -> bool;
}
extern "C" {
    pub fn mosquitto_threaded_set(mosq: *mut mosquitto, threaded: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_opts_set(
        mosq: *mut mosquitto,
        option: mosq_opt_t,
        value: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_int_option(
        mosq: *mut mosquitto,
        option: mosq_opt_t,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_void_option(
        mosq: *mut mosquitto,
        option: mosq_opt_t,
        value: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_string_option(
        mosq: *mut mosquitto,
        option: mosq_opt_t,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_reconnect_delay_set(
        mosq: *mut mosquitto,
        reconnect_delay: ::std::os::raw::c_uint,
        reconnect_delay_max: ::std::os::raw::c_uint,
        reconnect_exponential_backoff: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_max_inflight_messages_set(
        mosq: *mut mosquitto,
        max_inflight_messages: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_message_retry_set(mosq: *mut mosquitto, message_retry: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn mosquitto_user_data_set(mosq: *mut mosquitto, obj: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn mosquitto_userdata(mosq: *mut mosquitto) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mosquitto_tls_set(
        mosq: *mut mosquitto,
        cafile: *const ::std::os::raw::c_char,
        capath: *const ::std::os::raw::c_char,
        certfile: *const ::std::os::raw::c_char,
        keyfile: *const ::std::os::raw::c_char,
        pw_callback: ::std::option::Option<
            unsafe extern "C" fn(
                buf: *mut ::std::os::raw::c_char,
                size: ::std::os::raw::c_int,
                rwflag: ::std::os::raw::c_int,
                userdata: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_tls_insecure_set(mosq: *mut mosquitto, value: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_tls_opts_set(
        mosq: *mut mosquitto,
        cert_reqs: ::std::os::raw::c_int,
        tls_version: *const ::std::os::raw::c_char,
        ciphers: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_tls_psk_set(
        mosq: *mut mosquitto,
        psk: *const ::std::os::raw::c_char,
        identity: *const ::std::os::raw::c_char,
        ciphers: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_connect_callback_set(
        mosq: *mut mosquitto,
        on_connect: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_connect_with_flags_callback_set(
        mosq: *mut mosquitto,
        on_connect: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_connect_v5_callback_set(
        mosq: *mut mosquitto,
        on_connect: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
                props: *const mosquitto_property,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_disconnect_callback_set(
        mosq: *mut mosquitto,
        on_disconnect: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_disconnect_v5_callback_set(
        mosq: *mut mosquitto,
        on_disconnect: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: *const mosquitto_property,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_publish_callback_set(
        mosq: *mut mosquitto,
        on_publish: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_publish_v5_callback_set(
        mosq: *mut mosquitto,
        on_publish: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
                arg5: *const mosquitto_property,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_message_callback_set(
        mosq: *mut mosquitto,
        on_message: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: *const mosquitto_message,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_message_v5_callback_set(
        mosq: *mut mosquitto,
        on_message: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: *const mosquitto_message,
                arg4: *const mosquitto_property,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_subscribe_callback_set(
        mosq: *mut mosquitto,
        on_subscribe: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
                arg5: *const ::std::os::raw::c_int,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_subscribe_v5_callback_set(
        mosq: *mut mosquitto,
        on_subscribe: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
                arg5: *const ::std::os::raw::c_int,
                arg6: *const mosquitto_property,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_unsubscribe_callback_set(
        mosq: *mut mosquitto,
        on_unsubscribe: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_unsubscribe_v5_callback_set(
        mosq: *mut mosquitto,
        on_unsubscribe: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: *const mosquitto_property,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_log_callback_set(
        mosq: *mut mosquitto,
        on_log: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
                arg4: *const ::std::os::raw::c_char,
            ),
        >,
    );
}
extern "C" {
    pub fn mosquitto_socks5_set(
        mosq: *mut mosquitto,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_strerror(mosq_errno: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mosquitto_connack_string(
        connack_code: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mosquitto_reason_string(
        reason_code: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mosquitto_string_to_command(
        str: *const ::std::os::raw::c_char,
        cmd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_sub_topic_tokenise(
        subtopic: *const ::std::os::raw::c_char,
        topics: *mut *mut *mut ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_sub_topic_tokens_free(
        topics: *mut *mut *mut ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_topic_matches_sub(
        sub: *const ::std::os::raw::c_char,
        topic: *const ::std::os::raw::c_char,
        result: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_topic_matches_sub2(
        sub: *const ::std::os::raw::c_char,
        sublen: size_t,
        topic: *const ::std::os::raw::c_char,
        topiclen: size_t,
        result: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_pub_topic_check(topic: *const ::std::os::raw::c_char)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_pub_topic_check2(
        topic: *const ::std::os::raw::c_char,
        topiclen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_sub_topic_check(topic: *const ::std::os::raw::c_char)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_sub_topic_check2(
        topic: *const ::std::os::raw::c_char,
        topiclen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_validate_utf8(
        str: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libmosquitto_will {
    pub topic: *mut ::std::os::raw::c_char,
    pub payload: *mut ::std::os::raw::c_void,
    pub payloadlen: ::std::os::raw::c_int,
    pub qos: ::std::os::raw::c_int,
    pub retain: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libmosquitto_tls {
    pub cafile: *mut ::std::os::raw::c_char,
    pub capath: *mut ::std::os::raw::c_char,
    pub certfile: *mut ::std::os::raw::c_char,
    pub keyfile: *mut ::std::os::raw::c_char,
    pub ciphers: *mut ::std::os::raw::c_char,
    pub tls_version: *mut ::std::os::raw::c_char,
    pub pw_callback: ::std::option::Option<
        unsafe extern "C" fn(
            buf: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_int,
            rwflag: ::std::os::raw::c_int,
            userdata: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub cert_reqs: ::std::os::raw::c_int,
}
extern "C" {
    pub fn mosquitto_subscribe_simple(
        messages: *mut *mut mosquitto_message,
        msg_count: ::std::os::raw::c_int,
        want_retained: bool,
        topic: *const ::std::os::raw::c_char,
        qos: ::std::os::raw::c_int,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        client_id: *const ::std::os::raw::c_char,
        keepalive: ::std::os::raw::c_int,
        clean_session: bool,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
        will: *const libmosquitto_will,
        tls: *const libmosquitto_tls,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_subscribe_callback(
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut mosquitto,
                arg2: *mut ::std::os::raw::c_void,
                arg3: *const mosquitto_message,
            ) -> ::std::os::raw::c_int,
        >,
        userdata: *mut ::std::os::raw::c_void,
        topic: *const ::std::os::raw::c_char,
        qos: ::std::os::raw::c_int,
        host: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        client_id: *const ::std::os::raw::c_char,
        keepalive: ::std::os::raw::c_int,
        clean_session: bool,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
        will: *const libmosquitto_will,
        tls: *const libmosquitto_tls,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_byte(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_int16(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: u16,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_int32(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_varint(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_binary(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        len: u16,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_string(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_add_string_pair(
        proplist: *mut *mut mosquitto_property,
        identifier: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_read_byte(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *mut u8,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_read_int16(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *mut u16,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_read_int32(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *mut u32,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_read_varint(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *mut u32,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_read_binary(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *mut *mut ::std::os::raw::c_void,
        len: *mut u16,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_read_string(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        value: *mut *mut ::std::os::raw::c_char,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_read_string_pair(
        proplist: *const mosquitto_property,
        identifier: ::std::os::raw::c_int,
        name: *mut *mut ::std::os::raw::c_char,
        value: *mut *mut ::std::os::raw::c_char,
        skip_first: bool,
    ) -> *const mosquitto_property;
}
extern "C" {
    pub fn mosquitto_property_free_all(properties: *mut *mut mosquitto_property);
}
extern "C" {
    pub fn mosquitto_property_copy_all(
        dest: *mut *mut mosquitto_property,
        src: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_check_command(
        command: ::std::os::raw::c_int,
        identifier: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_property_check_all(
        command: ::std::os::raw::c_int,
        properties: *const mosquitto_property,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_string_to_property_info(
        propname: *const ::std::os::raw::c_char,
        identifier: *mut ::std::os::raw::c_int,
        type_: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}