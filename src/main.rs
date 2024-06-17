// use std::fs::*;

// struct Signal

use std::collections::HashMap;

// enum ModuleStmt {
//     Wire(Wire),
// }
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
        wire_text += "wire ";
        if self.width > 0 {wire_text += format!("width {} ", self.width).as_str();}
        if self.offset > 0 {wire_text += format!("offset {} ", self.offset).as_str();}
        match self.port_dir {
            PortDir::In => wire_text += format!("input {} ", self.port).as_str(),
            PortDir::Out => wire_text += format!("output {} ", self.port).as_str(),
            PortDir::Inout => wire_text += format!("inout {} ", self.port).as_str(),
            PortDir::Nodir => {},
        }
        if self.upto {wire_text += "upto "};
        if self.signed {wire_text += "signed "};
        wire_text += format!("\\{}", self.id).as_str();
        wire_text += "\n";
        wire_text
    } 

}



struct Module {
    attrs: HashMap<String, String>,
    id: String,
    wires: Vec<Wire>,
}

impl Module {
    fn new() -> Self {
        Module {
            attrs: HashMap::new(),
            id: "top".to_string(),
            wires: Vec::new(),
        }
    }

    fn wire(&mut self, inst: Wire) {
        self.wires.push(inst);
    }

    fn emit(&self) -> String {
        let mut module_text = String::new();
        let mut ident_level = 0;

        module_text.push_str("autoidx 1\n");
        for (k, v) in self.attrs.clone().into_iter() {
            module_text.push_str(format!("attribute \\{} {}\n", k, v).as_str());
        }
        module_text.push_str(format!("module \\{}\n", self.id).as_str());
        ident_level += 1;

        module_text +=  self.wires.iter()
            .map(|wire| wire.emit(ident_level))
            .collect::<Vec<String>>()
            .join("")
            .as_str();

        /* Emmit module contents */
        module_text.push_str("end\n");
        module_text
    }
}

fn gen_module() -> String {
    let mut design_text = String::new();
    let mut port_count: usize = 0;

    let mut top = Module::new();

    port_count += 1;
    let iport = Wire {
        port_dir: PortDir::In,
        port: port_count,
        width: 8,
        ..Wire::new("din")
    };

    port_count += 1;
    let oport = Wire {
        port_dir: PortDir::Out,
        port: port_count,
        width: 2,
        ..Wire::new("dout")
    };

    top.wire(iport);
    top.wire(oport);

    design_text += top.emit().as_str();

    design_text
}

fn main() {
    print!("{}", gen_module());
}
