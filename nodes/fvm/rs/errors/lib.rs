#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs;

agent! {
    input(file_error: fs_file_error, semantic_error: core_semantic_error),
    output(output: core_graph),
    fn run(&mut self) -> Result<Signal>{

        match self.input.semantic_error.try_recv() {
            Ok(mut msg) => {
                let error: core_semantic_error::Reader = try!(msg.read_schema());

                println!("Graph at : {}", try!(error.get_path()));
                let parsing = error.get_parsing()?.iter();
                for error in parsing {
                    println!("  {}", error?);
                }
                println!("");
            }
            _ => {}
        };

        match self.input.file_error.try_recv() {
            Ok(mut msg) => {
                let error: fs_file_error::Reader = try!(msg.read_schema());
                println!("Subgraph doesn't exist at file location : {}\n", try!(error.get_not_found()));
            }
            _ => {}
        };


        let mut new_msg = Msg::new();
        {
            let mut msg = new_msg.build_schema::<core_graph::Builder>();
            msg.set_path("error");
        }
        let _ = self.output.output.send(new_msg);
        Ok(End)
    }
}
