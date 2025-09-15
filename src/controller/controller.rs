use std::sync::{Arc, Mutex};
use std::thread::spawn;

#[derive(Clone)]
struct BusInfo {
    tx_state: ElectricalState,
    rx_state: ElectricalState,
    clk_state: ElectricalState,
}


#[derive(Clone)]
enum ElectricalState {
    High,
    Low,
    HighImpedance,
}


/// Start the clock and send data_tx continuously
fn create_transmission_process(bus: &mut BusInfo, hertz: u8, data_tx: &mut u8){
}