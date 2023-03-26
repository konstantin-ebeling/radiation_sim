const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

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
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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

const CLOSURE_DTORS = new FinalizationRegistry(state => {
    wasm.__wbindgen_export_3.get(state.dtor)(state.a, state.b)
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
                wasm.__wbindgen_export_3.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state)
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_36(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h89d1ec4c460bc49f(arg0, arg1);
}

function __wbg_adapter_39(arg0, arg1, arg2) {
    wasm.closure27816_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_54(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb6ae8ec7c09da6da(arg0, arg1);
}

function __wbg_adapter_57(arg0, arg1, arg2) {
    wasm.closure30343_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_60(arg0, arg1, arg2) {
    wasm.closure31218_externref_shim(arg0, arg1, arg2);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export function wasm_main() {
    wasm.wasm_main();
}

let cachedFloat32Memory0 = null;

function getFloat32Memory0() {
    if (cachedFloat32Memory0 === null || cachedFloat32Memory0.byteLength === 0) {
        cachedFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI32FromWasm0(ptr, len) {
    return getInt32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function getArrayU32FromWasm0(ptr, len) {
    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

async function load(module, imports) {
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

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = arg0.original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Response_fb3a4df648c1859b = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Response;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_fetch_e8596d8a939a0853 = function(arg0, arg1, arg2) {
        const ret = arg0.fetch(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_arrayBuffer_cb886e06a9e36e4d = function() { return handleError(function (arg0) {
        const ret = arg0.arrayBuffer();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_537b7341ce90bb31 = function(arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_mark_40e050a77cc39fea = function(arg0, arg1) {
        performance.mark(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_log_c9486ca5d8e2cbe8 = function(arg0, arg1) {
        try {
            console.log(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_log_aba5996d9bde071f = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        try {
            console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), getStringFromWasm0(arg6, arg7));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_width_a40e21a22129b197 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbg_height_98d51321254345a5 = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_addEventListener_615d4590d38da1c9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_document_950215a728589a2d = function(arg0) {
        const ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_querySelector_32b9d7ebb2df951d = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_parentElement_0e8c9afce5cb9d6e = function(arg0) {
        const ret = arg0.parentElement;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getBoundingClientRect_aaa701cbcb448965 = function(arg0) {
        const ret = arg0.getBoundingClientRect();
        return ret;
    };
    imports.wbg.__wbg_width_f0cbf7dcbbe056da = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbg_height_e46975153da440ae = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_requestAnimationFrame_afe426b568f84138 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.requestAnimationFrame(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setTimeout_6609c9aa64f32bfc = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.setTimeout(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_f5f69dab93281ebe = function(arg0) {
        let result;
        try {
            result = arg0 instanceof HTMLCanvasElement;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_body_be46234bb33edd63 = function(arg0) {
        const ret = arg0.body;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_appendChild_b8199dc1655c852d = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.appendChild(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_requestPointerLock_810495dd0fa1efc0 = function(arg0) {
        arg0.requestPointerLock();
    };
    imports.wbg.__wbg_stopPropagation_7647c9985222f9b0 = function(arg0) {
        arg0.stopPropagation();
    };
    imports.wbg.__wbg_cancelBubble_c9a8182589205d54 = function(arg0) {
        const ret = arg0.cancelBubble;
        return ret;
    };
    imports.wbg.__wbg_preventDefault_16b2170b12f56317 = function(arg0) {
        arg0.preventDefault();
    };
    imports.wbg.__wbg_matches_46e979ff3e4d0811 = function(arg0) {
        const ret = arg0.matches;
        return ret;
    };
    imports.wbg.__wbg_new_f9876326328f45ed = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_target_b629c177f9bee3da = function(arg0) {
        const ret = arg0.target;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_is_8f1618fe9a4fd388 = function(arg0, arg1) {
        const ret = Object.is(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbg_buttons_42a7b7de33d8e572 = function(arg0) {
        const ret = arg0.buttons;
        return ret;
    };
    imports.wbg.__wbg_requestFullscreen_4eee04b9090fa98a = function() { return handleError(function (arg0) {
        arg0.requestFullscreen();
    }, arguments) };
    imports.wbg.__wbg_offsetX_413d9f02022e72ad = function(arg0) {
        const ret = arg0.offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_488f80a0a9666028 = function(arg0) {
        const ret = arg0.offsetY;
        return ret;
    };
    imports.wbg.__wbg_movementX_f4d07f6658c1e16f = function(arg0) {
        const ret = arg0.movementX;
        return ret;
    };
    imports.wbg.__wbg_movementY_30276c1f90aec4fa = function(arg0) {
        const ret = arg0.movementY;
        return ret;
    };
    imports.wbg.__wbg_pointerType_df759fa0bd6634ed = function(arg0, arg1) {
        const ret = arg1.pointerType;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_pointerId_d2caae4465ba386f = function(arg0) {
        const ret = arg0.pointerId;
        return ret;
    };
    imports.wbg.__wbg_clientX_35f23f953e04ec0e = function(arg0) {
        const ret = arg0.clientX;
        return ret;
    };
    imports.wbg.__wbg_clientY_8104e462abc0b3ec = function(arg0) {
        const ret = arg0.clientY;
        return ret;
    };
    imports.wbg.__wbg_pressure_352c13794490720b = function(arg0) {
        const ret = arg0.pressure;
        return ret;
    };
    imports.wbg.__wbg_keyCode_b33194be2ceec53b = function(arg0) {
        const ret = arg0.keyCode;
        return ret;
    };
    imports.wbg.__wbg_charCode_504e79c3e550d1bb = function(arg0) {
        const ret = arg0.charCode;
        return ret;
    };
    imports.wbg.__wbg_setPointerCapture_5479dc0d082282b7 = function() { return handleError(function (arg0, arg1) {
        arg0.setPointerCapture(arg1);
    }, arguments) };
    imports.wbg.__wbg_key_f0decac219aa904b = function(arg0, arg1) {
        const ret = arg1.key;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_ctrlKey_993b558f853d64ce = function(arg0) {
        const ret = arg0.ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_dff2a075455ac01b = function(arg0) {
        const ret = arg0.altKey;
        return ret;
    };
    imports.wbg.__wbg_getModifierState_03b72700dbe33ad6 = function(arg0, arg1, arg2) {
        const ret = arg0.getModifierState(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_removeEventListener_343e3ea9fe4c8533 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_error_2d344a50ccf38b3b = typeof console.error == 'function' ? console.error : notDefined('console.error');
    imports.wbg.__wbg_addEventListener_cf5b03cd29763277 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_resume_72fe7cd3e68b861a = function() { return handleError(function (arg0) {
        const ret = arg0.resume();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_close_82409a9d656a7c26 = function() { return handleError(function (arg0) {
        const ret = arg0.close();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_eval_be0434aab6074e1e = function() { return handleError(function (arg0, arg1) {
        const ret = eval(getStringFromWasm0(arg0, arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = arg0;
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbg_randomFillSync_6894564c2c334c42 = function() { return handleError(function (arg0, arg1, arg2) {
        arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_subarray_7526649b91a252a6 = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_805f1c3d65988a5a = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_crypto_e1d53a1d73fb10b8 = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_process_038c26bf42b093f8 = function(arg0) {
        const ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbg_versions_ab37218d2f0b24a8 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbg_node_080f4b19d15bc1fe = function(arg0) {
        const ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbg_require_78a3dcfbdba9cbce = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbg_call_9495de66fdbe016b = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_msCrypto_6e7d3e1f92610cbb = function(arg0) {
        const ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_b56c882b57805732 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_connected_149a005d845e67e2 = function(arg0) {
        const ret = arg0.connected;
        return ret;
    };
    imports.wbg.__wbg_id_291975ef0fb03ce5 = function(arg0, arg1) {
        const ret = arg1.id;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_buttons_e33d9bb4a83e0700 = function(arg0) {
        const ret = arg0.buttons;
        return ret;
    };
    imports.wbg.__wbg_length_e498fbc24f9c1d4f = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_axes_81f7594079c3e88c = function(arg0) {
        const ret = arg0.axes;
        return ret;
    };
    imports.wbg.__wbg_mapping_6ef03dc7a02c4bef = function(arg0) {
        const ret = arg0.mapping;
        return ret;
    };
    imports.wbg.__wbg_get_27fe3dac1c4d0224 = function(arg0, arg1) {
        const ret = arg0[arg1 >>> 0];
        return ret;
    };
    imports.wbg.__wbg_pressed_3b05e38d48c4c7ab = function(arg0) {
        const ret = arg0.pressed;
        return ret;
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        const ret = arg0 === null;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_isSecureContext_c3e5510caacaa0ce = function(arg0) {
        const ret = arg0.isSecureContext;
        return ret;
    };
    imports.wbg.__wbg_navigator_b18e629f7f0b75fa = function(arg0) {
        const ret = arg0.navigator;
        return ret;
    };
    imports.wbg.__wbg_getGamepads_4c461e89e0e20e75 = function() { return handleError(function (arg0) {
        const ret = arg0.getGamepads();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_index_648379aa28ee0be3 = function(arg0) {
        const ret = arg0.index;
        return ret;
    };
    imports.wbg.__wbg_instanceof_DomException_0dd0987418c574eb = function(arg0) {
        let result;
        try {
            result = arg0 instanceof DOMException;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_message_f15effc8b20828e2 = function(arg0, arg1) {
        const ret = arg1.message;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_now_931686b195a14f9d = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_getExtension_f0070583175271d4 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.getExtension(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_getSupportedExtensions_7af8f7bbdd4d7b2c = function(arg0) {
        const ret = arg0.getSupportedExtensions();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getParameter_56d47f9b55e463d4 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.getParameter(arg1 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_texSubImage2D_bb1504dd3641be28 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_75941cc7af95dbe0 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_eec64ab194e54dc5 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_texSubImage3D_e8042ab768cdd214 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
        arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
    }, arguments) };
    imports.wbg.__wbg_texSubImage3D_f563114226a95faf = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
        arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
    }, arguments) };
    imports.wbg.__wbg_texSubImage3D_0a4fbb250617ca13 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
        arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
    }, arguments) };
    imports.wbg.__wbg_framebufferTextureMultiviewOVR_0ae55d2aa52fd2cb = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        arg0.framebufferTextureMultiviewOVR(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5, arg6);
    };
    imports.wbg.__wbg_bindFramebuffer_a5ab0ed0463586cb = function(arg0, arg1, arg2) {
        arg0.bindFramebuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bindFramebuffer_0522db2a250c29f0 = function(arg0, arg1, arg2) {
        arg0.bindFramebuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_createFramebuffer_1f943a32c748753e = function(arg0) {
        const ret = arg0.createFramebuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createFramebuffer_86883935c13ddd59 = function(arg0) {
        const ret = arg0.createFramebuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createQuery_42b609ba267d041d = function(arg0) {
        const ret = arg0.createQuery();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createRenderbuffer_a76dcfda7bdc749a = function(arg0) {
        const ret = arg0.createRenderbuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createRenderbuffer_b392324e044d389a = function(arg0) {
        const ret = arg0.createRenderbuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createSampler_d1255ae3836b1bee = function(arg0) {
        const ret = arg0.createSampler();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createShader_c5fcd8592f47b510 = function(arg0, arg1) {
        const ret = arg0.createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createShader_96339db58713e350 = function(arg0, arg1) {
        const ret = arg0.createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createTexture_81fd93af28301e0e = function(arg0) {
        const ret = arg0.createTexture();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createTexture_c651f9e28d1ce9d2 = function(arg0) {
        const ret = arg0.createTexture();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_deleteShader_fac9fb3cdefdf6ec = function(arg0, arg1) {
        arg0.deleteShader(arg1);
    };
    imports.wbg.__wbg_deleteShader_f48f72524f5ee3ed = function(arg0, arg1) {
        arg0.deleteShader(arg1);
    };
    imports.wbg.__wbg_shaderSource_457e8bc42050401d = function(arg0, arg1, arg2, arg3) {
        arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_shaderSource_57883245cdfb0dca = function(arg0, arg1, arg2, arg3) {
        arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_compileShader_4e9130ccbd4a0238 = function(arg0, arg1) {
        arg0.compileShader(arg1);
    };
    imports.wbg.__wbg_compileShader_af777dd3b15798b3 = function(arg0, arg1) {
        arg0.compileShader(arg1);
    };
    imports.wbg.__wbg_getShaderParameter_685d7d7092c6bae6 = function(arg0, arg1, arg2) {
        const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getShaderParameter_f9240892c9e7a0a3 = function(arg0, arg1, arg2) {
        const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_createProgram_28db0ff3cee5f71a = function(arg0) {
        const ret = arg0.createProgram();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createProgram_7d25c1dd3bb0ce39 = function(arg0) {
        const ret = arg0.createProgram();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_deleteProgram_dd5f0e2bc555e270 = function(arg0, arg1) {
        arg0.deleteProgram(arg1);
    };
    imports.wbg.__wbg_deleteProgram_9c8fa1ef341cb01d = function(arg0, arg1) {
        arg0.deleteProgram(arg1);
    };
    imports.wbg.__wbg_attachShader_14fb12e2ae589dc3 = function(arg0, arg1, arg2) {
        arg0.attachShader(arg1, arg2);
    };
    imports.wbg.__wbg_attachShader_3038234860d2d59d = function(arg0, arg1, arg2) {
        arg0.attachShader(arg1, arg2);
    };
    imports.wbg.__wbg_linkProgram_ca9df3fba2fd4125 = function(arg0, arg1) {
        arg0.linkProgram(arg1);
    };
    imports.wbg.__wbg_linkProgram_2d5cc584654696b8 = function(arg0, arg1) {
        arg0.linkProgram(arg1);
    };
    imports.wbg.__wbg_getProgramParameter_af1cfcccbbc80f71 = function(arg0, arg1, arg2) {
        const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getProgramParameter_5b1a40917aa850f8 = function(arg0, arg1, arg2) {
        const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getActiveUniform_e49dcda694ae15ab = function(arg0, arg1, arg2) {
        const ret = arg0.getActiveUniform(arg1, arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_size_5ce324b99223d189 = function(arg0) {
        const ret = arg0.size;
        return ret;
    };
    imports.wbg.__wbg_type_979610383a4b7c57 = function(arg0) {
        const ret = arg0.type;
        return ret;
    };
    imports.wbg.__wbg_name_1e6651aff4fe7a88 = function(arg0, arg1) {
        const ret = arg1.name;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getActiveUniform_9c4ac7c1ccf5f894 = function(arg0, arg1, arg2) {
        const ret = arg0.getActiveUniform(arg1, arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_useProgram_2f4094faf45ecba1 = function(arg0, arg1) {
        arg0.useProgram(arg1);
    };
    imports.wbg.__wbg_useProgram_6c9019d05fb8d280 = function(arg0, arg1) {
        arg0.useProgram(arg1);
    };
    imports.wbg.__wbg_createBuffer_8c64250e5283611c = function(arg0) {
        const ret = arg0.createBuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createBuffer_5ed0554ab35780b5 = function(arg0) {
        const ret = arg0.createBuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_bindBuffer_9cb064991696b79f = function(arg0, arg1, arg2) {
        arg0.bindBuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bindBuffer_b7c382dcd70e33f6 = function(arg0, arg1, arg2) {
        arg0.bindBuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bindBufferRange_f2c529259df5358e = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.bindBufferRange(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_bindRenderbuffer_1e4928d9bf839c02 = function(arg0, arg1, arg2) {
        arg0.bindRenderbuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bindRenderbuffer_2d67c879cdbe5ea9 = function(arg0, arg1, arg2) {
        arg0.bindRenderbuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_blitFramebuffer_86eee8a5763ded5e = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        arg0.blitFramebuffer(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0);
    };
    imports.wbg.__wbg_createVertexArray_de7292bbd7ea02dd = function(arg0) {
        const ret = arg0.createVertexArray();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createVertexArrayOES_02cfe655604046eb = function(arg0) {
        const ret = arg0.createVertexArrayOES();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_deleteVertexArray_dc4f1b2e5ac93f24 = function(arg0, arg1) {
        arg0.deleteVertexArray(arg1);
    };
    imports.wbg.__wbg_deleteVertexArrayOES_ba22911f739464a7 = function(arg0, arg1) {
        arg0.deleteVertexArrayOES(arg1);
    };
    imports.wbg.__wbg_bindVertexArray_8b71290041cb6746 = function(arg0, arg1) {
        arg0.bindVertexArray(arg1);
    };
    imports.wbg.__wbg_bindVertexArrayOES_688eba003a98a0bb = function(arg0, arg1) {
        arg0.bindVertexArrayOES(arg1);
    };
    imports.wbg.__wbg_pixelStorei_f97b971917582269 = function(arg0, arg1, arg2) {
        arg0.pixelStorei(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_pixelStorei_a0b83efc92cd29fe = function(arg0, arg1, arg2) {
        arg0.pixelStorei(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bufferData_05664df801d7aec0 = function(arg0, arg1, arg2, arg3) {
        arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferData_573e61c49a480c4d = function(arg0, arg1, arg2, arg3) {
        arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferData_023700b2ed207c43 = function(arg0, arg1, arg2, arg3) {
        arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferData_16f948547d74c866 = function(arg0, arg1, arg2, arg3) {
        arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferSubData_4e653f611d7a962d = function(arg0, arg1, arg2, arg3) {
        arg0.bufferSubData(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_bufferSubData_c7180c0b681078e8 = function(arg0, arg1, arg2, arg3) {
        arg0.bufferSubData(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_getBufferSubData_cd8138c86821bca3 = function(arg0, arg1, arg2, arg3) {
        arg0.getBufferSubData(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_clearBufferiv_fe2a00a8f8fb7322 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearBufferiv(arg1 >>> 0, arg2, getArrayI32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_clearBufferuiv_a41730a8d84c6ac6 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearBufferuiv(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_clearBufferfv_b3c90fbed3b74920 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearBufferfv(arg1 >>> 0, arg2, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_clearBufferfi_95daf829c568e58a = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearBufferfi(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_clientWaitSync_ae8f3712f85a57fb = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.clientWaitSync(arg1, arg2 >>> 0, arg3 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_copyBufferSubData_c903618a0e0a9fca = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.copyBufferSubData(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_copyTexSubImage2D_7c0b0080eece3c1a = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        arg0.copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
    };
    imports.wbg.__wbg_copyTexSubImage2D_47b14ff8459fd4c8 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        arg0.copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
    };
    imports.wbg.__wbg_copyTexSubImage3D_88fc9e1c56d3e7db = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.copyTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
    };
    imports.wbg.__wbg_deleteBuffer_17feed38f3a70ec9 = function(arg0, arg1) {
        arg0.deleteBuffer(arg1);
    };
    imports.wbg.__wbg_deleteBuffer_cf67a696a7857b3f = function(arg0, arg1) {
        arg0.deleteBuffer(arg1);
    };
    imports.wbg.__wbg_deleteFramebuffer_130abca01c89b7d6 = function(arg0, arg1) {
        arg0.deleteFramebuffer(arg1);
    };
    imports.wbg.__wbg_deleteFramebuffer_f9c2bceeb5422d9d = function(arg0, arg1) {
        arg0.deleteFramebuffer(arg1);
    };
    imports.wbg.__wbg_deleteQuery_0981fb4d492e46a7 = function(arg0, arg1) {
        arg0.deleteQuery(arg1);
    };
    imports.wbg.__wbg_deleteRenderbuffer_385f3c9e8759b99e = function(arg0, arg1) {
        arg0.deleteRenderbuffer(arg1);
    };
    imports.wbg.__wbg_deleteRenderbuffer_cad502ac8d1398f2 = function(arg0, arg1) {
        arg0.deleteRenderbuffer(arg1);
    };
    imports.wbg.__wbg_deleteSampler_6d832d1900eafbea = function(arg0, arg1) {
        arg0.deleteSampler(arg1);
    };
    imports.wbg.__wbg_deleteSync_f8f026807b7eee54 = function(arg0, arg1) {
        arg0.deleteSync(arg1);
    };
    imports.wbg.__wbg_deleteTexture_605a36a7e380df5f = function(arg0, arg1) {
        arg0.deleteTexture(arg1);
    };
    imports.wbg.__wbg_deleteTexture_1b5f5e536e0d5545 = function(arg0, arg1) {
        arg0.deleteTexture(arg1);
    };
    imports.wbg.__wbg_disable_65425605098b79cf = function(arg0, arg1) {
        arg0.disable(arg1 >>> 0);
    };
    imports.wbg.__wbg_disable_3adb8645ea1d92d4 = function(arg0, arg1) {
        arg0.disable(arg1 >>> 0);
    };
    imports.wbg.__wbg_disableVertexAttribArray_cf25f8beb5872364 = function(arg0, arg1) {
        arg0.disableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_disableVertexAttribArray_f469283fda607cee = function(arg0, arg1) {
        arg0.disableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_drawArrays_e5fa3cfc2b5d7c6d = function(arg0, arg1, arg2, arg3) {
        arg0.drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_drawArrays_84de8a2416396807 = function(arg0, arg1, arg2, arg3) {
        arg0.drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_drawArraysInstanced_1222b6236d008088 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_drawArraysInstancedANGLE_403faa11d52ccf6d = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_new_b525de17f44a8943 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbg_push_49c286f04dd3bf59 = function(arg0, arg1) {
        const ret = arg0.push(arg1);
        return ret;
    };
    imports.wbg.__wbg_of_892d7838f8e4cc20 = function(arg0) {
        const ret = Array.of(arg0);
        return ret;
    };
    imports.wbg.__wbg_drawBuffersWEBGL_dfb0d803ea7ebe07 = function(arg0, arg1) {
        arg0.drawBuffersWEBGL(arg1);
    };
    imports.wbg.__wbg_drawBuffers_3223f0aeb44f7057 = function(arg0, arg1) {
        arg0.drawBuffers(arg1);
    };
    imports.wbg.__wbg_drawElementsInstanced_b4714f8dd90fd2a8 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.drawElementsInstanced(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_drawElementsInstancedANGLE_0230afc27cf9cec9 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.drawElementsInstancedANGLE(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_enable_2c3b6a4692af9b1b = function(arg0, arg1) {
        arg0.enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enable_1ac9f14a577b7c8b = function(arg0, arg1) {
        arg0.enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_6dd3d0668209ae19 = function(arg0, arg1) {
        arg0.enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_53139716d9c95dba = function(arg0, arg1) {
        arg0.enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_framebufferRenderbuffer_77bdb2f359a5728f = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4);
    };
    imports.wbg.__wbg_framebufferRenderbuffer_3bf1420713a0b21a = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4);
    };
    imports.wbg.__wbg_framebufferTexture2D_885176f16a153fec = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_framebufferTexture2D_ed03c0674b9979ce = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_framebufferTextureLayer_e644333b8ec36f9d = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.framebufferTextureLayer(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_frontFace_00177185d2fae697 = function(arg0, arg1) {
        arg0.frontFace(arg1 >>> 0);
    };
    imports.wbg.__wbg_frontFace_3d7784c56ffede8a = function(arg0, arg1) {
        arg0.frontFace(arg1 >>> 0);
    };
    imports.wbg.__wbg_getParameter_d6cd2dd2cde656ec = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.getParameter(arg1 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getIndexedParameter_5f5c79f6c05edd18 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.getIndexedParameter(arg1 >>> 0, arg2 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getUniformLocation_b46e5db76599a918 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getUniformLocation_c6caabb349b43da7 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getSyncParameter_b2f55318719e958c = function(arg0, arg1, arg2) {
        const ret = arg0.getSyncParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_renderbufferStorage_37eab84be1494aef = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
    };
    imports.wbg.__wbg_renderbufferStorage_2192d9cd09128339 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
    };
    imports.wbg.__wbg_renderbufferStorageMultisample_9cb173d2fd461513 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.renderbufferStorageMultisample(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_samplerParameterf_38ca759dc5c40461 = function(arg0, arg1, arg2, arg3) {
        arg0.samplerParameterf(arg1, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_samplerParameteri_c631c02ceefc6dc1 = function(arg0, arg1, arg2, arg3) {
        arg0.samplerParameteri(arg1, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_texStorage2D_89c29252632da923 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.texStorage2D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_texStorage3D_3897fb6b91eb82d8 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        arg0.texStorage3D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5, arg6);
    };
    imports.wbg.__wbg_uniform1i_1f8256271b54cf41 = function(arg0, arg1, arg2) {
        arg0.uniform1i(arg1, arg2);
    };
    imports.wbg.__wbg_uniform1i_4fdc6d6740375d22 = function(arg0, arg1, arg2) {
        arg0.uniform1i(arg1, arg2);
    };
    imports.wbg.__wbg_uniform2iv_58c3d5ee9e70c71d = function(arg0, arg1, arg2, arg3) {
        arg0.uniform2iv(arg1, getArrayI32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform2iv_32329f9a4d491136 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform2iv(arg1, getArrayI32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform3iv_0a103fe131bd9213 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform3iv(arg1, getArrayI32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform3iv_100a284f5a3cbca5 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform3iv(arg1, getArrayI32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform4iv_9436eeda2a27cce8 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform4iv(arg1, getArrayI32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform4iv_7f03c41e6e49bbd6 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform4iv(arg1, getArrayI32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform1f_062c683ec584f7e8 = function(arg0, arg1, arg2) {
        arg0.uniform1f(arg1, arg2);
    };
    imports.wbg.__wbg_uniform1f_dcc6951bde745417 = function(arg0, arg1, arg2) {
        arg0.uniform1f(arg1, arg2);
    };
    imports.wbg.__wbg_uniform4f_68fac972655f5359 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.uniform4f(arg1, arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_uniform4f_19b349303edb7836 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.uniform4f(arg1, arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_uniform2fv_c29ce786946f1aae = function(arg0, arg1, arg2, arg3) {
        arg0.uniform2fv(arg1, getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform2fv_ffd0b1d3c3a4070a = function(arg0, arg1, arg2, arg3) {
        arg0.uniform2fv(arg1, getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform3fv_5ca48b3279e0c643 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform3fv(arg1, getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform3fv_bc831e48acb2c057 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform3fv(arg1, getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform4fv_14f1c5ef10bfb4c9 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform4fv(arg1, getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform4fv_26d822da5c3fdb00 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform4fv(arg1, getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniformMatrix2fv_1a40e9f63b2005c8 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniformMatrix2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix2fv_5f1f56c7cbfb533f = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniformMatrix2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix3fv_dcde28ba8c34d30e = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniformMatrix3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix3fv_ae9271db8127a57b = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniformMatrix3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix4fv_4575a018c8188146 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniformMatrix4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix4fv_0f42d678a568ded9 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniformMatrix4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_cullFace_d4450f8718c6b3eb = function(arg0, arg1) {
        arg0.cullFace(arg1 >>> 0);
    };
    imports.wbg.__wbg_cullFace_79e4ddbea13278b3 = function(arg0, arg1) {
        arg0.cullFace(arg1 >>> 0);
    };
    imports.wbg.__wbg_colorMask_99120a2c8caf1298 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
    };
    imports.wbg.__wbg_colorMask_a7f067283ed312c9 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
    };
    imports.wbg.__wbg_depthMask_134f9e3073ca4fd0 = function(arg0, arg1) {
        arg0.depthMask(arg1 !== 0);
    };
    imports.wbg.__wbg_depthMask_27d367443a80541d = function(arg0, arg1) {
        arg0.depthMask(arg1 !== 0);
    };
    imports.wbg.__wbg_blendColor_13739d87434b79c3 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.blendColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_blendColor_a17ddceb3534e0b3 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.blendColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_invalidateFramebuffer_696c3c456c34a207 = function() { return handleError(function (arg0, arg1, arg2) {
        arg0.invalidateFramebuffer(arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_polygonOffset_fb73618b77fd3f6f = function(arg0, arg1, arg2) {
        arg0.polygonOffset(arg1, arg2);
    };
    imports.wbg.__wbg_polygonOffset_03d3955d5a1afa08 = function(arg0, arg1, arg2) {
        arg0.polygonOffset(arg1, arg2);
    };
    imports.wbg.__wbg_bindTexture_0c284b1604ba527c = function(arg0, arg1, arg2) {
        arg0.bindTexture(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bindTexture_c1c0e00507424f8e = function(arg0, arg1, arg2) {
        arg0.bindTexture(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_bindSampler_6eb88b542e5a410f = function(arg0, arg1, arg2) {
        arg0.bindSampler(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_activeTexture_01d5469eb22c10e7 = function(arg0, arg1) {
        arg0.activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_activeTexture_0daf7c1698e49f00 = function(arg0, arg1) {
        arg0.activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_fenceSync_fb3e1185847ee462 = function(arg0, arg1, arg2) {
        const ret = arg0.fenceSync(arg1 >>> 0, arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_texParameteri_9fbb09bbf9670af4 = function(arg0, arg1, arg2, arg3) {
        arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_texParameteri_e0ce3810261e0864 = function(arg0, arg1, arg2, arg3) {
        arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_texSubImage2D_53b6a050a0b9b24e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_6a8b0f3381d734c3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_57792696288b0a61 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_compressedTexSubImage2D_23b602b828848fb7 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8, arg9);
    };
    imports.wbg.__wbg_compressedTexSubImage2D_d6c95fc640a9f4de = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8);
    };
    imports.wbg.__wbg_compressedTexSubImage2D_788296e97b316838 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8);
    };
    imports.wbg.__wbg_texSubImage3D_84ef903e11598af0 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
        arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
    }, arguments) };
    imports.wbg.__wbg_texSubImage3D_1d82135e9ce965bf = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
        arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
    }, arguments) };
    imports.wbg.__wbg_compressedTexSubImage3D_00b794917e65d559 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
        arg0.compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10, arg11);
    };
    imports.wbg.__wbg_compressedTexSubImage3D_c9c7b42e0f7db586 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        arg0.compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10);
    };
    imports.wbg.__wbg_depthFunc_00d8a905436dc681 = function(arg0, arg1) {
        arg0.depthFunc(arg1 >>> 0);
    };
    imports.wbg.__wbg_depthFunc_2060ec3687ac1f95 = function(arg0, arg1) {
        arg0.depthFunc(arg1 >>> 0);
    };
    imports.wbg.__wbg_depthRange_f34f19edea1feadd = function(arg0, arg1, arg2) {
        arg0.depthRange(arg1, arg2);
    };
    imports.wbg.__wbg_depthRange_7109c2393819a37b = function(arg0, arg1, arg2) {
        arg0.depthRange(arg1, arg2);
    };
    imports.wbg.__wbg_scissor_8bc2e761846f53f0 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.scissor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_scissor_2b084e0dc81d67f4 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.scissor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_vertexAttribDivisor_77f020121066a4d9 = function(arg0, arg1, arg2) {
        arg0.vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_vertexAttribDivisorANGLE_6bbb3df4c6e7d08b = function(arg0, arg1, arg2) {
        arg0.vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_vertexAttribPointer_ccabef9be68fe1c4 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_vertexAttribPointer_ad370785358334f4 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_vertexAttribIPointer_b15ad1437a268cf5 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        arg0.vertexAttribIPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_viewport_4bdfc4b8959593ee = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_viewport_cc41e28a71c23915 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_blendFunc_f4365f78b650180f = function(arg0, arg1, arg2) {
        arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_blendFunc_d456b0c766f8dbc9 = function(arg0, arg1, arg2) {
        arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_blendFuncSeparate_b508053691b6ebbe = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_blendFuncSeparate_9a7146974b3cd76d = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_blendEquation_562c3267161e4675 = function(arg0, arg1) {
        arg0.blendEquation(arg1 >>> 0);
    };
    imports.wbg.__wbg_blendEquation_b5d5be767bd3835a = function(arg0, arg1) {
        arg0.blendEquation(arg1 >>> 0);
    };
    imports.wbg.__wbg_blendEquationSeparate_48b95e78f7224be4 = function(arg0, arg1, arg2) {
        arg0.blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_blendEquationSeparate_d2fa3b718ee3579f = function(arg0, arg1, arg2) {
        arg0.blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_stencilFuncSeparate_510d3287542b4574 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
    };
    imports.wbg.__wbg_stencilFuncSeparate_3be68afd7ca6efcc = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
    };
    imports.wbg.__wbg_stencilMask_e1887eeaabe22771 = function(arg0, arg1) {
        arg0.stencilMask(arg1 >>> 0);
    };
    imports.wbg.__wbg_stencilMask_144b86d15d9fdbe6 = function(arg0, arg1) {
        arg0.stencilMask(arg1 >>> 0);
    };
    imports.wbg.__wbg_stencilMaskSeparate_e89abefeb5641657 = function(arg0, arg1, arg2) {
        arg0.stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_stencilMaskSeparate_84a2494b967772c7 = function(arg0, arg1, arg2) {
        arg0.stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_stencilOpSeparate_aa3d09aa448a6f48 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_stencilOpSeparate_1708aea1aea0dc48 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
    };
    imports.wbg.__wbg_getUniformBlockIndex_a05b0c144aa49817 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getUniformBlockIndex(arg1, getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_uniformBlockBinding_1971f4528d9c3043 = function(arg0, arg1, arg2, arg3) {
        arg0.uniformBlockBinding(arg1, arg2 >>> 0, arg3 >>> 0);
    };
    imports.wbg.__wbg_readBuffer_bade27c1171e00cf = function(arg0, arg1) {
        arg0.readBuffer(arg1 >>> 0);
    };
    imports.wbg.__wbg_readPixels_30de7174c15126d3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
    }, arguments) };
    imports.wbg.__wbg_readPixels_493558abd28a3b61 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
    }, arguments) };
    imports.wbg.__wbg_readPixels_92102ee9fe1c81a0 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
    }, arguments) };
    imports.wbg.__wbg_beginQuery_fb152d8d84f2b130 = function(arg0, arg1, arg2) {
        arg0.beginQuery(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_endQuery_726967da9d5d1ca7 = function(arg0, arg1) {
        arg0.endQuery(arg1 >>> 0);
    };
    imports.wbg.__wbg_getQueryParameter_e0f43fb85f793bbe = function(arg0, arg1, arg2) {
        const ret = arg0.getQueryParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_get_baf4855f9a986186 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(arg0, arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_now_c644db5194be8437 = function(arg0) {
        const ret = arg0.now();
        return ret;
    };
    imports.wbg.__wbg_self_e7c1f827057f6584 = function() { return handleError(function () {
        const ret = self.self;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_window_a09ec664e14b1b81 = function() { return handleError(function () {
        const ret = window.window;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_globalThis_87cbb8506fecf3a9 = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_global_c85a9259e621f3db = function() { return handleError(function () {
        const ret = global.global;
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg_newnoargs_2b8b6bd7753c76ba = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_call_95d1ea488d03e4e8 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_resolve_fd40f858d9db1a04 = function(arg0) {
        const ret = Promise.resolve(arg0);
        return ret;
    };
    imports.wbg.__wbg_then_ec5db6d509eb475f = function(arg0, arg1) {
        const ret = arg0.then(arg1);
        return ret;
    };
    imports.wbg.__wbg_then_f753623316e2873a = function(arg0, arg1, arg2) {
        const ret = arg0.then(arg1, arg2);
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_55f9ffb569d9fa74 = function(arg0, arg1, arg2) {
        const ret = new Int8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_f477e654086cbbb6 = function(arg0, arg1, arg2) {
        const ret = new Int16Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_b57a602974d4b1cd = function(arg0, arg1, arg2) {
        const ret = new Int32Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_9fb2f11355ecadf5 = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_set_17499e8aa4003ebd = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_length_27a2afe8ab42b09f = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_9241d9d251418ebf = function(arg0, arg1, arg2) {
        const ret = new Uint16Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_5c5a6e21987c3bee = function(arg0, arg1, arg2) {
        const ret = new Uint32Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_4078d56428eb2926 = function(arg0, arg1, arg2) {
        const ret = new Float32Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_set_6aa458a4ebdb65cb = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(arg0, arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newwithcontextoptions_6c6a79a71ed7efa3 = function() { return handleError(function (arg0) {
        const ret = new lAudioContext(arg0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createBuffer_d142e00390bff447 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = arg0.createBuffer(arg1 >>> 0, arg2 >>> 0, arg3);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_currentTime_d94729a1b5fd59a5 = function(arg0) {
        const ret = arg0.currentTime;
        return ret;
    };
    imports.wbg.__wbg_createBufferSource_1473226efd418a08 = function() { return handleError(function (arg0) {
        const ret = arg0.createBufferSource();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setbuffer_bad384d1628a8306 = function(arg0, arg1) {
        arg0.buffer = arg1;
    };
    imports.wbg.__wbg_destination_5dfc354bcf2eb941 = function(arg0) {
        const ret = arg0.destination;
        return ret;
    };
    imports.wbg.__wbg_connect_77f2f818a74097e1 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.connect(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setonended_15b13187aec41ac9 = function(arg0, arg1) {
        arg0.onended = arg1;
    };
    imports.wbg.__wbg_start_9169e040a16354b9 = function() { return handleError(function (arg0, arg1) {
        arg0.start(arg1);
    }, arguments) };
    imports.wbg.__wbg_copyToChannel_9babe9bf308bffc3 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        arg0.copyToChannel(getArrayF32FromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_measure_aa7a73f17813f708 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        try {
            performance.measure(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
            wasm.__wbindgen_free(arg2, arg3);
        }
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_instanceof_WebGl2RenderingContext_61bb2cb23346dbb7 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof WebGL2RenderingContext;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_getProgramInfoLog_7654794297967ac0 = function(arg0, arg1, arg2) {
        const ret = arg1.getProgramInfoLog(arg2);
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderInfoLog_915d0e8506c11159 = function(arg0, arg1, arg2) {
        const ret = arg1.getShaderInfoLog(arg2);
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_innerWidth_7e9d12e05bcb598e = function() { return handleError(function (arg0) {
        const ret = arg0.innerWidth;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_innerHeight_3ef25a30618357e0 = function() { return handleError(function (arg0) {
        const ret = arg0.innerHeight;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_devicePixelRatio_5f8f5cab76864090 = function(arg0) {
        const ret = arg0.devicePixelRatio;
        return ret;
    };
    imports.wbg.__wbg_cancelAnimationFrame_d079cdb83bc43b26 = function() { return handleError(function (arg0, arg1) {
        arg0.cancelAnimationFrame(arg1);
    }, arguments) };
    imports.wbg.__wbg_matchMedia_967e50e4289050fa = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.matchMedia(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_open_caf5dfe2d159a600 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = arg0.open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_get_e6ae480a4b8df368 = function(arg0, arg1, arg2) {
        const ret = arg0[getStringFromWasm0(arg1, arg2)];
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_clearTimeout_b2b8af0f044e02e9 = function(arg0, arg1) {
        arg0.clearTimeout(arg1);
    };
    imports.wbg.__wbg_instanceof_Window_e266f02eee43b570 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setProperty_21e2e7868b86a93e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_deltaX_b7d127c94d6265c0 = function(arg0) {
        const ret = arg0.deltaX;
        return ret;
    };
    imports.wbg.__wbg_deltaY_b32fa858e16edcc0 = function(arg0) {
        const ret = arg0.deltaY;
        return ret;
    };
    imports.wbg.__wbg_deltaMode_11f7b19e64d9a515 = function(arg0) {
        const ret = arg0.deltaMode;
        return ret;
    };
    imports.wbg.__wbg_setwidth_81c62bc806e0a727 = function(arg0, arg1) {
        arg0.width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setheight_98cf0db22c40ef07 = function(arg0, arg1) {
        arg0.height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_89a318b610dc5fd4 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = arg0.getContext(getStringFromWasm0(arg1, arg2), arg3);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_videoWidth_41c6e04eac7ce78b = function(arg0) {
        const ret = arg0.videoWidth;
        return ret;
    };
    imports.wbg.__wbg_videoHeight_8ef0e09e1674d6fc = function(arg0) {
        const ret = arg0.videoHeight;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_31e62e9d172b26f0 = function(arg0) {
        const ret = arg0.shiftKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_9f0f19692d0498bd = function(arg0) {
        const ret = arg0.metaKey;
        return ret;
    };
    imports.wbg.__wbg_code_aed21120de275a12 = function(arg0, arg1) {
        const ret = arg1.code;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_fullscreenElement_65f14a4df7c25129 = function(arg0) {
        const ret = arg0.fullscreenElement;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createElement_e2a0e21263eb5416 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_exitFullscreen_36506b10bd87f8b8 = function(arg0) {
        arg0.exitFullscreen();
    };
    imports.wbg.__wbg_exitPointerLock_c255b2b7e186916c = function(arg0) {
        arg0.exitPointerLock();
    };
    imports.wbg.__wbg_getProgramInfoLog_7fd2a7c6c1a280c1 = function(arg0, arg1, arg2) {
        const ret = arg1.getProgramInfoLog(arg2);
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderInfoLog_d057293074e59c61 = function(arg0, arg1, arg2) {
        const ret = arg1.getShaderInfoLog(arg2);
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setAttribute_79c9562d32d05e66 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_style_2141664e428fef46 = function(arg0) {
        const ret = arg0.style;
        return ret;
    };
    imports.wbg.__wbg_x_0938e87a3ff14a2e = function(arg0) {
        const ret = arg0.x;
        return ret;
    };
    imports.wbg.__wbg_y_b881176a43492948 = function(arg0) {
        const ret = arg0.y;
        return ret;
    };
    imports.wbg.__wbg_width_716d2242c9bd6c20 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbg_height_d45e4d57562f8ae0 = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_width_8a17f65e11a44bf6 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbg_height_641410e41fce27a6 = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_matches_7b5ad9e6bb56f1f3 = function(arg0) {
        const ret = arg0.matches;
        return ret;
    };
    imports.wbg.__wbg_addListener_dfc3f9e430149b14 = function() { return handleError(function (arg0, arg1) {
        arg0.addListener(arg1);
    }, arguments) };
    imports.wbg.__wbg_removeListener_6f811d2fb59768b9 = function() { return handleError(function (arg0, arg1) {
        arg0.removeListener(arg1);
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_e1b8f1de1eb24d5d = function(arg0) {
        const ret = arg0.ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_fdd99b6df96e25c5 = function(arg0) {
        const ret = arg0.shiftKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_d531a4d3704557cb = function(arg0) {
        const ret = arg0.altKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_934772989e28020c = function(arg0) {
        const ret = arg0.metaKey;
        return ret;
    };
    imports.wbg.__wbg_button_a1c470d5e4c997f2 = function(arg0) {
        const ret = arg0.button;
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper2393 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1592, __wbg_adapter_36);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39769 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39771 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39773 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39775 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39777 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39779 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper39781 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 27817, __wbg_adapter_39);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper48903 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 30050, __wbg_adapter_54);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper50336 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 30344, __wbg_adapter_57);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper54251 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 31219, __wbg_adapter_60);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat32Memory0 = null;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('radiation_sim_bin_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
