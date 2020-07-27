#![allow(unused)]

#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
use bakkesmod::wrappers::*;
use bakkesmod::{vec2, vec3};
use bakkesmod::{self, log_console, plugin_init};

#[plugin_init]
pub fn on_load() {
    bakkesmod::register_notifier("rust_notifier", Box::new(move |params: Vec<String>| {
        log_console!("this is the callback for rust_notifier!");
        log_console!("params = {:x?}", params);
        log_console!("unhooking GameViewportClient.Tick...");
        bakkesmod::unhook_event("Function Engine.GameViewportClient.Tick");
    }));

    bakkesmod::register_notifier("rust_demolish", Box::new(move |_: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => car.demolish(),
            None => log_console!("Car is NULL")
        };
    }));

    bakkesmod::register_notifier("rust_set_loc", Box::new(move |_: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => {
                let origin = Vector3::new(0.0, 0.0, 0.0);
                let new_loc = origin + Vector3::new(200.0, 1000.0, 50.0);
                car.set_location(new_loc);
            }
            None => log_console!("Car is NULL")
        };
    }));

    bakkesmod::register_cvar("rust_cvar");

    let counter_base = Rc::new(RefCell::new(0));
    let counter_ref = Rc::clone(&counter_base);

    // bakkesmod::hook_event("Function Engine.GameViewportClient.Tick", Box::new(move || {
    //     let mut counter = counter_ref.borrow_mut();
    //     *counter += 1;
    //     if (*counter % 240) == 0 {
    //         // log_console!("viewport client tick");

    //         match bakkesmod::get_local_car() {
    //             Some(car) => {
    //                 let location = car.get_location();
    //                 log_console!("{}", location);

    //                 let vehicle_sim = car.get_vehicle_sim();
    //                 info!("got vehicle sim: {:x?}", vehicle_sim.addr());
    //                 let wheels = vehicle_sim.get_wheels();
    //                 info!("got wheels: {:x?}", wheels.data);
    //                 let wheel0 = wheels.get(0);
    //                 info!("got wheel 0: {:x?}", wheel0.addr());
    //                 log_console!("wheel 0 spin speed: {}", wheel0.get_spin_speed());
    //                 let wheel3 = wheels.get(3);
    //                 info!("got wheel 3: {:x?}", wheel3.addr());
    //                 log_console!("wheel 3 spin speed: {}", wheel3.get_spin_speed());
    //             }
    //             None => log_console!("Car is NULL")
    //         };
    //     }
    // }));

    bakkesmod::register_drawable(Box::new(move |canvas: Canvas| {
        let top_left = vec2!(100, 100);
        let width = vec2!(250, 0);
        let height = vec2!(0, 150);
        canvas.draw_line(top_left, top_left + width);
        canvas.draw_line(top_left + width, top_left + width + height);
        canvas.draw_line(top_left, top_left + height);
        canvas.draw_line(top_left + height, top_left + width + height);
    }));

    bakkesmod::hook_event_with_caller_post(
        "Function TAGame.Car_TA.ApplyBallImpactForces",
        Box::new(move |car: Box<CarWrapper>| {
            log_console!("Ball hit!");
            log_console!("car location: {}", car.get_location());
        })
    );

    bakkesmod::register_notifier("rust_set_timer", Box::new(move |params: Vec<String>| {
        if params.len() < 2 {
            log_console!("not enough parameters!");
        } else {
            let time_param = &params[1];
            let time: f32 = match time_param.parse::<f32>() {
                Ok(secs) => secs,
                Err(_) => { log_console!("invalid input!"); return; }
            };
            bakkesmod::set_timeout(Box::new(move || {
                log_console!("{} secs have passed!", time);
            }), time);
        }
    }));

    // bakkesmod::hook_event_with_caller(
    //     "Function TAGame.Car_TA.SetVehicleInput",
    //     Box::new(move |car: Box<CarWrapper>| {
    //         car.demolish();
    //     }));
}