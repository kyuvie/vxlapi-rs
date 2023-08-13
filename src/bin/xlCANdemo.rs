use std::env;
use std::ffi::*;
use std::ptr;
use std::str;
use std::sync::Mutex;
use windows::Win32::Foundation::HANDLE;

use once_cell::sync::Lazy;

use vxlapi_rs::vxlapi::*;

use windows::Win32::System::Threading::CreateThread;
use windows::Win32::System::Threading::Sleep;
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::Win32::System::Threading::THREAD_CREATION_FLAGS;

use win32console::console::WinConsole;
use win32console::input::InputRecord::KeyEvent;


/////////////////////////////////////////////////////////////////////////////
// globals
const RX_QUEUE_SIZE: u32 = 4096u32; // internal driver queue size in CAN events
const RX_QUEUE_SIZE_FD: u32 = 16384u32; // driver queue size for CAN-FD Rx events

static g_xlPortHandle: Lazy<Mutex<XLportHandle>> = Lazy::new(|| Mutex::new(XL_INVALID_PORTHANDLE)); //  Global porthandle (we use only one!)
static g_xlDrvConfig: Lazy<Mutex<XLdriverConfig>> = Lazy::new(|| Mutex::new(XLdriverConfig::default())); // Contains the actual hardware configuration
static g_xlChannelMask: Lazy<Mutex<XLaccess>> = Lazy::new(|| Mutex::new(0)); //  Global channelmask (includes all founded channels)
static g_xlPermissionMask: Lazy<Mutex<XLaccess>> = Lazy::new(|| Mutex::new(0)); // Global permissionmask (includes all founded channels)
static g_BaudRate: Lazy<Mutex<c_uint>> = Lazy::new(|| Mutex::new(500000 as c_uint)); // Default baudrate
static mut g_silent: c_int = 0; // flag to visualize the message events (on/off)
static mut g_TimerRate: c_uint = 0u32;
static g_canFdModeNoIso: Lazy<Mutex<c_uint>> = Lazy::new(|| Mutex::new(0 as c_uint));
static g_canFdSupport: Lazy<Mutex<c_uint>> = Lazy::new(|| Mutex::new(0 as c_uint));

// thread variables
static g_hMsgEvent: Lazy<Mutex<HANDLE>> = Lazy::new(|| Mutex::new(HANDLE(0isize))); // notification handle for the receive queue
static g_hRXThread: Lazy<Mutex<HANDLE>> = Lazy::new(|| Mutex::new(HANDLE(0isize))); // thread handle (RX)
static g_hTXThread: Lazy<Mutex<HANDLE>> = Lazy::new(|| Mutex::new(HANDLE(0isize))); // thread handle (RX)

static g_RXThreadRun: Lazy<Mutex<c_int>> = Lazy::new(|| Mutex::new(0 as c_int)); // flag to start/stop the RX thread

static g_TXThreadRun: Lazy<Mutex<c_int>> = Lazy::new(|| Mutex::new(0 as c_int)); // flag to start/stop the TX thread (for the transmission burst)
static g_RXCANThreadRun: Lazy<Mutex<c_int>> = Lazy::new(|| Mutex::new(0 as c_int)); // flag to start/stop the RX thread
static g_TXThreadCanId: Lazy<Mutex<c_uint>> = Lazy::new(|| Mutex::new(0 as c_uint)); // CAN-ID the TX thread transmits under
static g_TXThreadTxMask: Lazy<Mutex<XLaccess>> = Lazy::new(|| Mutex::new(0 as XLaccess)); // channel mask the TX thread uses for transmitting

