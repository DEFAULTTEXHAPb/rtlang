// use std::fs::*;

// struct Signal

enum PortDir {
    In,
    Out,
    Inout
}

fn gen_wire(name: &str, width: usize) -> String {
    let mut wire_text = String::from("wire");
    if width > 1 {
        wire_text.push_str(format!(" width {}", width).as_str())
    }
    wire_text.push_str(format!(" \\{name}\n").as_str());
    wire_text
}

fn gen_port(name: &str, width: usize, dir: PortDir, port_id: &mut usize) -> String {
    let mut port_text = String::from("wire");
    if width > 1 {
        port_text.push_str(format!(" width {}", width).as_str())
    }
    *port_id += 1;
    match dir {
        PortDir::In => port_text.push_str(format!(" input {}", *port_id).as_str()),
        PortDir::Out => port_text.push_str(format!(" output {}", *port_id).as_str()),
        PortDir::Inout => port_text.push_str(format!(" inout {}", *port_id).as_str()),
    }
    port_text.push_str(format!(" \\{name}\n").as_str());
    port_text
}


fn gen_module() -> String {
    let mut design_text = String::new();
    let mut port_count: usize = 0;
    design_text.push_str("module \\top\n");
    design_text += "\t";
    design_text += &gen_wire("net_1", 4);
    design_text += "\t";
    design_text += &gen_port("x1", 1, PortDir::In, &mut port_count);
    design_text.push_str("endmodule\n");
    design_text
}

fn main() {
    print!("{}", gen_module());
    // println!("Hello, world!");
}
