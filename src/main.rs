// use std::fs::*;

// struct Signal

use std::collections::HashMap;

enum PortDir {
    In,
    Out,
    Inout,
    Nodir,
}

struct Wire {
    attrs: HashMap<String, String>,
    id: String,
    width: usize,
    port: usize,
    port_dir: PortDir,
    offset: usize,
    upto: bool,
    signed: bool,
}

impl Wire {
    fn new(name: &str) -> Self {
        Wire {
            attrs: HashMap::new(),
            id: name.to_string(),
            width: 0,
            port: 0,
            port_dir: PortDir::Nodir,
            offset: 0,
            upto: false,
            signed: false,
        }
    }
    fn emit(&self, ident_lvl: usize) -> String {
        let mut wire_text = String::new();
        for (k, v) in self.attrs.clone().into_iter() {
            wire_text += "\t".repeat(ident_lvl).as_str();
            wire_text += format!("attribute \\{} {}\n", k, v).as_str();
        }
        wire_text += "\t".repeat(ident_lvl).as_str();
        wire_text += format!("wire \\{} ", self.id).as_str();
        wire_text += format!("width \\{} ", self.id).as_str();
        wire_text += format!("width \\{} ", self.id).as_str();
        // wire_text += 
        wire_text
    } 

}



struct Module {
    attrs: HashMap<String, String>,
    id: String,
    wires: Vec<Wire>,
}

impl Module {
    fn emit(&self) -> String {
        let mut module_text = String::new();
        let mut ident_level = 0;
        module_text.push_str("autoidx 1\n");
        for (k, v) in self.attrs.clone().into_iter() {
            module_text.push_str(format!("attribute \\{} {}\n", k, v).as_str());
        }
        module_text.push_str(format!("module \\{}\n", self.id).as_str());
        ident_level += 1;

        for i in 0..self.wires.len() {

        }

        /* Emmit module contents */
        module_text.push_str("end\n");
        module_text
    }
}

fn gen_wire(name: &str, width: usize) -> String {
    let mut wire_text = String::from("wire");
    if width > 1 {
        wire_text += format!(" width {}", width).as_str()
    }
    wire_text += format!(" \\{name}\n").as_str();
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
        _ => {}
    }
    port_text.push_str(format!(" \\{name}\n").as_str());
    port_text
}

fn cell(name: &str) -> String {
    let mut cell_text = format!("cell $and \\{name}\n");
    // cell_text +=
    cell_text
}

fn connect(from: &str, to: &str) -> String {
    format!("connect \\{from} \\{to}\n")
}

fn gen_module() -> String {
    let mut design_text = String::new();
    let mut port_count: usize = 0;
    design_text.push_str("module \\top\n");
    design_text += "\t";
    design_text += &gen_wire("net", 2);
    design_text += "\t";
    design_text += &gen_port("x1", 1, PortDir::In, &mut port_count);
    design_text += "\t";
    design_text += &gen_port("x2", 1, PortDir::In, &mut port_count);
    design_text += "\t";
    design_text += &gen_port("y1", 1, PortDir::Out, &mut port_count);
    design_text += "\t";
    design_text += &gen_port("y2", 1, PortDir::Out, &mut port_count);
    design_text += "\t";
    design_text += &connect("net[0]", "y1");
    design_text += "\t";
    design_text += &connect("net[1]", "y2");
    design_text.push_str("endmodule\n");
    design_text
}

fn main() {
    print!("{}", gen_module());
    // println!("Hello, world!");
}