fn main() {
    let mut xlStatus: XLstatus;
    let mut xlChanMaskTx: XLaccess = 0;

    let mut stop: c_uint = 0;
    let mut activated: c_uint = 0;
    let mut xlChanIndex: c_uint = 0;

    let mut outputMode: u8 = XL_OUTPUT_MODE_NORMAL;

    print!("----------------------------------------------------------\n");
    print!("- xlCANdemo - Test Application for XL Family Driver API  -\n");
    // fixme: add date

    // ------------------------------------
    // commandline may specify application
    // name and baudrate
    // ------------------------------------
    let args: Vec<String> = env::args().collect();
    if args.len() != 3usize {
        print!("Usage: cargo run --bin xlCANdemo <ApplicationName> <Identifier>\n\n");
        return;
    }
    let appName = &args[1];
    println!("AppName = {}", appName);
    let mut txID = u32::from_str_radix(&args[2], 16).expect("arg[2] must be hex i32 for Tx ID");
    println!("TX ID = 0x{:X}", txID);

    // ------------------------------------
    // initialize the driver structures
    // for the application
    // ------------------------------------
    xlStatus = demoInitDriver(&mut xlChanMaskTx, &mut xlChanIndex, appName);

    print!("- Init             : {}\n", xl_get_error_string(xlStatus));

    if XL_SUCCESS == xlStatus {
        // ------------------------------------
        // create the RX thread to read the
        // messages
        // ------------------------------------
        xlStatus = demoCreateRxThread();
        print!("- Create RX thread : {}\n", xl_get_error_string(xlStatus));
    }

    if XL_SUCCESS == xlStatus {
        // ------------------------------------
        // go with all selected channels on bus
        // ------------------------------------
        xlStatus = xl_activate_channel(
            *g_xlPortHandle.lock().unwrap(),
            *g_xlChannelMask.lock().unwrap(),
            XL_BUS_TYPE_CAN,
            XL_ACTIVATE_RESET_CLOCK,
        );
        print!(
            "- ActivateChannel  : CM=0x{:X}, {}\n",
            *g_xlChannelMask.lock().unwrap(),
            xl_get_error_string(xlStatus)
        );
        if xlStatus == XL_SUCCESS {
            activated = 1;
        }
    }

    print!(
        "\n: Press <h> for help - actual channel Ch={}, CM=0x{:02X}\n",
        xlChanIndex, xlChanMaskTx
    );

    // ------------------------------------
    // parse the key - commands
    // ------------------------------------
    while stop == 0 {
        let c: char;

        let ir = WinConsole::input().read_input_n(1).unwrap()[0];

        if let KeyEvent(keyEventRecord) = ir {
            if keyEventRecord.key_down {
                match keyEventRecord.u_char {
                    'v' => {
                        if unsafe { g_silent != 0 } {
                            unsafe {
                                g_silent = 0;
                                print!("- screen on\n");
                            }
                        } else {
                            unsafe {
                                g_silent = 1;
                                print!("- screen off\n");
                            }
                        }
                    }
                    't' => {
                        // transmit a message
                        demoTransmit(txID, xlChanMaskTx);
                    }
                    'b' => {
                        // transmit message burst
                        if *g_TXThreadRun.lock().unwrap() != 0i32 {
                            demoStopTransmitBurst();
                        } else {
                            demoTransmitBurst(txID, xlChanMaskTx);
                        }
                    }
                    'm' => {
                        // transmit a remote message
                        demoTransmitRemote(txID, xlChanMaskTx);
                    }
                    '-' => {
                        // channel selection
                        if xlChanIndex == 0 {
                            xlChanIndex = g_xlDrvConfig.lock().unwrap().channelCount;
                        }
                        xlChanIndex -= 1;

                        xlChanMaskTx =
                            g_xlDrvConfig.lock().unwrap().channel[xlChanIndex as usize].channelMask;

                        let channelIndex;

                        {
                            channelIndex = g_xlDrvConfig.lock().unwrap().channel
                                [xlChanIndex as usize]
                                .channelIndex;
                        }

                        print!(
                            "- TX Channel set to channel: {:02}, {} CM(0x{:x})\n",
                            channelIndex,
                            unsafe {
                                CStr::from_ptr(
                                    &g_xlDrvConfig.lock().unwrap().channel[xlChanIndex as usize]
                                        .name as *const i8,
                                )
                                .to_str()
                                .unwrap()
                            },
                            xlChanMaskTx
                        );
                    }
                    '+' => {
                        // channel selection
                        xlChanIndex += 1;
                        if xlChanIndex >= g_xlDrvConfig.lock().unwrap().channelCount {
                            xlChanIndex = 0;
                        }

                        xlChanMaskTx =
                            g_xlDrvConfig.lock().unwrap().channel[xlChanIndex as usize].channelMask;

                        let channelIndex;

                        {
                            channelIndex = g_xlDrvConfig.lock().unwrap().channel
                                [xlChanIndex as usize]
                                .channelIndex;
                        }

                        print!(
                            "- TX Channel set to channel: {:02}, {} CM(0x{:x})\n",
                            channelIndex,
                            unsafe {
                                CStr::from_ptr(
                                    &g_xlDrvConfig.lock().unwrap().channel[xlChanIndex as usize]
                                        .name as *const i8,
                                )
                                .to_str()
                                .unwrap()
                            },
                            xlChanMaskTx
                        );
                    }
                    'x' => {
                        txID ^= XL_CAN_EXT_MSG_ID; // toggle ext/std
                        print!("- Id set to 0x{:08X}\n", txID);
                    }
                    'I' => {
                        // id selection
                        if txID & XL_CAN_EXT_MSG_ID != 0 {
                            txID = (txID - 1) | XL_CAN_EXT_MSG_ID;
                        } else if (txID == 0) {
                            txID = 0x7FF;
                        } else {
                            txID -= 1;
                        }
                        print!("- Id set to 0x{:08X}\n", txID);
                    }
                    'i' => {
                        if (txID & XL_CAN_EXT_MSG_ID != 0) {
                            txID = (txID + 1) | XL_CAN_EXT_MSG_ID;
                        } else if txID == 0x7FF {
                            txID = 0;
                        } else {
                            txID += 1;
                        }
                        print!("- Id set to 0x{:08X}\n", txID);
                    }
                    'g' => {
                        xlStatus = xl_can_request_chip_state(
                            *g_xlPortHandle.lock().unwrap(),
                            *g_xlChannelMask.lock().unwrap(),
                        );
                        print!(
                            "- RequestChipState : CM(0x{:x}), {}\n",
                            *g_xlChannelMask.lock().unwrap(),
                            xl_get_error_string(xlStatus)
                        );
                    }
                    'a' => {
                        if unsafe { g_TimerRate != 0u32 } {
                            unsafe {
                                g_TimerRate = 0;
                            }
                        } else {
                            unsafe {
                                g_TimerRate = 20000;
                            }
                        }

                        xlStatus = xl_set_timer_rate(*g_xlPortHandle.lock().unwrap(), unsafe {
                            g_TimerRate
                        });
                        print!(
                            "- SetTimerRate     : {}, {}\n",
                            unsafe { g_TimerRate },
                            xl_get_error_string(xlStatus)
                        );
                    }
                    'o' => {
                        if outputMode == XL_OUTPUT_MODE_NORMAL {
                            outputMode = XL_OUTPUT_MODE_SILENT;
                            demoSetOutput(outputMode, "SILENT", xlChanMaskTx);
                        } else {
                            outputMode = XL_OUTPUT_MODE_NORMAL;
                            demoSetOutput(outputMode, "NORMAL", xlChanMaskTx);
                        }
                    }
                    'r' => {
                        xlStatus = xl_reset_clock(*g_xlPortHandle.lock().unwrap());
                        print!("- ResetClock       : {}\n", xl_get_error_string(xlStatus));
                    }
                    's' => {
                        if activated != 0u32 {
                            activated = 0;
                        } else {
                            activated = 1;
                        }
                        demoStartStop(activated as c_int);
                    }
                    'p' => {
                        demoPrintConfig();
                    }
                    'y' => {
                        xlStatus =
                            xl_generate_sync_pulse(*g_xlPortHandle.lock().unwrap(), xlChanMaskTx);
                        print!(
                            "- xlGenerateSyncPulse : CM(0x{:x}), {}\n",
                            xlChanMaskTx,
                            xl_get_error_string(xlStatus)
                        );
                    }
                    '\x1b' => {
                        // end application
                        stop = 1;
                    }
                    'h' => {
                        demoHelp();
                    }
                    '\x00' => {
                        // nothing to do
                    }
                    _ => {
                        break;
                    } // end switch
                }
            }
        }
    } // end while

    if (XL_SUCCESS != xlStatus) && (activated != 0u32) {
        xlStatus = xl_deactivate_channel(
            *g_xlPortHandle.lock().unwrap(),
            *g_xlChannelMask.lock().unwrap(),
        );
        print!(
            "- DeactivateChannel: CM(0x{:x}), {}\n",
            *g_xlChannelMask.lock().unwrap(),
            xl_get_error_string(xlStatus)
        );
    }
    demoCleanUp();

    return;
}

