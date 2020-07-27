#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;
mod bakkesmod {
    #![allow(unused)]
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::atomic::AtomicBool;
    use std::sync::Mutex;
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;
    use super::wrappers::CarWrapper;
    use super::wrappers::Car;
    use super::wrappers::Object;
    use super::wrappers::Canvas;
    trait BakkesMod {
        fn id(&self) -> u64;
        fn add_notifier_callback(&self, addr: usize);
        fn add_hook_callback(&self, addr: usize);
        fn add_drawable_callback(&self, addr: usize);
    }
    static mut BAKKESMOD: &dyn BakkesMod = &Dummy;
    pub fn bakkesmod_init(id: u64) {
        let bm_wrapper = Box::new(BakkesModWrapper {
            id,
            notifier_callbacks: Mutex::new(Vec::new()),
            hook_callbacks: Mutex::new(Vec::new()),
            drawable_callbacks: Mutex::new(Vec::new()),
        });
        unsafe {
            BAKKESMOD = Box::leak(bm_wrapper);
        }
    }
    fn bakkesmod() -> &'static dyn BakkesMod {
        unsafe { BAKKESMOD }
    }
    type NotifierCallback = dyn FnMut(Vec<String>);
    type HookCallback = dyn FnMut();
    type HookWithCallerCallback<T> = dyn FnMut(Box<T>);
    type HookWithCallerCallbackInternal = dyn FnMut(usize, usize);
    type DrawableCallback = dyn FnMut(Canvas);
    type TimeoutCallback = dyn FnMut();
    type ExecuteCallback = dyn FnMut();
    pub fn register_notifier(name: &str, callback: Box<NotifierCallback>) {
        let callback = Box::new(callback);
        let addr = Box::into_raw(callback) as usize;
        let bm = bakkesmod();
        bm.add_notifier_callback(addr);
        let id = bm.id();
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        let c_callback = notifier_callback as usize;
        let user_data = addr;
        let c_desc = CString::new("").unwrap();
        let c_desc: *const c_char = c_desc.as_ptr();
        unsafe {
            RegisterNotifier(id, user_data, c_name, c_callback, c_desc, 0);
        }
    }
    extern "C" fn notifier_callback(addr: usize, params: *const *const c_char, len: u32) {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["callback called!"],
                        &match () {
                            () => [],
                        },
                    ),
                    lvl,
                    &(
                        "bakkesmod::bakkesmod",
                        "bakkesmod::bakkesmod",
                        "src\\bakkesmod.rs",
                        78u32,
                    ),
                );
            }
        };
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["user data: "],
                        &match (&addr,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                        &[::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 16u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        }],
                    ),
                    lvl,
                    &(
                        "bakkesmod::bakkesmod",
                        "bakkesmod::bakkesmod",
                        "src\\bakkesmod.rs",
                        79u32,
                    ),
                );
            }
        };
        if len <= 0 {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["callback called but len is <= 0 !"],
                            &match () {
                                () => [],
                            },
                        ),
                        lvl,
                        &(
                            "bakkesmod::bakkesmod",
                            "bakkesmod::bakkesmod",
                            "src\\bakkesmod.rs",
                            81u32,
                        ),
                    );
                }
            };
            return;
        }
        let params_ptr_ptr = params as *const *const c_char;
        if params_ptr_ptr.is_null() {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["ptr to params ptr is null!"],
                            &match () {
                                () => [],
                            },
                        ),
                        lvl,
                        &(
                            "bakkesmod::bakkesmod",
                            "bakkesmod::bakkesmod",
                            "src\\bakkesmod.rs",
                            84u32,
                        ),
                    );
                }
            };
            return;
        }
        let mut rust_params: Vec<String> = Vec::new();
        for i in 0..len {
            let params_ptr = unsafe { *(params_ptr_ptr.offset(i as isize)) as *const c_char };
            if params_ptr.is_null() {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["params ptr is null!"],
                                &match () {
                                    () => [],
                                },
                            ),
                            lvl,
                            &(
                                "bakkesmod::bakkesmod",
                                "bakkesmod::bakkesmod",
                                "src\\bakkesmod.rs",
                                90u32,
                            ),
                        );
                    }
                };
                return;
            }
            let params_c_str = unsafe { CStr::from_ptr(params_ptr) };
            match params_c_str.to_str() {
                Ok(s) => rust_params.push(String::from(s)),
                Err(_) => {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["params null"],
                                    &match () {
                                        () => [],
                                    },
                                ),
                                lvl,
                                &(
                                    "bakkesmod::bakkesmod",
                                    "bakkesmod::bakkesmod",
                                    "src\\bakkesmod.rs",
                                    95u32,
                                ),
                            );
                        }
                    };
                    return;
                }
            };
        }
        let mut closure = unsafe { Box::from_raw(addr as *mut Box<NotifierCallback>) };
        closure(rust_params);
        let _ = Box::into_raw(closure);
    }
    pub fn register_cvar(name: &str) {
        let id = bakkesmod().id();
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        let c_defval = CString::new("").unwrap();
        let c_defval: *const c_char = c_defval.as_ptr();
        let c_desc = CString::new("").unwrap();
        let c_desc: *const c_char = c_desc.as_ptr();
        unsafe {
            RegisterCvar(
                id, c_name, c_defval, c_desc, true, false, 0.0, false, 0.0, false,
            );
        }
    }
    pub fn hook_event(name: &str, callback: Box<HookCallback>) {
        let callback = Box::new(callback);
        let addr = Box::into_raw(callback) as usize;
        let bm = bakkesmod();
        bm.add_hook_callback(addr);
        let id = bm.id();
        let c_name = {
            let c_string = CString::new(name).unwrap();
            c_string.as_ptr() as *const c_char
        };
        let c_callback = hook_callback as usize;
        let user_data = addr;
        unsafe {
            HookEvent(id, user_data, c_name, c_callback);
        }
    }
    extern "C" fn hook_callback(addr: usize) {
        let mut closure = unsafe { Box::from_raw(addr as *mut Box<HookCallback>) };
        closure();
        let _ = Box::into_raw(closure);
    }
    pub fn hook_event_with_caller<T: Object + 'static>(
        name: &str,
        mut callback: Box<HookWithCallerCallback<T>>,
    ) {
        let wrapper_callback = Box::new(move |caller: usize, params: usize| {
            let obj_wrapper = T::new(caller);
            callback(Box::new(obj_wrapper));
        });
        hook_event_with_caller_internal(name, wrapper_callback);
    }
    fn hook_event_with_caller_internal(name: &str, callback: Box<HookWithCallerCallbackInternal>) {
        let callback = Box::new(callback);
        let addr = Box::into_raw(callback) as usize;
        let bm = bakkesmod();
        bm.add_hook_callback(addr);
        let id = bm.id();
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        let c_callback = hook_with_caller_callback as usize;
        let user_data = addr;
        unsafe {
            HookEventWithCaller(id, user_data, c_name, c_callback);
        }
    }
    extern "C" fn hook_with_caller_callback(addr: usize, caller: usize, params: usize) {
        let mut closure =
            unsafe { Box::from_raw(addr as *mut Box<HookWithCallerCallbackInternal>) };
        closure(caller, params);
        let _ = Box::into_raw(closure);
    }
    pub fn register_drawable(callback: Box<DrawableCallback>) {
        let callback = Box::new(callback);
        let addr = Box::into_raw(callback) as usize;
        let bm = bakkesmod();
        bm.add_drawable_callback(addr);
        let id = bm.id();
        let c_callback = drawable_callback as usize;
        let user_data = addr;
        unsafe {
            RegisterDrawable(id, user_data, c_callback);
        }
    }
    extern "C" fn drawable_callback(addr: usize, canvas: usize) {
        let mut closure = unsafe { Box::from_raw(addr as *mut Box<DrawableCallback>) };
        let canvas = Canvas::new(canvas);
        closure(canvas);
        let _ = Box::into_raw(closure);
    }
    pub fn log(text: &str) {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["trying to log \"", "\""],
                        &match (&text,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    lvl,
                    &(
                        "bakkesmod::bakkesmod",
                        "bakkesmod::bakkesmod",
                        "src\\bakkesmod.rs",
                        201u32,
                    ),
                );
            }
        };
        let id = bakkesmod().id();
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe {
            LogConsole(id, c_text);
        }
    }
    pub fn get_local_car() -> Option<CarWrapper> {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["calling get_local_car()"],
                        &match () {
                            () => [],
                        },
                    ),
                    lvl,
                    &(
                        "bakkesmod::bakkesmod",
                        "bakkesmod::bakkesmod",
                        "src\\bakkesmod.rs",
                        209u32,
                    ),
                );
            }
        };
        let p_car = unsafe { GetLocalCar() };
        match p_car {
            0 => None,
            _ => Some(CarWrapper(p_car)),
        }
    }
    struct Dummy;
    impl BakkesMod for Dummy {
        fn id(&self) -> u64 {
            0
        }
        fn add_notifier_callback(&self, addr: usize) {}
        fn add_hook_callback(&self, addr: usize) {}
        fn add_drawable_callback(&self, addr: usize) {}
    }
    struct BakkesModWrapper {
        pub id: u64,
        pub notifier_callbacks: Mutex<Vec<usize>>,
        pub hook_callbacks: Mutex<Vec<usize>>,
        pub drawable_callbacks: Mutex<Vec<usize>>,
    }
    impl BakkesMod for BakkesModWrapper {
        fn id(&self) -> u64 {
            self.id
        }
        fn add_notifier_callback(&self, addr: usize) {
            let mut callbacks = self.notifier_callbacks.lock().unwrap();
            callbacks.push(addr);
        }
        fn add_hook_callback(&self, addr: usize) {
            let mut callbacks = self.hook_callbacks.lock().unwrap();
            callbacks.push(addr);
        }
        fn add_drawable_callback(&self, addr: usize) {
            let mut callbacks = self.drawable_callbacks.lock().unwrap();
            callbacks.push(addr);
        }
    }
    extern "C" {
        fn LogConsole(id: u64, text: *const c_char);
        fn RegisterNotifier(
            id: u64,
            user_data: usize,
            cvar: *const c_char,
            callback: usize,
            description: *const c_char,
            permissions: u8,
        );
        fn RegisterCvar(
            id: u64,
            cvar: *const c_char,
            default_value: *const c_char,
            desc: *const c_char,
            searchable: bool,
            has_min: bool,
            min: f32,
            has_max: bool,
            max: f32,
            save_to_cfg: bool,
        );
        fn HookEvent(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
        fn HookEventPost(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
        fn UnhookEvent(id: u64, event_name: *const c_char);
        fn HookEventWithCaller(
            id: u64,
            user_data: usize,
            event_name: *const c_char,
            callback: usize,
        );
        fn HookEventWithCallerPost(
            id: u64,
            user_data: usize,
            event_name: *const c_char,
            callback: usize,
        );
        fn UnhookEventPost(id: u64, event_name: *const c_char);
        fn RegisterDrawable(id: u64, user_data: usize, callback: usize);
        fn UnregisterDrawables(id: u64);
        fn SetTimeout(id: u64, user_data: usize, callback: usize, time: f32);
        fn Execute(id: u64, user_data: usize, callback: usize);
        fn GetLocalCar() -> usize;
    }
}
pub use bakkesmod::*;
pub mod wrappers {
    #![allow(unused)]
    use std::fmt;
    use std::ops;
    use std::convert::From;
    use std::marker::PhantomData;
    pub trait Object {
        fn new(addr: usize) -> Self;
        fn addr(&self) -> usize;
    }
    pub trait Actor: Object {
        fn get_location(&self) -> Vector3 {
            unsafe { Actor_GetLocation(self.addr()) }
        }
        fn set_location(&self, new_loc: Vector3) {
            unsafe {
                Actor_SetLocation(self.addr(), new_loc);
            }
        }
    }
    pub trait Car: Actor {
        fn demolish(&self);
        fn get_last_wheels_hit_ball_time(&self) -> f32;
        fn get_vehicle_sim(&self) -> VehicleSimWrapper;
    }
    pub trait Wheel: Object {
        fn get_spin_speed(&self) -> f32;
    }
    pub struct WheelWrapper(pub usize);
    impl Object for WheelWrapper {
        fn new(addr: usize) -> Self {
            Self(addr)
        }
        fn addr(&self) -> usize {
            self.0
        }
    }
    impl Wheel for WheelWrapper {
        fn get_spin_speed(&self) -> f32 {
            unsafe { Wheel_Get_SpinSpeed(self.addr()) }
        }
    }
    pub trait VehicleSim: Object {
        fn get_wheels(&self) -> RLArray<WheelWrapper>;
    }
    pub struct VehicleSimWrapper(pub usize);
    impl Object for VehicleSimWrapper {
        fn new(addr: usize) -> Self {
            Self(addr)
        }
        fn addr(&self) -> usize {
            self.0
        }
    }
    impl VehicleSim for VehicleSimWrapper {
        fn get_wheels(&self) -> RLArray<WheelWrapper> {
            unsafe {
                let mut array = RLArrayRaw::new();
                let ptr: *mut RLArrayRaw = &mut array as *mut RLArrayRaw;
                VehicleSim_Get_Wheels(self.addr(), ptr);
                RLArray::from_raw(array)
            }
        }
    }
    impl Object for CarWrapper {
        fn new(addr: usize) -> Self {
            Self(addr)
        }
        fn addr(&self) -> usize {
            self.0
        }
    }
    impl Actor for CarWrapper {}
    pub struct CarWrapper(pub usize);
    impl Car for CarWrapper {
        fn demolish(&self) {
            unsafe {
                Car_Demolish(self.addr());
            }
        }
        fn get_last_wheels_hit_ball_time(&self) -> f32 {
            unsafe { Car_Get_LastWheelsHitBallTime(self.addr()) }
        }
        fn get_vehicle_sim(&self) -> VehicleSimWrapper {
            unsafe { VehicleSimWrapper(Car_Get_VehicleSim(self.addr())) }
        }
    }
    #[repr(C)]
    struct RLArrayRaw {
        data: usize,
        count: u32,
        max: u32,
    }
    impl RLArrayRaw {
        fn new() -> RLArrayRaw {
            RLArrayRaw {
                data: 0,
                count: 0,
                max: 0,
            }
        }
    }
    #[repr(C)]
    pub struct RLArray<T: Object> {
        pub data: *mut usize,
        count: u32,
        max: u32,
        phantom: PhantomData<T>,
    }
    impl<T: Object> RLArray<T> {
        fn from_raw(raw: RLArrayRaw) -> RLArray<T> {
            RLArray {
                data: raw.data as *mut usize,
                count: 0,
                max: 0,
                phantom: PhantomData,
            }
        }
        pub fn get(&self, index: isize) -> T {
            unsafe {
                let ptr = self.data.offset(index);
                T::new(*ptr)
            }
        }
    }
    extern "C" {
        fn Car_Demolish(p_car: usize);
        fn Actor_GetLocation(p_actor: usize) -> Vector3;
        fn Actor_SetLocation(p_actor: usize, new_loc: Vector3);
        fn Car_Get_LastWheelsHitBallTime(p_car: usize) -> f32;
        fn Car_Get_VehicleSim(p_car: usize) -> usize;
        fn VehicleSim_Get_Wheels(p_vehicle_sim: usize, result: *mut RLArrayRaw);
        fn Wheel_Get_SpinSpeed(p_wheel: usize) -> f32;
    }
    #[repr(C)]
    pub struct Vector3 {
        x: f32,
        y: f32,
        z: f32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Vector3 {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Vector3 {
        #[inline]
        fn clone(&self) -> Vector3 {
            {
                let _: ::core::clone::AssertParamIsClone<f32>;
                let _: ::core::clone::AssertParamIsClone<f32>;
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
    }
    impl Vector3 {
        pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
            Vector3 { x, y, z }
        }
    }
    impl fmt::Display for Vector3 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["(", ", ", ", ", ")"],
                &match (&self.x, &self.y, &self.z) {
                    (arg0, arg1, arg2) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                    ],
                },
            ))
        }
    }
    impl ops::Add for Vector3 {
        type Output = Vector3;
        fn add(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
    #[repr(C)]
    pub struct Vector2 {
        x: i32,
        y: i32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Vector2 {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Vector2 {
        #[inline]
        fn clone(&self) -> Vector2 {
            {
                let _: ::core::clone::AssertParamIsClone<i32>;
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
    }
    impl Vector2 {
        pub fn new(x: i32, y: i32) -> Vector2 {
            Vector2 { x, y }
        }
    }
    impl ops::Add for Vector2 {
        type Output = Vector2;
        fn add(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    pub struct Canvas(usize);
    impl Canvas {
        pub fn new(addr: usize) -> Canvas {
            Canvas(addr)
        }
        pub fn draw_line(&self, start: Vector2, end: Vector2) {
            unsafe {
                Canvas_DrawLine(self.0, start, end);
            }
        }
    }
    extern "C" {
        fn Canvas_DrawLine(p_canvas: usize, start: Vector2, end: Vector2);
    }
}
pub use bakkesmod_macros::plugin_init;
