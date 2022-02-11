import * as wasm from './spicyrs_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
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

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_18(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h105bbc8ca21101f7(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_21(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__haca42b9f3831d43c(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_24(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb8e7f6464bd43965(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf85346c18b5f5b7f(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_30(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h099a01f00f5872b5(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_33(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h97a58552085664f6(arg0, arg1, addHeapObject(arg2));
}

/**
*/
export function set_up() {
    wasm.set_up();
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_log_8af2ce82aab5c10d(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_string_new(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbindgen_object_clone_ref(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_string_get(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbindgen_number_new(arg0) {
    var ret = arg0;
    return addHeapObject(ret);
};

export function __wbg_instanceof_Window_c4b70662a0d2c5ec(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};

export function __wbg_document_1c64944725c0d81d(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_localStorage_6775414303ab5085() { return handleError(function (arg0) {
    var ret = getObject(arg0).localStorage;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

export function __wbg_createElement_86c152812a141a62() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_querySelector_b92a6c73bcfe671b() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

export function __wbg_querySelectorAll_7f26183d7dfc576e() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).querySelectorAll(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_type_7a49279491e15d0a(arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_ctrlKey_fb62ba10b63b34a4(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};

export function __wbg_key_10dcaa4bb6d5449f(arg0, arg1) {
    var ret = getObject(arg1).key;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e(arg0) {
    var ret = getObject(arg0) instanceof HTMLCanvasElement;
    return ret;
};

export function __wbg_setwidth_c1a7061891b71f25(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

export function __wbg_setheight_88894b05710ff752(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

export function __wbg_getContext_f701d0231ae22393() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

export function __wbg_length_62e6735d81b8b0f1(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};

export function __wbg_item_19347f9da4575496(arg0, arg1) {
    var ret = getObject(arg0).item(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_new_a1e3cb63557e0336() { return handleError(function (arg0) {
    var ret = new MutationObserver(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_observe_be333cec1293c673() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).observe(getObject(arg1), getObject(arg2));
}, arguments) };

export function __wbg_appendChild_d318db34c4559916() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_setProperty_1460c660bc329763() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

export function __wbg_instanceof_HtmlSelectElement_27fb687660e6b5ba(arg0) {
    var ret = getObject(arg0) instanceof HTMLSelectElement;
    return ret;
};

export function __wbg_value_bc4bb925ad58795b(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_getItem_77fb9d4666f3b93a() { return handleError(function (arg0, arg1, arg2, arg3) {
    var ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

export function __wbg_setItem_b0c4561489dffecd() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

export function __wbg_instanceof_Element_97d85e53f1805b82(arg0) {
    var ret = getObject(arg0) instanceof Element;
    return ret;
};

export function __wbg_classList_b666640fdfbcc8ab(arg0) {
    var ret = getObject(arg0).classList;
    return addHeapObject(ret);
};

export function __wbg_setinnerHTML_e5b817d6227a431c(arg0, arg1, arg2) {
    getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
};

export function __wbg_getAttribute_bb1d602e925e860a(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_setAttribute_1b533bf07966de55() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

export function __wbg_instanceof_HtmlElement_df66c8b4a687aa43(arg0) {
    var ret = getObject(arg0) instanceof HTMLElement;
    return ret;
};

export function __wbg_style_c88e323890d3a091(arg0) {
    var ret = getObject(arg0).style;
    return addHeapObject(ret);
};

export function __wbg_offsetWidth_69cd6669725b154f(arg0) {
    var ret = getObject(arg0).offsetWidth;
    return ret;
};

export function __wbg_offsetHeight_8da312843e7777ab(arg0) {
    var ret = getObject(arg0).offsetHeight;
    return ret;
};

export function __wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae(arg0) {
    var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
    return ret;
};

export function __wbg_setstrokeStyle_947bd4c26c94673f(arg0, arg1) {
    getObject(arg0).strokeStyle = getObject(arg1);
};

export function __wbg_setfillStyle_528a6a267c863ae7(arg0, arg1) {
    getObject(arg0).fillStyle = getObject(arg1);
};

export function __wbg_setlineWidth_3221b7818c00ed48(arg0, arg1) {
    getObject(arg0).lineWidth = arg1;
};

export function __wbg_setlineCap_5284a001e1efcecd(arg0, arg1, arg2) {
    getObject(arg0).lineCap = getStringFromWasm0(arg1, arg2);
};

export function __wbg_setlineJoin_a6af4e7d24a3a67e(arg0, arg1, arg2) {
    getObject(arg0).lineJoin = getStringFromWasm0(arg1, arg2);
};

export function __wbg_settextAlign_1891d6f4d7f9b9a3(arg0, arg1, arg2) {
    getObject(arg0).textAlign = getStringFromWasm0(arg1, arg2);
};

export function __wbg_beginPath_733d5a9e3e769d24(arg0) {
    getObject(arg0).beginPath();
};

export function __wbg_fill_dc4e97599365a189(arg0) {
    getObject(arg0).fill();
};

export function __wbg_stroke_7cdcdf3d07636d76(arg0) {
    getObject(arg0).stroke();
};

export function __wbg_setLineDash_af4d2b484e0801a1() { return handleError(function (arg0, arg1) {
    getObject(arg0).setLineDash(getObject(arg1));
}, arguments) };

export function __wbg_arc_bdfc39ad6001708b() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).arc(arg1, arg2, arg3, arg4, arg5);
}, arguments) };

export function __wbg_arcTo_09ad374047962b9e() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).arcTo(arg1, arg2, arg3, arg4, arg5);
}, arguments) };

export function __wbg_closePath_64f527552526a127(arg0) {
    getObject(arg0).closePath();
};

export function __wbg_lineTo_fde385edd804f315(arg0, arg1, arg2) {
    getObject(arg0).lineTo(arg1, arg2);
};

export function __wbg_moveTo_18ace182fe51d75d(arg0, arg1, arg2) {
    getObject(arg0).moveTo(arg1, arg2);
};

export function __wbg_clearRect_07caefec3496ced1(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
};

export function __wbg_fillRect_10e42dc7a5e8cccd(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
};

export function __wbg_strokeRect_74c84ef5e5ba1eaa(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).strokeRect(arg1, arg2, arg3, arg4);
};

export function __wbg_restore_fa948aac9e973228(arg0) {
    getObject(arg0).restore();
};

export function __wbg_save_552f7f081f942847(arg0) {
    getObject(arg0).save();
};

export function __wbg_fillText_25221e9cc35a1850() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
}, arguments) };

export function __wbg_rotate_360dbdd13dc1b620() { return handleError(function (arg0, arg1) {
    getObject(arg0).rotate(arg1);
}, arguments) };

export function __wbg_scale_2dde44389fea3e82() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).scale(arg1, arg2);
}, arguments) };

export function __wbg_translate_0b8c117f3669666a() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).translate(arg1, arg2);
}, arguments) };

export function __wbg_instanceof_HtmlInputElement_8cafe5f30dfdb6bc(arg0) {
    var ret = getObject(arg0) instanceof HTMLInputElement;
    return ret;
};

export function __wbg_checked_39d5ce76226024a7(arg0) {
    var ret = getObject(arg0).checked;
    return ret;
};

export function __wbg_setchecked_206243371da58f6a(arg0, arg1) {
    getObject(arg0).checked = arg1 !== 0;
};

export function __wbg_value_0627d4b1c27534e6(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_offsetX_5da3ebf8a8cda8a4(arg0) {
    var ret = getObject(arg0).offsetX;
    return ret;
};

export function __wbg_offsetY_b0edbc16723a55cb(arg0) {
    var ret = getObject(arg0).offsetY;
    return ret;
};

export function __wbg_ctrlKey_9761d22fa42f09c0(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};

export function __wbg_add_f36d97e1d70d27b0() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).add(getStringFromWasm0(arg1, arg2));
}, arguments) };

export function __wbg_remove_89670e56a41482a8() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).remove(getStringFromWasm0(arg1, arg2));
}, arguments) };

export function __wbg_detail_759d550c59ed0f81(arg0) {
    var ret = getObject(arg0).detail;
    return addHeapObject(ret);
};

export function __wbg_newwitheventinitdict_398fe79b8552d230() { return handleError(function (arg0, arg1, arg2) {
    var ret = new CustomEvent(getStringFromWasm0(arg0, arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_addEventListener_52721772cc0a7f30() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
}, arguments) };

export function __wbg_dispatchEvent_88350606c977dfef() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).dispatchEvent(getObject(arg1));
    return ret;
}, arguments) };

export function __wbg_deltaY_080604c20160c0e8(arg0) {
    var ret = getObject(arg0).deltaY;
    return ret;
};

export function __wbg_newnoargs_be86524d73f67598(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_call_888d259a5fefc347() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_new_0b83d3df67ecb33e() {
    var ret = new Object();
    return addHeapObject(ret);
};

export function __wbg_newwithlength_75ee2b96c288e6bc(arg0) {
    var ret = new Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_set_1820441f7fb79aad(arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
};

export function __wbg_self_c6fbdfc2918d5e58() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_window_baec038b5ab35c54() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_globalThis_3f735a5746d41fbd() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_global_1bc0b39582740e95() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_is_undefined(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbg_set_82a4e8a85e31ac42() { return handleError(function (arg0, arg1, arg2) {
    var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments) };

export function __wbg_new_693216e109162396() {
    var ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_error_09919627ac0992f5(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export function __wbindgen_debug_string(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_closure_wrapper448(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 160, __wbg_adapter_18);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper450(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 168, __wbg_adapter_21);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper452(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 166, __wbg_adapter_24);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper454(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 158, __wbg_adapter_27);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper456(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 162, __wbg_adapter_30);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper458(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 164, __wbg_adapter_33);
    return addHeapObject(ret);
};