////////////////////////////////////////////////////////////////////////////
// demoHelp()
// shows the program functionality
//
////////////////////////////////////////////////////////////////////////////

fn demoHelp() {
    print!("\n----------------------------------------------------------\n");
    print!("-                   xlCANdemo - HELP                     -\n");
    print!("----------------------------------------------------------\n");
    print!("- Keyboard commands:                                     -\n");
    print!("- 't'      Transmit a message                            -\n");
    print!("- 'b'      Transmit a message burst (toggle)             -\n");
    print!("- 'm'      Transmit a remote message                     -\n");
    print!("- 'g'      Request chipstate                             -\n");
    print!("- 's'      Start/Stop                                    -\n");
    print!("- 'r'      Reset clock                                   -\n");
    print!("- '+'      Select channel      (up)                      -\n");
    print!("- '-'      Select channel      (down)                    -\n");
    print!("- 'i'      Select transmit Id  (up)                      -\n");
    print!("- 'I'      Select transmit Id  (down)                    -\n");
    print!("- 'x'      Toggle extended/standard Id                   -\n");
    print!("- 'o'      Toggle output mode                            -\n");
    print!("- 'a'      Toggle timer                                  -\n");
    print!("- 'v'      Toggle logging to screen                      -\n");
    print!("- 'p'      Show hardware configuration                   -\n");
    print!("- 'y'      Trigger HW-Sync pulse                         -\n");
    print!("- 'h'      Help                                          -\n");
    print!("- 'ESC'    Exit                                          -\n");
    print!("----------------------------------------------------------\n");
    print!("- 'PH'->PortHandle; 'CM'->ChannelMask; 'PM'->Permission  -\n");
    print!("----------------------------------------------------------\n\n");
}

////////////////////////////////////////////////////////////////////////////
// demoPrintConfig()
// shows the actual hardware configuration
//
////////////////////////////////////////////////////////////////////////////

fn demoPrintConfig() {
    let channelCount: usize = g_xlDrvConfig.lock().unwrap().channelCount as usize;

    print!("----------------------------------------------------------\n");
    print!(
        "- {:02} channels       Hardware Configuration               -\n",
        channelCount
    );
    print!("----------------------------------------------------------\n");

    for i in 0usize..channelCount {
        let channelMask = g_xlDrvConfig.lock().unwrap().channel[i].channelMask;

        print!(
            "- Ch:{:02}, CM:0x{:03x},",
            g_xlDrvConfig.lock().unwrap().channel[i].channelIndex,
            channelMask
        );

        print!(
            " {:23},",
            unsafe {
                CStr::from_ptr(&(g_xlDrvConfig.lock().unwrap().channel[i].name) as *const i8)
            }
            .to_str()
            .unwrap()
        );

        if g_xlDrvConfig.lock().unwrap().channel[i].transceiverType != XL_TRANSCEIVER_TYPE_NONE {
            print!(
                "{:13} -\n",
                unsafe {
                    CStr::from_ptr(
                        &g_xlDrvConfig.lock().unwrap().channel[i].transceiverName as *const i8,
                    )
                }
                .to_str()
                .unwrap()
            );
        } else {
            print!("    no Cab!   -\n");
        }
    }

    print!("----------------------------------------------------------\n\n");
}

