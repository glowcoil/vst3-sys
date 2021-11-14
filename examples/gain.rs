use vst3_sys::*;

use std::ffi::c_void;
use std::os::raw::c_char;

#[repr(C)]
struct Factory {
    plugin_factory_3: *const IPluginFactory3,
}

unsafe impl Sync for Factory {}

impl Factory {
    unsafe extern "system" fn query_interface(
        this: *mut c_void,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> TResult {
        let iid = *iid;

        if iid == FUnknown::IID
            || iid == IPluginFactory::IID
            || iid == IPluginFactory2::IID
            || iid == IPluginFactory3::IID
        {
            *obj = this;
            return result::OK;
        }

        result::NO_INTERFACE
    }

    unsafe extern "system" fn add_ref(_this: *mut c_void) -> u32 {
        1
    }

    unsafe extern "system" fn release(_this: *mut c_void) -> u32 {
        1
    }

    unsafe extern "system" fn get_factory_info(
        _this: *mut c_void,
        _info: *mut PFactoryInfo,
    ) -> TResult {
        result::NOT_IMPLEMENTED
    }

    unsafe extern "system" fn count_classes(_this: *mut c_void) -> i32 {
        0
    }

    unsafe extern "system" fn get_class_info(
        _this: *mut c_void,
        _index: i32,
        _info: *mut PClassInfo,
    ) -> TResult {
        result::NOT_IMPLEMENTED
    }

    unsafe extern "system" fn create_instance(
        _this: *mut c_void,
        _cid: *const c_char,
        _iid: *const c_char,
        _obj: *mut *mut c_void,
    ) -> TResult {
        result::NOT_IMPLEMENTED
    }

    unsafe extern "system" fn get_class_info_2(
        _this: *mut c_void,
        _index: i32,
        _info: *mut PClassInfo2,
    ) -> TResult {
        result::NOT_IMPLEMENTED
    }

    unsafe extern "system" fn get_class_info_unicode(
        _this: *mut c_void,
        _index: i32,
        _info: *mut PClassInfoW,
    ) -> TResult {
        result::NOT_IMPLEMENTED
    }

    unsafe extern "system" fn set_host_context(
        _this: *mut c_void,
        _context: *mut *const FUnknown,
    ) -> TResult {
        result::NOT_IMPLEMENTED
    }
}

static PLUGIN_FACTORY_3_VTABLE: IPluginFactory3 = IPluginFactory3 {
    plugin_factory_2: IPluginFactory2 {
        plugin_factory: IPluginFactory {
            unknown: FUnknown {
                query_interface: Factory::query_interface,
                add_ref: Factory::add_ref,
                release: Factory::release,
            },
            get_factory_info: Factory::get_factory_info,
            count_classes: Factory::count_classes,
            get_class_info: Factory::get_class_info,
            create_instance: Factory::create_instance,
        },
        get_class_info_2: Factory::get_class_info_2,
    },
    get_class_info_unicode: Factory::get_class_info_unicode,
    set_host_context: Factory::set_host_context,
};

static PLUGIN_FACTORY: Factory = Factory {
    plugin_factory_3: &PLUGIN_FACTORY_3_VTABLE,
};

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn InitDll() -> bool {
    true
}

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn ExitDll() -> bool {
    true
}

#[cfg(target_os = "macos")]
#[no_mangle]
extern "system" fn BundleEntry(_bundle_ref: *mut c_void) -> bool {
    true
}

#[cfg(target_os = "macos")]
#[no_mangle]
extern "system" fn BundleExit() -> bool {
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
extern "system" fn ModuleEntry(_library_handle: *mut c_void) -> bool {
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
extern "system" fn ModuleExit() -> bool {
    true
}

#[no_mangle]
extern "system" fn GetPluginFactory() -> *mut c_void {
    &PLUGIN_FACTORY as *const Factory as *mut c_void
}
