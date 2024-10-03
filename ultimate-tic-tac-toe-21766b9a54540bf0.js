let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_52(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0e1ee50bf3ae07ee(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_61(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h03532963571218b4(arg0, arg1);
}

function __wbg_adapter_68(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he0583b95a42a2829(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_71(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h60c2de38e2d09895(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_74(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbdd5cd6c975671e2(arg0, arg1);
}

function __wbg_adapter_81(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h087d4ee519f28770(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_84(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4d5f66cd1d49a90f(arg0, arg1, addHeapObject(arg2));
}

/**
* @returns {ChannelWorkerFn}
*/
export function WORKERS_CHANNEL_MctsWorker() {
    const ret = wasm.WORKERS_CHANNEL_MctsWorker();
    return ChannelWorkerFn.__wrap(ret);
}

/**
*/
export function init_workers() {
    wasm.init_workers();
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {CallbackWorkerFn} callback_worker
*/
export function register_callback_worker(callback_worker) {
    _assertClass(callback_worker, CallbackWorkerFn);
    wasm.register_callback_worker(callback_worker.__wbg_ptr);
}

/**
* @param {ChannelWorkerFn} channel_worker
*/
export function register_channel_worker(channel_worker) {
    _assertClass(channel_worker, ChannelWorkerFn);
    wasm.register_channel_worker(channel_worker.__wbg_ptr);
}

/**
* @param {FutureWorkerFn} future_worker
*/
export function register_future_worker(future_worker) {
    _assertClass(future_worker, FutureWorkerFn);
    wasm.register_future_worker(future_worker.__wbg_ptr);
}

/**
* @param {StreamWorkerFn} stream_worker
*/
export function register_stream_worker(stream_worker) {
    _assertClass(stream_worker, StreamWorkerFn);
    wasm.register_stream_worker(stream_worker.__wbg_ptr);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
function __wbg_adapter_385(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__hed0d27d19220fb55(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const CallbackWorkerFnFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_callbackworkerfn_free(ptr >>> 0, 1));
/**
*/
export class CallbackWorkerFn {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CallbackWorkerFnFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_callbackworkerfn_free(ptr, 0);
    }
}

const ChannelWorkerFnFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_channelworkerfn_free(ptr >>> 0, 1));
/**
*/
export class ChannelWorkerFn {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChannelWorkerFn.prototype);
        obj.__wbg_ptr = ptr;
        ChannelWorkerFnFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChannelWorkerFnFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_channelworkerfn_free(ptr, 0);
    }
}

const FutureWorkerFnFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_futureworkerfn_free(ptr >>> 0, 1));
/**
*/
export class FutureWorkerFn {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FutureWorkerFnFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_futureworkerfn_free(ptr, 0);
    }
}

const IntoUnderlyingByteSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingbytesource_free(ptr >>> 0, 1));
/**
*/
export class IntoUnderlyingByteSource {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingByteSourceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingbytesource_free(ptr, 0);
    }
    /**
    * @returns {string}
    */
    get type() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.intounderlyingbytesource_type(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
        if (r0 !== 0) { wasm.__wbindgen_free(r0, r1, 1); }
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
/**
* @returns {number}
*/
get autoAllocateChunkSize() {
    const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);
    return ret >>> 0;
}
/**
* @param {ReadableByteStreamController} controller
*/
start(controller) {
    wasm.intounderlyingbytesource_start(this.__wbg_ptr, addHeapObject(controller));
}
/**
* @param {ReadableByteStreamController} controller
* @returns {Promise<any>}
*/
pull(controller) {
    const ret = wasm.intounderlyingbytesource_pull(this.__wbg_ptr, addHeapObject(controller));
    return takeObject(ret);
}
/**
*/
cancel() {
    const ptr = this.__destroy_into_raw();
    wasm.intounderlyingbytesource_cancel(ptr);
}
}

const IntoUnderlyingSinkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsink_free(ptr >>> 0, 1));
/**
*/
export class IntoUnderlyingSink {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingSinkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsink_free(ptr, 0);
    }
    /**
    * @param {any} chunk
    * @returns {Promise<any>}
    */
    write(chunk) {
        const ret = wasm.intounderlyingsink_write(this.__wbg_ptr, addHeapObject(chunk));
        return takeObject(ret);
    }
    /**
    * @returns {Promise<any>}
    */
    close() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_close(ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} reason
    * @returns {Promise<any>}
    */
    abort(reason) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_abort(ptr, addHeapObject(reason));
        return takeObject(ret);
    }
}

const IntoUnderlyingSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsource_free(ptr >>> 0, 1));
/**
*/
export class IntoUnderlyingSource {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingSourceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsource_free(ptr, 0);
    }
    /**
    * @param {ReadableStreamDefaultController} controller
    * @returns {Promise<any>}
    */
    pull(controller) {
        const ret = wasm.intounderlyingsource_pull(this.__wbg_ptr, addHeapObject(controller));
        return takeObject(ret);
    }
    /**
    */
    cancel() {
        const ptr = this.__destroy_into_raw();
        wasm.intounderlyingsource_cancel(ptr);
    }
}

const StreamWorkerFnFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_streamworkerfn_free(ptr >>> 0, 1));
/**
*/
export class StreamWorkerFn {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StreamWorkerFnFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_streamworkerfn_free(ptr, 0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'bigint';
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        const ret = getObject(arg0) === null;
        return ret;
    };
    imports.wbg.__wbg_setonmessage_6cf97f8d03f6e948 = function(arg0, arg1) {
        getObject(arg0).set_onmessage(getObject(arg1));
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
    console.error(v0);
};
imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
    const ret = getObject(arg0) == getObject(arg1);
    return ret;
};
imports.wbg.__wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    return ret;
};
imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};
imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};
imports.wbg.__wbg_getwithrefkey_5e6d9547403deab8 = function(arg0, arg1) {
    const ret = getObject(arg0)[getObject(arg1)];
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_841ac57cff3d672b = function(arg0, arg1, arg2) {
    getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
};
imports.wbg.__wbg_debug_783a3d4910bc24c7 = function(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    console.debug(...v0);
};
imports.wbg.__wbg_error_71d6845bf00a930f = function(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    console.error(...v0);
};
imports.wbg.__wbg_log_1f7f93998ab961f7 = function(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    console.log(...v0);
};
imports.wbg.__wbg_warn_0b90a269a514ae1d = function(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    console.warn(...v0);
};
imports.wbg.__wbg_performance_a1b8bde2ee512264 = function(arg0) {
    const ret = getObject(arg0).performance;
    return addHeapObject(ret);
};
imports.wbg.__wbg_now_abd80e969af37148 = function(arg0) {
    const ret = getObject(arg0).now();
    return ret;
};
imports.wbg.__wbg_crypto_1d1f22824a6a080c = function(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};
imports.wbg.__wbg_process_4a72847cc503995b = function(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
};
imports.wbg.__wbg_versions_f686565e586dd935 = function(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
};
imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
};
imports.wbg.__wbg_require_cca90b1a94a0255b = function() { return handleError(function () {
    const ret = module.require;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};
imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};
imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).randomFillSync(takeObject(arg1));
}, arguments) };
imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };
imports.wbg.__wbindgen_is_falsy = function(arg0) {
    const ret = !getObject(arg0);
    return ret;
};
imports.wbg.__wbg_classList_d725bcb3b32c27b5 = function(arg0) {
    const ret = getObject(arg0).classList;
    return addHeapObject(ret);
};
imports.wbg.__wbg_setinnerHTML_ea7e3c6a3c4790c6 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).innerHTML = v0;
};
imports.wbg.__wbg_getAttribute_e867e037f066c410 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getAttribute(v0);
    var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
};
imports.wbg.__wbg_removeAttribute_c80e298b60689065 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).removeAttribute(v0);
}, arguments) };
imports.wbg.__wbg_setAttribute_d5540a19be09f8dc = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_before_ac3792b457802cbf = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_remove_5b68b70c39041e2a = function(arg0) {
    getObject(arg0).remove();
};
imports.wbg.__wbg_append_e2fc366a9f0be0e9 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).append(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_body_b3bb488e8e54bf4b = function(arg0) {
    const ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_createComment_7a1d9856e50567bb = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createComment(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createDocumentFragment_5d919bd9d2e05b55 = function(arg0) {
    const ret = getObject(arg0).createDocumentFragment();
    return addHeapObject(ret);
};
imports.wbg.__wbg_createElement_5921e9eb06b9ec89 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createTextNode_8bce33cf33bf8f6e = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createTextNode(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_querySelector_e21c39150aa72078 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).querySelector(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_Window_5012736c80a01584 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_document_8554450897a855b9 = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_location_af118da6c50d4c3f = function(arg0) {
    const ret = getObject(arg0).location;
    return addHeapObject(ret);
};
imports.wbg.__wbg_settype_b6ab7b74bd1908a1 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).type = v0;
};
imports.wbg.__wbg_add_e210e3b838bff57f = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).add(v0);
}, arguments) };
imports.wbg.__wbg_remove_0dd2beafdaa4d9ba = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).remove(v0);
}, arguments) };
imports.wbg.__wbg_href_9c2fe204628af7a3 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).href;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_search_20c15d493b8602c5 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).search;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_close_cef2400b120c9c73 = function() { return handleError(function (arg0) {
    getObject(arg0).close();
}, arguments) };
imports.wbg.__wbg_enqueue_6f3d433b5e457aea = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).enqueue(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_channel_215e3e6fcd37e5b7 = function(arg0) {
    const ret = getObject(arg0).channel;
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_ShadowRoot_72d8e783f8e0093c = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof ShadowRoot;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_host_fdfe1258b06fe937 = function(arg0) {
    const ret = getObject(arg0).host;
    return addHeapObject(ret);
};
imports.wbg.__wbg_name_43d2150405fbcbf2 = function(arg0, arg1) {
    const ret = getObject(arg1).name;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_postMessage_815692a1ede7b5f1 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).postMessage(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_label_955a86ae47f0da4b = function(arg0, arg1) {
    const ret = getObject(arg1).label;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_setonopen_30f99f291ccec312 = function(arg0, arg1) {
    getObject(arg0).onopen = getObject(arg1);
};
imports.wbg.__wbg_setonerror_73235662321ca2db = function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
};
imports.wbg.__wbg_setonmessage_30aaf581e4dec924 = function(arg0, arg1) {
    getObject(arg0).onmessage = getObject(arg1);
};
imports.wbg.__wbg_send_dd2540b03aa636b6 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).send(v0);
}, arguments) };
imports.wbg.__wbg_iceGatheringState_5a2b41be9feefaa8 = function(arg0) {
    const ret = getObject(arg0).iceGatheringState;
    return {"new":0,"gathering":1,"complete":2,}[ret] ?? 3;
};
imports.wbg.__wbg_iceConnectionState_460b5d9ddbfc5d78 = function(arg0) {
    const ret = getObject(arg0).iceConnectionState;
    return {"new":0,"checking":1,"connected":2,"completed":3,"failed":4,"disconnected":5,"closed":6,}[ret] ?? 7;
};
imports.wbg.__wbg_setonnegotiationneeded_30209d5059e3f79e = function(arg0, arg1) {
    getObject(arg0).onnegotiationneeded = getObject(arg1);
};
imports.wbg.__wbg_setonicecandidate_2905405d9a35513c = function(arg0, arg1) {
    getObject(arg0).onicecandidate = getObject(arg1);
};
imports.wbg.__wbg_setoniceconnectionstatechange_e8aeb867c461deed = function(arg0, arg1) {
    getObject(arg0).oniceconnectionstatechange = getObject(arg1);
};
imports.wbg.__wbg_setonicegatheringstatechange_25deb4f71a642ed8 = function(arg0, arg1) {
    getObject(arg0).onicegatheringstatechange = getObject(arg1);
};
imports.wbg.__wbg_setondatachannel_4d7c6e85c4c669d5 = function(arg0, arg1) {
    getObject(arg0).ondatachannel = getObject(arg1);
};
imports.wbg.__wbg_new_cfa0ae3571f203bc = function() { return handleError(function () {
    const ret = new RTCPeerConnection();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_newwithconfiguration_05616c1f670e37f6 = function() { return handleError(function (arg0) {
    const ret = new RTCPeerConnection(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_addIceCandidate_0c9770c7186d1914 = function(arg0, arg1) {
    const ret = getObject(arg0).addIceCandidate(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_createAnswer_4ef4e8b6e24ad286 = function(arg0) {
    const ret = getObject(arg0).createAnswer();
    return addHeapObject(ret);
};
imports.wbg.__wbg_createDataChannel_17815c5304549c23 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createDataChannel(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createOffer_6f315a46446c28dd = function(arg0) {
    const ret = getObject(arg0).createOffer();
    return addHeapObject(ret);
};
imports.wbg.__wbg_setLocalDescription_124f64811e9eee83 = function(arg0, arg1) {
    const ret = getObject(arg0).setLocalDescription(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_setRemoteDescription_1f6796c241214296 = function(arg0, arg1) {
    const ret = getObject(arg0).setRemoteDescription(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_view_2a901bda0727aeb3 = function(arg0) {
    const ret = getObject(arg0).view;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_respond_a799bab31a44f2d7 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).respond(arg1 >>> 0);
}, arguments) };
imports.wbg.__wbg_setonopen_7e770c87269cae90 = function(arg0, arg1) {
    getObject(arg0).onopen = getObject(arg1);
};
imports.wbg.__wbg_setonmessage_b670c12ea34acd8b = function(arg0, arg1) {
    getObject(arg0).onmessage = getObject(arg1);
};
imports.wbg.__wbg_setbinaryType_d164a0be4c212c9c = function(arg0, arg1) {
    getObject(arg0).binaryType = ["blob","arraybuffer",][arg1];
};
imports.wbg.__wbg_new_0bf4a5b0632517ed = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new WebSocket(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_send_82b52e2f9f8946d9 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).send(v0);
}, arguments) };
imports.wbg.__wbg_newwithstrsequenceandoptions_f700d764298e22da = function() { return handleError(function (arg0, arg1) {
    const ret = new Blob(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_append_d510a297e3ba948e = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).append(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_data_5c47a6985fefc490 = function(arg0) {
    const ret = getObject(arg0).data;
    return addHeapObject(ret);
};
imports.wbg.__wbg_seticeservers_6c4f5299fca15f78 = function(arg0, arg1) {
    getObject(arg0).iceServers = getObject(arg1);
};
imports.wbg.__wbg_setcandidate_42593f0af7e3458b = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).candidate = v0;
};
imports.wbg.__wbg_setsdpmlineindex_db9d0124014c93ea = function(arg0, arg1) {
    getObject(arg0).sdpMLineIndex = arg1 === 0xFFFFFF ? undefined : arg1;
};
imports.wbg.__wbg_setsdpmid_e09033e1ad8a1333 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).sdpMid = v0;
};
imports.wbg.__wbg_value_d4a95e7a0d390578 = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_setvalue_688819688274bec0 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).value = v0;
};
imports.wbg.__wbg_candidate_a88e26280807878d = function(arg0, arg1) {
    const ret = getObject(arg1).candidate;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_sdpMid_6d643de4c03e8fa1 = function(arg0, arg1) {
    const ret = getObject(arg1).sdpMid;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_sdpMLineIndex_de158602e155b62e = function(arg0) {
    const ret = getObject(arg0).sdpMLineIndex;
    return isLikeNone(ret) ? 0xFFFFFF : ret;
};
imports.wbg.__wbg_new_798c0475a977f01a = function() { return handleError(function (arg0) {
    const ret = new RTCIceCandidate(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_parentNode_3e06cf96d7693d57 = function(arg0) {
    const ret = getObject(arg0).parentNode;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_childNodes_031aa96d5e3d21b0 = function(arg0) {
    const ret = getObject(arg0).childNodes;
    return addHeapObject(ret);
};
imports.wbg.__wbg_previousSibling_076df2178284ef97 = function(arg0) {
    const ret = getObject(arg0).previousSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_nextSibling_f6396d6fd0b97830 = function(arg0) {
    const ret = getObject(arg0).nextSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_settextContent_cd38ea7d4e0f7260 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
};
imports.wbg.__wbg_appendChild_ac45d1abddf1b89b = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_cloneNode_629a1b180e91c467 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).cloneNode();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_length_4919f4a62b9b1e94 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_byobRequest_b32c77640da946ac = function(arg0) {
    const ret = getObject(arg0).byobRequest;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_close_aca7442e6619206b = function() { return handleError(function (arg0) {
    getObject(arg0).close();
}, arguments) };
imports.wbg.__wbg_setname_34512937225fceae = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).name = v0;
};
imports.wbg.__wbg_settype_4c4ad7b3603563e6 = function(arg0, arg1) {
    getObject(arg0).type = ["classic","module",][arg1];
};
imports.wbg.__wbg_new_95093d1a71aba61d = function() { return handleError(function () {
    const ret = new Range();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_deleteContents_45ba9b733b3b97ea = function() { return handleError(function (arg0) {
    getObject(arg0).deleteContents();
}, arguments) };
imports.wbg.__wbg_setEndBefore_7d55a9db0e0f4c41 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).setEndBefore(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_setStartBefore_a28dcb3c6ece9e07 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).setStartBefore(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_addEventListener_e167f012cbedfa4e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3));
}, arguments) };
imports.wbg.__wbg_addEventListener_14b036ff7cb8747c = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3), getObject(arg4));
}, arguments) };
imports.wbg.__wbg_candidate_b451e00fc96de4e5 = function(arg0) {
    const ret = getObject(arg0).candidate;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_setonmessage_7cee8e224acfa056 = function(arg0, arg1) {
    getObject(arg0).onmessage = getObject(arg1);
};
imports.wbg.__wbg_newwithoptions_1d8b2a34000b563b = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Worker(v0, getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_postMessage_37faac1bc005e5c0 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).postMessage(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_settype_233a2bf71c25ed93 = function(arg0, arg1) {
    getObject(arg0).type = ["offer","pranswer","answer","rollback",][arg1];
};
imports.wbg.__wbg_setsdp_0b3df5fc570c101e = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).sdp = v0;
};
imports.wbg.__wbg_target_b7cb1739bee70928 = function(arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_cancelBubble_0374b329f66f59b5 = function(arg0) {
    const ret = getObject(arg0).cancelBubble;
    return ret;
};
imports.wbg.__wbg_composedPath_d1052062308beae5 = function(arg0) {
    const ret = getObject(arg0).composedPath();
    return addHeapObject(ret);
};
imports.wbg.__wbg_preventDefault_c55d86c27b2dfa6e = function(arg0) {
    getObject(arg0).preventDefault();
};
imports.wbg.__wbg_error_09480e4aadca50ad = function(arg0) {
    console.error(getObject(arg0));
};
imports.wbg.__wbg_log_b103404cc5920657 = function(arg0) {
    console.log(getObject(arg0));
};
imports.wbg.__wbg_warn_2b3adb99ce26c314 = function(arg0) {
    console.warn(getObject(arg0));
};
imports.wbg.__wbg_setdata_27c6828c5a5a5ce4 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).data = v0;
};
imports.wbg.__wbg_newwithbase_ba5e3790a41efd02 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var v1 = getCachedStringFromWasm0(arg2, arg3);
    const ret = new URL(v0, v1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createObjectURL_ca544150f40fb1bf = function() { return handleError(function (arg0, arg1) {
    const ret = URL.createObjectURL(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_queueMicrotask_48421b3cc9052b68 = function(arg0) {
    const ret = getObject(arg0).queueMicrotask;
    return addHeapObject(ret);
};
imports.wbg.__wbg_queueMicrotask_12a30234db4045d3 = function(arg0) {
    queueMicrotask(getObject(arg0));
};
imports.wbg.__wbg_get_3baa728f9d58d3f6 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_ae22078168b726f5 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_new_a220cf903aa02ca2 = function() {
    const ret = new Array();
    return addHeapObject(ret);
};
imports.wbg.__wbg_newnoargs_76313bd6ff35d0f2 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_next_de3e9db4440638b2 = function(arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
};
imports.wbg.__wbg_next_f9cb570345655b9a = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).next();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_done_bfda7aa8f252b39f = function(arg0) {
    const ret = getObject(arg0).done;
    return ret;
};
imports.wbg.__wbg_value_6d39332ab4788d86 = function(arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
};
imports.wbg.__wbg_iterator_888179a48810a9fe = function() {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
};
imports.wbg.__wbg_get_224d16597dbbfd96 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_call_1084a111329e68ce = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_new_525245e2b9901204 = function() {
    const ret = new Object();
    return addHeapObject(ret);
};
imports.wbg.__wbg_self_3093d5d1f7bcb682 = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_3bcfc4d31bc012f8 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_86b222e13bdf32ed = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_e5a3fe56f8be9485 = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_set_673dda6c73d19609 = function(arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
};
imports.wbg.__wbg_isArray_8364a5371e9737d8 = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
};
imports.wbg.__wbg_push_37c89022f34c01ca = function(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_instanceof_ArrayBuffer_61dfc3198373c902 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof ArrayBuffer;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_new_796382978dfd4fb0 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Error(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_89af060b4e1523f2 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_isSafeInteger_7f1ed56200d90674 = function(arg0) {
    const ret = Number.isSafeInteger(getObject(arg0));
    return ret;
};
imports.wbg.__wbg_now_b7a162010a9e75b4 = function() {
    const ret = Date.now();
    return ret;
};
imports.wbg.__wbg_entries_7a0e06255456ebcd = function(arg0) {
    const ret = Object.entries(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_is_009b1ef508712fda = function(arg0, arg1) {
    const ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
};
imports.wbg.__wbg_toString_e17a6671146f47c1 = function(arg0) {
    const ret = getObject(arg0).toString();
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_b85e72ed1bfd57f9 = function(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_385(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return addHeapObject(ret);
    } finally {
        state0.a = state0.b = 0;
    }
};
imports.wbg.__wbg_resolve_570458cb99d56a43 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_95e6edc0f89b73b1 = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_876bb3c633745cc6 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};
imports.wbg.__wbg_buffer_b7b08af79b0b0974 = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9 = function(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_ea1883e1e5e86686 = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_d1e79e2388520f18 = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_length_8339fcf5d8ecd12e = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_instanceof_Uint8Array_247a91427532499e = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Uint8Array;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_newwithlength_ec548f448387c968 = function(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_buffer_0710d1b9dbe2eea6 = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbg_subarray_7c2e3576afe181d1 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_byteLength_850664ef28f3e42f = function(arg0) {
    const ret = getObject(arg0).byteLength;
    return ret;
};
imports.wbg.__wbg_byteOffset_ea14c35fa6de38cc = function(arg0) {
    const ret = getObject(arg0).byteOffset;
    return ret;
};
imports.wbg.__wbg_set_eacc7d73fefaafdf = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments) };
imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
    const v = getObject(arg1);
    const ret = typeof(v) === 'bigint' ? v : undefined;
    getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};
imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper757 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_52);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper758 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_52);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper759 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_52);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper761 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_52);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper763 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_61);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper767 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_52);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper769 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 246, __wbg_adapter_52);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper878 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 309, __wbg_adapter_68);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1308 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 530, __wbg_adapter_71);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1310 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 530, __wbg_adapter_74);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1312 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 530, __wbg_adapter_71);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1314 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 530, __wbg_adapter_71);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1446 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 572, __wbg_adapter_81);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper3222 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 603, __wbg_adapter_84);
    return addHeapObject(ret);
};

return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined' && Object.getPrototypeOf(module) === Object.prototype)
    ({module} = module)
    else
    console.warn('using deprecated parameters for `initSync()`; pass a single object instead')

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined' && Object.getPrototypeOf(module_or_path) === Object.prototype)
    ({module_or_path} = module_or_path)
    else
    console.warn('using deprecated parameters for the initialization function; pass a single object instead')

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('ultimate-tic-tac-toe_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