////////////////////////////////////////////////////////////////////////////
// demoTransmit
// transmit a CAN message (depending on an ID, channel)
//
////////////////////////////////////////////////////////////////////////////
static mut cnt: c_uint = 0;

fn demoTransmit(txID: c_uint, xlChanMaskTx: XLaccess) -> XLstatus {
    let xlStatus: XLstatus;
    let messageCount: c_uint = 1;

    if { *g_canFdSupport.lock().unwrap() } != 0 {
        let fl: [c_uint; 3] = [
            0, // CAN (no FD)
            XL_CAN_TXMSG_FLAG_EDL,
            XL_CAN_TXMSG_FLAG_EDL | XL_CAN_TXMSG_FLAG_BRS,
        ];

        let mut canTxEvt: XLcanTxEvent = Default::default();
        let mut cntSent: c_uint = 0u32;

        canTxEvt.tag = XL_CAN_EV_TAG_TX_MSG;

        canTxEvt.tagData.canMsg.canId = txID;
        canTxEvt.tagData.canMsg.msgFlags = fl[unsafe { cnt } as usize % fl.len()];
        canTxEvt.tagData.canMsg.dlc = 8;

        // if EDL is set, demonstrate transmit with DLC=15 (64 bytes)
        if (unsafe { canTxEvt.tagData.canMsg.msgFlags } & XL_CAN_TXMSG_FLAG_EDL) != 0 {
            canTxEvt.tagData.canMsg.dlc = 15;
        }

        unsafe { cnt += 1 };

        for i in 1..XL_CAN_MAX_DATA_LEN {
            unsafe { canTxEvt.tagData.canMsg.data[i] = (i - 1) as c_uchar };
        }
        unsafe { canTxEvt.tagData.canMsg.data[0] = cnt as c_uchar };
        xlStatus = xl_can_transmit_ex(
            *g_xlPortHandle.lock().unwrap(),
            xlChanMaskTx,
            messageCount,
            &mut cntSent,
            &canTxEvt,
        );
    } else {
        let mut xlEvent: XLevent = Default::default();

        xlEvent.tag = XL_TRANSMIT_MSG;
        xlEvent.tagData.msg.id = txID;
        xlEvent.tagData.msg.dlc = 8;
        xlEvent.tagData.msg.flags = 0;
        unsafe {
            xlEvent.tagData.msg.data[0] += 1;
            xlEvent.tagData.msg.data[1] = 2;
            xlEvent.tagData.msg.data[2] = 3;
            xlEvent.tagData.msg.data[3] = 4;
            xlEvent.tagData.msg.data[4] = 5;
            xlEvent.tagData.msg.data[5] = 6;
            xlEvent.tagData.msg.data[6] = 7;
            xlEvent.tagData.msg.data[7] = 8;
        }

        xlStatus = xl_can_transmit(
            *g_xlPortHandle.lock().unwrap(),
            xlChanMaskTx,
            &messageCount,
            ptr::addr_of_mut!(xlEvent) as *const c_void,
        );
    }

    print!(
        "- Transmit         : CM(0x{:x}), {}\n",
        xlChanMaskTx,
        xl_get_error_string(xlStatus)
    );

    return xlStatus;
}

////////////////////////////////////////////////////////////////////////////
// demoStopTransmitBurst
// Stop the TX thread if it is running.
//
////////////////////////////////////////////////////////////////////////////

fn demoStopTransmitBurst() {
    if g_hTXThread.lock().unwrap().0 != 0isize {
        *g_TXThreadRun.lock().unwrap() = 0i32;
        unsafe { WaitForSingleObject(*g_hTXThread.lock().unwrap(), 10) };
        g_hTXThread.lock().unwrap().0 = 0isize;
    }
}

////////////////////////////////////////////////////////////////////////////
// demoTransmitBurst
// transmit a message burst (also depending on an IC, channel).
//
////////////////////////////////////////////////////////////////////////////

fn demoTransmitBurst(txID: c_uint, xlChanMaskTx: XLaccess) {
    // first collect old TX-Thread
    demoStopTransmitBurst();

    print!("- print txID: {}\n", txID);
    *g_TXThreadCanId.lock().unwrap() = txID;
    *g_TXThreadTxMask.lock().unwrap() = xlChanMaskTx;
    *g_TXThreadRun.lock().unwrap() = 1;
    *g_hTXThread.lock().unwrap() = unsafe {
        CreateThread(
            Option::None,
            0x1000,
            Option::Some(TxThread),
            Option::None,
            THREAD_CREATION_FLAGS(0),
            Option::None,
        )
    }
    .unwrap();
}

