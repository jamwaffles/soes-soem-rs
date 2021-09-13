use soes_sys::*;

#[no_mangle]
extern "C" fn application_hook() {
    println!("application_hook")
}

#[no_mangle]
extern "C" fn cb_get_inputs() {
    println!("cb_get_inputs")
}

#[no_mangle]
extern "C" fn cb_set_outputs() {
    println!("cb_set_outputs")
}

#[no_mangle]
static mut SDOobjects: [_objectlist; 0] = [];

fn main() {
    let mut config: esc_cfg = esc_cfg {
        user_arg: "/dev/lan9252".as_ptr() as _,
        use_interrupt: 0,
        watchdog_cnt: 150,
        set_defaults_hook: None,
        pre_state_change_hook: None,
        post_state_change_hook: None,
        application_hook: Some(application_hook),
        safeoutput_override: None,
        pre_object_download_hook: None,
        post_object_download_hook: None,
        rxpdo_override: None,
        txpdo_override: None,
        esc_hw_interrupt_enable: None,
        esc_hw_interrupt_disable: None,
        esc_hw_eep_handler: None,
        esc_check_dc_handler: None,
        pre_object_upload_hook: None,
        post_object_upload_hook: None,
        skip_default_initialization: false,
    };

    println!("Hello Main");

    let config = &mut config as *mut _ as *mut esc_cfg;

    unsafe { ecat_slv_init(config) };

    loop {
        unsafe { ecat_slv() };
    }
}

//

// #include <stdio.h>
// #include "ecat_slv.h"
// #include "utypes.h"

// /* Application variables */
// _Objects    Obj;

// void cb_get_inputs (void)
// {
// }

// void cb_set_outputs (void)
// {
// }

// int main_run (void * arg)
// {
//    static esc_cfg_t config =
//    {
//       .user_arg = "/dev/lan9252",
//       .use_interrupt = 0,
//       .watchdog_cnt = 150,
//       .set_defaults_hook = NULL,
//       .pre_state_change_hook = NULL,
//       .post_state_change_hook = NULL,
//       .application_hook = NULL,
//       .safeoutput_override = NULL,
//       .pre_object_download_hook = NULL,
//       .post_object_download_hook = NULL,
//       .rxpdo_override = NULL,
//       .txpdo_override = NULL,
//       .esc_hw_interrupt_enable = NULL,
//       .esc_hw_interrupt_disable = NULL,
//       .esc_hw_eep_handler = NULL,
//       .esc_check_dc_handler = NULL,
//    };

//    printf ("Hello Main\n");
//    ecat_slv_init (&config);

//    while (1)
//    {
//       ecat_slv();
//    }

//    return 0;
// }

// int main (void)
// {
//    printf ("Hello Main\n");
//    main_run (NULL);
//    return 0;
// }
