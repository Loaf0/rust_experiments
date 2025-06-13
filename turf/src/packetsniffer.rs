#![allow(unused)]

use eframe::egui;
use std::sync::mpsc::{self, Receiver};
use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pnet::packet::{ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, Packet};
use std::fs;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native("Turf", options, Box::new(|_cc| Ok(Box::<MyApp>::default())))
}

struct MyApp {
    selected_tab: Tab,
    packets: Vec<PacketInfo>,
    capturing: bool,
    packet_rx: Option<Receiver<PacketInfo>>,
    network_interfaces: Vec<NetworkInterface>,
    selected_interface_idx: usize,
    filter: String,
    selected_packet: Option<usize>,
}

#[derive(PartialEq, Eq)]
enum Tab {
    PacketInspection,
    MemoryEditing,
    Settings,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::PacketInspection
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let interfaces = datalink::interfaces();
        Self {
            selected_tab: Tab::PacketInspection,
            packets: Vec::new(),
            capturing: false,
            packet_rx: None,
            network_interfaces: interfaces,
            selected_interface_idx: 0,
            filter: String::new(),
            selected_packet: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(rx) = &self.packet_rx {
            while let Ok(packet) = rx.try_recv() {
                self.packets.push(packet);
            }
        }

        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_tab == Tab::PacketInspection, "Packet Inspection").clicked() {
                    self.selected_tab = Tab::PacketInspection;
                }
                if ui.selectable_label(self.selected_tab == Tab::MemoryEditing, "Memory Editing").clicked() {
                    self.selected_tab = Tab::MemoryEditing;
                }
                if ui.selectable_label(self.selected_tab == Tab::Settings, "Settings").clicked() {
                    self.selected_tab = Tab::Settings;
                }
            });
        });
        

        match self.selected_tab {
            Tab::PacketInspection => {
                egui::SidePanel::left("packet_list_panel").min_width(400.0).show(ctx, |ui| {
                    ui.heading("Packets");

                    // Network interface selector
                    ui.horizontal(|ui| {
                        ui.label("Interface:");
                        egui::ComboBox::from_id_salt("interface_selector")
                            .selected_text(
                                self.network_interfaces
                                    .get(self.selected_interface_idx)
                                    .map(|iface| iface.name.clone())
                                    .unwrap_or_else(|| "None".to_string())
                            )
                            .show_ui(ui, |cb_ui| {
                                for (i, iface) in self.network_interfaces.iter().enumerate() {
                                    let ip = iface.ips.get(0)
                                        .map(|ip| ip.to_string())
                                        .unwrap_or_else(|| "No IP".to_string());
                                    let label = format!("{} ({})", iface.name, ip);
                                    cb_ui.selectable_value(&mut self.selected_interface_idx, i, label);
                                }
                            });
                    });

                    // Buttons row
                    ui.horizontal(|ui| {
                        if ui.button("Start").clicked() {
                            if !self.capturing {
                                self.capturing = true;
                                let (tx, rx) = mpsc::channel();
                                self.packet_rx = Some(rx);

                                let interface = self.network_interfaces[self.selected_interface_idx].clone();
                                std::thread::spawn(move || {
                                    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
                                        Ok(Ethernet(_tx, rx)) => (_tx, rx),
                                        Ok(_) => panic!("Unhandled channel type"),
                                        Err(e) => panic!("Failed to create datalink channel: {}", e),
                                    };

                                    while let Ok(packet) = rx.next() {
                                        let summary = format!("Packet: {} bytes", packet.len());
                                        let info = PacketInfo {
                                            summary,
                                            data: packet.to_vec(),
                                        };
                                        let _ = tx.send(info);
                                    }
                                });
                            }
                        }
                        if ui.button("Pause").clicked() {
                            self.capturing = false;
                            self.packet_rx = None;
                        }
                        if ui.button("Clear").clicked() {
                            self.capturing = false;
                            self.packets.clear();
                            self.packet_rx = None;
                        }
                    });

                    // Filter row
                    ui.horizontal(|ui| {
                        ui.label("Filter:");
                        ui.text_edit_singleline(&mut self.filter);
                    });
                    ui.separator();

                    // Packet table
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("packet_table")
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Seq");
                                ui.label("Source");
                                ui.label("Destination");
                                ui.label("Protocol");
                                ui.label("Length");
                                ui.end_row();

                                for (idx, packet) in self.packets.iter().enumerate() {
                                    let (src, dst, proto) = parse_packet_info(&packet.data);
                                    let filter = self.filter.to_lowercase();
                                    if !filter.is_empty()
                                        && !src.to_lowercase().contains(&filter)
                                        && !dst.to_lowercase().contains(&filter)
                                        && !proto.to_lowercase().contains(&filter)
                                    {
                                        continue;
                                    }
                                    let selected = Some(idx) == self.selected_packet;
                                    if ui.selectable_label(selected, format!("{}", idx + 1)).clicked() {
                                        self.selected_packet = Some(idx);
                                    }
                                    ui.label(src);
                                    ui.label(dst);
                                    ui.label(proto);
                                    ui.label(format!("{}", packet.data.len()));
                                    ui.end_row();
                                }
                            });
                    });

                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Packet Details");
                    if let Some(idx) = self.selected_packet {
                        if let Some(packet) = self.packets.get(idx) {
                            if ui.button("Resend Packet").clicked() {
                                if let Some(interface) = self.network_interfaces.get(self.selected_interface_idx) {
                                    if let Ok(pnet::datalink::Channel::Ethernet(mut tx, _)) = pnet::datalink::channel(interface, Default::default()) {
                                        let _ = tx.send_to(&packet.data, None);
                                    }
                                }
                            }

                            if ui.button("Save Packet Contents").clicked() {
                                let seq = idx + 1;
                                let contents = format!("{:02X?}", &packet.data);
                                let _ = fs::create_dir_all("packets");
                                fs::write(format!("packets/packet{}.txt", seq), contents.as_bytes()).unwrap();
                            }

                            ui.label(format!("Summary: "));
                            ui.label(format!("Length: {} bytes", packet.data.len()));
                            ui.label(format!("Data (hex): {:02X?}", &packet.data));
                            let text = String::from_utf8_lossy(&packet.data);
                            ui.label("Text:");
                            ui.add(egui::TextEdit::multiline(&mut text.clone()).font(egui::TextStyle::Monospace).desired_rows(4).lock_focus(true));
                        } else {
                            ui.label("No packet selected.");
                        }
                    } else {
                        ui.label("Select a packet to view details.");
                    }
                });
            }
            Tab::MemoryEditing => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Memory Editing");
                    ui.label("Here you can edit process memory.");
                });
            }
            Tab::Settings => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Settings");
                    ui.label("You can change things here.");
                });
            }
        }
    }
}

struct PacketInfo {
    summary: String,
    data: Vec<u8>,
}

// Helper to parse Ethernet/IPv4 info for display
fn parse_packet_info(data: &[u8]) -> (String, String, String) {
    if let Some(eth) = EthernetPacket::new(data) {
        let proto = format!("{:?}", eth.get_ethertype());
        if eth.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
            if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                let proto_name = match ipv4.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => "TCP",
                    IpNextHeaderProtocols::Udp => "UDP",
                    IpNextHeaderProtocols::Icmp => "ICMP",
                    IpNextHeaderProtocols::Igmp => "IGMP",
                    other => return (
                        ipv4.get_source().to_string(),
                        ipv4.get_destination().to_string(),
                        format!("{:?}", other),
                    ),
                };
                return (
                    ipv4.get_source().to_string(),
                    ipv4.get_destination().to_string(),
                    proto_name.to_string(),
                );
            }
        }
        (
            eth.get_source().to_string(),
            eth.get_destination().to_string(),
            proto,
        )
    } else {
        ("?".into(), "?".into(), "?".into())
    }
}