////////////////////////////////////////////////////////////////////////////
// demoTransmitRemote
// transmit a remote frame
//
////////////////////////////////////////////////////////////////////////////
fn demoTransmitRemote(txID: c_uint, xlChanMaskTx: XLaccess) -> XLstatus {
    let xlStatus: XLstatus;
    let messageCount: c_uint = 1u32;

    if *g_canFdSupport.lock().unwrap() != 0 {
        let mut canTxEvt: XLcanTxEvent = Default::default();
        let mut cntSent: c_uint = Default::default();

        canTxEvt.tag = XL_CAN_EV_TAG_TX_MSG;

        canTxEvt.tagData.canMsg.canId = txID;
        canTxEvt.tagData.canMsg.msgFlags = XL_CAN_TXMSG_FLAG_RTR;
        canTxEvt.tagData.canMsg.dlc = 8;

        xlStatus = xl_can_transmit_ex(
            *g_xlPortHandle.lock().unwrap(),
            xlChanMaskTx,
            messageCount,
            &mut cntSent,
            &canTxEvt,
        );
    } else {
        let mut xlEvent: XLevent = Default::default();

        xlEvent.tag = XL_TRANSMIT_MSG;
        xlEvent.tagData.msg.id = txID;
        xlEvent.tagData.msg.flags = XL_CAN_MSG_FLAG_REMOTE_FRAME;
        xlEvent.tagData.msg.dlc = 8;

        xlStatus = xl_can_transmit(
            *g_xlPortHandle.lock().unwrap(),
            xlChanMaskTx,
            &messageCount,
            ptr::addr_of_mut!(xlEvent) as *const c_void,
        );
    }

    print!(
        "- Transmit REMOTE  : CM(0x{:x}), {}\n",
        *g_xlChannelMask.lock().unwrap(),
        xl_get_error_string(xlStatus)
    );

    return XL_SUCCESS;
}

////////////////////////////////////////////////////////////////////////////
// demoStartStop
// toggle the channel activate/deactivate
//
////////////////////////////////////////////////////////////////////////////

fn demoStartStop(activated: c_int) -> XLstatus {
    let xlStatus: XLstatus;

    if activated != 0i32 {
        xlStatus = xl_activate_channel(
            *g_xlPortHandle.lock().unwrap(),
            *g_xlChannelMask.lock().unwrap(),
            XL_BUS_TYPE_CAN,
            XL_ACTIVATE_RESET_CLOCK,
        );
        print!(
            "- ActivateChannel : CM(0x{:x}), {}\n",
            *g_xlChannelMask.lock().unwrap(),
            xl_get_error_string(xlStatus)
        );
    } else {
        demoStopTransmitBurst();
        xlStatus = xl_deactivate_channel(
            *g_xlPortHandle.lock().unwrap(),
            *g_xlChannelMask.lock().unwrap(),
        );
        print!(
            "- DeativateChannel: CM(0x{:x}), {}\n",
            *g_xlChannelMask.lock().unwrap(),
            xl_get_error_string(xlStatus)
        );
    }

    return XL_SUCCESS;
}

////////////////////////////////////////////////////////////////////////////
// demoSetOutput
// toggle NORMAL/SILENT mode of a CAN channel
//
////////////////////////////////////////////////////////////////////////////
fn demoSetOutput(outputMode: c_uchar, sMode: &str, xlChanMaskTx: XLaccess) -> XLstatus {
    let mut xlStatus: XLstatus;

    // to get an effect we deactivate the channel first.
    xlStatus = xl_deactivate_channel(
        *g_xlPortHandle.lock().unwrap(),
        *g_xlChannelMask.lock().unwrap(),
    );

    xlStatus = xl_can_set_channel_output(*g_xlPortHandle.lock().unwrap(), xlChanMaskTx, outputMode);
    print!(
        "- SetChannelOutput: CM(0x{:x}), {}, {}, {}\n",
        xlChanMaskTx,
        sMode,
        xl_get_error_string(xlStatus),
        outputMode
    );

    // and activate the channel again.
    xlStatus = xl_activate_channel(
        *g_xlPortHandle.lock().unwrap(),
        *g_xlChannelMask.lock().unwrap(),
        XL_BUS_TYPE_CAN,
        XL_ACTIVATE_RESET_CLOCK,
    );

    return xlStatus;
}

////////////////////////////////////////////////////////////////////////////
// demoCreateRxThread
// set the notification and creates the thread.
//
////////////////////////////////////////////////////////////////////////////

fn demoCreateRxThread() -> XLstatus {
    let mut xlStatus: XLstatus = XL_ERROR;
    let mut ThreadId: u32 = 0u32;

    if *g_xlPortHandle.lock().unwrap() != XL_INVALID_PORTHANDLE {
        // Send a event for each Msg!!!
        xlStatus = xl_set_notification(
            *g_xlPortHandle.lock().unwrap(),
            &mut ((*g_hMsgEvent.lock().unwrap()).0),
            1,
        );

        if *g_canFdSupport.lock().unwrap() != 0u32 {
            *g_hRXThread.lock().unwrap() = unsafe {
                CreateThread(
                    Option::None,
                    0x1000,
                    Option::Some(RxCanFdThread),
                    Option::None,
                    THREAD_CREATION_FLAGS(0),
                    Option::Some(&mut ThreadId),
                )
            }
            .unwrap();
        } else {
            *g_hRXThread.lock().unwrap() = unsafe {
                CreateThread(
                    Option::None,
                    0x1000,
                    Option::Some(RxThread),
                    Option::None,
                    THREAD_CREATION_FLAGS(0),
                    Option::Some(&mut ThreadId),
                )
            }
            .unwrap();
        }
    }

    return xlStatus;
}

////////////////////////////////////////////////////////////////////////////
// demoInitDriver
// initializes the driver with one port and all founded channels which
// have a connected CAN cab/piggy.
//
////////////////////////////////////////////////////////////////////////////
fn demoInitDriver(
    pxlChannelMaskTx: &mut XLaccess,
    pxlChannelIndex: &mut c_uint,
    app_name: &String,
) -> XLstatus {
    let mut xlStatus: XLstatus;
    let mut i: usize;
    let mut xlChannelMaskFd: XLaccess = 0;
    let mut xlChannelMaskFdNoIso: XLaccess = 0;

    // ------------------------------------
    // open the driver
    // ------------------------------------
    xlStatus = xl_open_driver();

    // ------------------------------------
    // get/print the hardware configuration
    // ------------------------------------
    if XL_SUCCESS == xlStatus {
        xlStatus = xl_get_driver_config(g_xlDrvConfig.lock().as_mut().unwrap());
    }

    if XL_SUCCESS == xlStatus {
        demoPrintConfig();

        print!("Usage: cargo run --bin xlCANdemo <ApplicationName> <Identifier>\n\n");

        // ------------------------------------
        // select the wanted channels
        // ------------------------------------
        {
            *g_xlChannelMask.lock().unwrap() = 0;
        }

        let channel_cnt: usize;
        {
            channel_cnt = g_xlDrvConfig.lock().unwrap().channelCount as usize;
        }

        for i in 0..channel_cnt {
            // we take all hardware we found and supports CAN
            if g_xlDrvConfig.lock().unwrap().channel[i].channelBusCapabilities
                & XL_BUS_ACTIVE_CAP_CAN
                != 0
            {
                if !(*pxlChannelMaskTx != 0) {
                    *pxlChannelMaskTx = g_xlDrvConfig.lock().unwrap().channel[i].channelMask;
                    *pxlChannelIndex = g_xlDrvConfig.lock().unwrap().channel[i].channelIndex as u32;
                }

                // check if we can use CAN FD - the virtual CAN driver supports CAN-FD, but we don't use it
                if (g_xlDrvConfig.lock().unwrap().channel[i].channelCapabilities
                    & XL_CHANNEL_FLAG_CANFD_ISO_SUPPORT
                    != 0)
                    && (g_xlDrvConfig.lock().unwrap().channel[i].hwType != XL_HWTYPE_VIRTUAL)
                {
                    xlChannelMaskFd |= g_xlDrvConfig.lock().unwrap().channel[i].channelMask;

                    // check CAN FD NO ISO support
                    if g_xlDrvConfig.lock().unwrap().channel[i].channelCapabilities
                        & XL_CHANNEL_FLAG_CANFD_BOSCH_SUPPORT
                        != 0
                    {
                        xlChannelMaskFdNoIso |=
                            g_xlDrvConfig.lock().unwrap().channel[i].channelMask;
                    }
                } else {
                    *g_xlChannelMask.lock().unwrap() |=
                        g_xlDrvConfig.lock().unwrap().channel[i].channelMask;
                }
            }
        }

        // if we found a CAN FD supported channel - we use it.
        if (xlChannelMaskFd != 0u64) && (*g_canFdModeNoIso.lock().unwrap() == 0u32) {
            *g_xlChannelMask.lock().unwrap() = xlChannelMaskFd;
            print!(
                "- Use CAN-FD for   : CM=0x{:x}\n",
                *g_xlChannelMask.lock().unwrap()
            );
            *g_canFdSupport.lock().unwrap() = 1;
        }

        if (xlChannelMaskFdNoIso != 0u64) && (*g_canFdModeNoIso.lock().unwrap() != 0u32) {
            *g_xlChannelMask.lock().unwrap() = xlChannelMaskFdNoIso;
            print!(
                "- Use CAN-FD NO ISO for   : CM=0x{:x}\n",
                *g_xlChannelMask.lock().unwrap()
            );
            *g_canFdSupport.lock().unwrap() = 1;
        }

        if *g_xlChannelMask.lock().unwrap() == 0u64 {
            print!("ERROR: no available channels found! (e.g. no CANcabs...)\n\n");
            xlStatus = XL_ERROR;
        }
    }

    *g_xlPermissionMask.lock().unwrap() = *g_xlChannelMask.lock().unwrap();

    // ------------------------------------
    // open ONE port including all channels
    // ------------------------------------
    if XL_SUCCESS == xlStatus {
        // check if we can use CAN FD
        if *g_canFdSupport.lock().unwrap() != 0 {
            xlStatus = xl_open_port(
                g_xlPortHandle.lock().as_mut().unwrap(),
                app_name,
                *g_xlChannelMask.lock().unwrap(),
                g_xlPermissionMask.lock().as_mut().unwrap(),
                RX_QUEUE_SIZE_FD,
                XL_INTERFACE_VERSION_V4,
                XL_BUS_TYPE_CAN,
            );
        }
        // if not, we make 'normal' CAN
        else {
            xlStatus = xl_open_port(
                g_xlPortHandle.lock().as_mut().unwrap(),
                app_name,
                *g_xlChannelMask.lock().unwrap(),
                g_xlPermissionMask.lock().as_mut().unwrap(),
                RX_QUEUE_SIZE,
                XL_INTERFACE_VERSION,
                XL_BUS_TYPE_CAN,
            );
        }

        print!(
            "- OpenPort         : CM=0x{:x}, PH=0x{:02x}, PM=0x{:x}, {}\n",
            *g_xlChannelMask.lock().unwrap(),
            *g_xlPortHandle.lock().unwrap(),
            *g_xlPermissionMask.lock().unwrap(),
            xl_get_error_string(xlStatus)
        );
    }

    if (XL_SUCCESS == xlStatus) && (XL_INVALID_PORTHANDLE != *g_xlPortHandle.lock().unwrap()) {
        // ------------------------------------
        // if we have permission we set the
        // bus parameters (baudrate)
        // ------------------------------------
        if *g_xlChannelMask.lock().unwrap() == *g_xlPermissionMask.lock().unwrap() {
            if *g_canFdSupport.lock().unwrap() != 0 {
                let mut fdParams: XLcanFdConf = XLcanFdConf {
                    arbitrationBitRate: 0,
                    sjwAbr: 0, // CAN bus timing for nominal / arbitration bit rate
                    tseg1Abr: 0,
                    tseg2Abr: 0,
                    dataBitRate: 0,
                    sjwDbr: 0, // CAN bus timing for data bit rate
                    tseg1Dbr: 0,
                    tseg2Dbr: 0,
                    reserved: 0,       // has to be zero
                    options: 0,        // CANFD_CONFOPT_
                    reserved1: [0, 0], // has to be zero
                    reserved2: 0,      // has to be zero
                };

                // arbitration bitrate
                fdParams.arbitrationBitRate = 1000000;
                fdParams.tseg1Abr = 6;
                fdParams.tseg2Abr = 3;
                fdParams.sjwAbr = 2;

                // data bitrate
                fdParams.dataBitRate = fdParams.arbitrationBitRate * 2;
                fdParams.tseg1Dbr = 6;
                fdParams.tseg2Dbr = 3;
                fdParams.sjwDbr = 2;

                if *g_canFdModeNoIso.lock().unwrap() != 0 {
                    fdParams.options = CANFD_CONFOPT_NO_ISO;
                }

                let arbitration_bitrate = fdParams.arbitrationBitRate; // for removing unaligned fields warnings
                let data_bitrate: u32 = fdParams.dataBitRate; // for removing unaligned fields warnings

                xlStatus = xl_can_fd_set_configuration(
                    *g_xlPortHandle.lock().unwrap(),
                    *g_xlChannelMask.lock().unwrap(),
                    &fdParams,
                );
                print!(
                    "- SetFdConfig.     : ABaudr.={}, DBaudr.={}, {}\n",
                    arbitration_bitrate,
                    data_bitrate,
                    xl_get_error_string(xlStatus)
                );
            } else {
                xlStatus = xl_can_set_channel_bitrate(
                    *g_xlPortHandle.lock().unwrap(),
                    *g_xlChannelMask.lock().unwrap(),
                    *g_BaudRate.lock().unwrap(),
                );
                print!(
                    "- SetChannelBitrate: baudrate.={}, {}\n",
                    g_BaudRate.lock().unwrap(),
                    xl_get_error_string(xlStatus)
                );
            }
        } else {
            print!("-                  : we have NO init access!\n");
        }
    } else {
        xl_close_port(*g_xlPortHandle.lock().unwrap());
        *g_xlPortHandle.lock().unwrap() = XL_INVALID_PORTHANDLE;
        xlStatus = XL_ERROR;
    }

    return xlStatus;
}

////////////////////////////////////////////////////////////////////////////
// demoCleanUp()
// close the port and the driver
//
////////////////////////////////////////////////////////////////////////////
fn demoCleanUp() -> XLstatus {
    let xlStatus: XLstatus;

    if *g_xlPortHandle.lock().unwrap() != XL_INVALID_PORTHANDLE {
        xlStatus = xl_close_port(*g_xlPortHandle.lock().unwrap());
        print!(
            "- ClosePort        : PH(0x{:x}), {}\n",
            *g_xlPortHandle.lock().unwrap(),
            xl_get_error_string(xlStatus)
        );
    }

    *g_xlPortHandle.lock().unwrap() = XL_INVALID_PORTHANDLE;
    xl_close_driver();

    return XL_SUCCESS; // No error handling
}

////////////////////////////////////////////////////////////////////////////
// TxThread
//
//
////////////////////////////////////////////////////////////////////////////

unsafe extern "system" fn TxThread(_: *mut c_void) -> u32 {
    let mut xlStatus: XLstatus = XL_SUCCESS;
    let n: c_uint = 1;
    let mut canTxEvt: XLcanTxEvent = Default::default();
    let mut xlEvent: XLevent = Default::default();
    let mut cntSent: c_uint = 0u32;

    if *g_canFdSupport.lock().unwrap() != 0u32 {
        canTxEvt.tag = XL_CAN_EV_TAG_TX_MSG;

        canTxEvt.tagData.canMsg.canId = *g_TXThreadCanId.lock().unwrap();
        canTxEvt.tagData.canMsg.msgFlags = XL_CAN_TXMSG_FLAG_EDL | XL_CAN_TXMSG_FLAG_BRS;
        canTxEvt.tagData.canMsg.dlc = 15;

        for i in 1..XL_CAN_MAX_DATA_LEN {
            canTxEvt.tagData.canMsg.data[i] = (i - 1) as u8;
        }
    } else {
        xlEvent.tag = XL_TRANSMIT_MSG;
        xlEvent.tagData.msg.id = *g_TXThreadCanId.lock().unwrap();
        xlEvent.tagData.msg.dlc = 8u16;
        xlEvent.tagData.msg.flags = 0u16;
        xlEvent.tagData.msg.data[0] = 1u8;
        xlEvent.tagData.msg.data[1] = 2u8;
        xlEvent.tagData.msg.data[2] = 3u8;
        xlEvent.tagData.msg.data[3] = 4u8;
        xlEvent.tagData.msg.data[4] = 5u8;
        xlEvent.tagData.msg.data[5] = 6u8;
        xlEvent.tagData.msg.data[6] = 7u8;
        xlEvent.tagData.msg.data[7] = 8u8;
    }

    while (*g_TXThreadRun.lock().unwrap() != 0i32) && (XL_SUCCESS == xlStatus) {
        if *g_canFdSupport.lock().unwrap() != 0u32 {
            canTxEvt.tagData.canMsg.data[0] += if canTxEvt.tagData.canMsg.data[0] == 0xFFu8 {
                canTxEvt.tagData.canMsg.data[0] = 0u8;
                0u8
            } else {
                1u8
            };
            xlStatus = xl_can_transmit_ex(
                *g_xlPortHandle.lock().unwrap(),
                *g_TXThreadTxMask.lock().unwrap(),
                n,
                &mut cntSent,
                &canTxEvt,
            );
        } else {
            xlEvent.tagData.msg.data[0] += if xlEvent.tagData.msg.data[0] == 0xFFu8 {
                xlEvent.tagData.msg.data[0] = 0u8;
                0u8
            } else {
                1u8
            };
            xlStatus = xl_can_transmit(
                *g_xlPortHandle.lock().unwrap(),
                *g_TXThreadTxMask.lock().unwrap(),
                &n,
                ptr::addr_of_mut!(xlEvent) as *const c_void,
            );
        }

        unsafe { Sleep(10) };
    }

    if XL_SUCCESS != xlStatus {
        print!("Error xlCanTransmit:{}\n", xl_get_error_string(xlStatus));
    }

    *g_TXThreadRun.lock().unwrap() = 0;
    return 0u32; // NO_ERROR;
}

const RECEIVE_EVENT_SIZE: c_int = 1; // DO NOT EDIT! Currently 1 is supported only

///////////////////////////////////////////////////////////////////////////
// RxThread
// thread to readout the message queue and parse the incoming messages
//
////////////////////////////////////////////////////////////////////////////

unsafe extern "system" fn RxThread(_: *mut c_void) -> c_uint {
    let mut xlStatus: XLstatus;

    let mut msgsrx: c_int = RECEIVE_EVENT_SIZE;
    let mut xlEvent: XLevent = Default::default();

    *g_RXThreadRun.lock().unwrap() = 1i32;

    while *g_RXThreadRun.lock().unwrap() != 0i32 {
        let rc = unsafe { WaitForSingleObject(*g_hMsgEvent.lock().unwrap(), 10) };

        xlStatus = XL_SUCCESS;

        while !(xlStatus != 0) {
            msgsrx = RECEIVE_EVENT_SIZE;

            xlStatus = xl_receive(*g_xlPortHandle.lock().unwrap(), &mut msgsrx, &mut xlEvent);
            if xlStatus != XL_ERR_QUEUE_IS_EMPTY {
                if !(g_silent != 0) {
                    print!("{}\n", xl_get_event_string(&xlEvent));
                }
            }
        }
    }
    return 0u32; // NO_ERROR;
}

///////////////////////////////////////////////////////////////////////////
// RxCANThread
// thread to read the message queue and parse the incoming messages
//
////////////////////////////////////////////////////////////////////////////
unsafe extern "system" fn RxCanFdThread(_: *mut c_void) -> c_uint {
    let mut xlStatus: XLstatus = XL_SUCCESS;
    let mut rc: u32;
    let mut xlCanRxEvt: XLcanRxEvent = XLcanRxEvent::default();

    *g_RXCANThreadRun.lock().unwrap() = 1;

    while *g_RXCANThreadRun.lock().unwrap() != 0i32 {
        rc = unsafe { WaitForSingleObject(*g_hMsgEvent.lock().unwrap(), 10) }.0;

        if rc != 0
        /* WAIT_OBJECT_0 */
        {
            continue;
        }

        while XL_SUCCESS == xlStatus {
            xlStatus = xl_can_receive(*g_xlPortHandle.lock().unwrap(), &mut xlCanRxEvt);

            if xlStatus == XL_ERR_QUEUE_IS_EMPTY {
                xlStatus = XL_SUCCESS;
                break;
            }

            if !(g_silent != 0i32) {
                print!("{}\n", xl_can_get_event_string(&xlCanRxEvt));
            }
        }
    }

    return 0; // NO_ERROR
} // RxCanFdThread
