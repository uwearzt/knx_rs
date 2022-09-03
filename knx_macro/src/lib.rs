// ------------------------------------------------------------------------------
// Copyright 2022 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::{punctuated::Punctuated, MetaNameValue, Token};
use std::str::FromStr;

// ------------------------------------------------------------------------------
#[derive(Debug)]
struct DptData {
    name: String,
    main: u16,
    sub: u16,
}

impl DptData {
    // format to "DPT::DPT_Switch => 1,"
    fn to_main_match(&self, enum_id: &str) -> String {
        return format!(
            "{}::{} => {},",
            enum_id, self.name, self.main
        );
    }
    // format to "DPT::DPT_Switch => 1,"
    fn to_sub_match(&self, enum_id: &str) -> String {
        return format!(
            "{}::{} => {},",
            enum_id, self.name, self.sub
        );
    }
    // format to "DPT::DPT_Switch => format!(\"DPST-1-1\"),"
    fn to_dpst_match(&self, enum_id: &str) -> String {
        return format!(
            "{}::{} => \"DPST-{}-{}\",",
            enum_id, self.name, self.main, self.sub
        );
    }
    // format to "(1, 1) => Ok(DPT::DPT_Switch)"
    fn to_main_sub_match(&self, enum_id: &str) -> String {
        return format!(
            "({}, {}) => Ok({}::{}),",
            self.main, self.sub, enum_id, self.name
        );
    }
}

// ------------------------------------------------------------------------------
#[proc_macro_derive(dpt_types, attributes(dpt_type))]
pub fn dpt_types(input: TokenStream) -> TokenStream {

    let mut dpt_vec : Vec<DptData> = Vec::new();

    let mut ast: DeriveInput = syn::parse(input.clone()).unwrap();
    //eprintln!("{:?}", ast);

    if let syn::Data::Enum(ref mut data_enum) = ast.data {
        //eprintln!("enum {}", ast.ident);
        let enum_id = ast.ident.to_string();

        // ----------
        for s in &mut data_enum.variants {
            let x = s.ident.to_string();
            // eprintln!("------------------------------------------");
            // eprintln!("{:?}", s);
            // eprintln!("------------------------------------------");
            // eprintln!("{:?}", s.attrs);

            if s.attrs.is_empty() {
                panic!("no #[dpt_type] for {x}");
            }

            let mut main = 0;
            let mut sub = 0;

            for l in &s.attrs {
                if l.path.is_ident("dpt_type") {

                    let name_values: Result<Punctuated<MetaNameValue, Token![,]>, _> =
                        l.parse_args_with(Punctuated::parse_terminated);
                    // eprintln!("{:?}", name_values);

                    match name_values {
                        Ok(name_value) => {
                            for nv in name_value {
                                let value = match nv.lit {
                                    syn::Lit::Int(v) => v.base10_parse::<u16>().unwrap(),
                                    _ => panic!("expeced a u16 value"),
                                };
                                //eprintln!("Meta VALUE: {:?}", value);
                                //eprintln!("Meta NV: {:?}", nv.path.get_ident());
                                match nv.path.get_ident().unwrap().to_string().as_str() {
                                    "main" => main = value,
                                    "sub" => sub = value,
                                    _ => panic!("expected main/sub as names"),
                                }
                            }
                        }
                        Err(_) => todo!(),
                    };
                }
            }

            dpt_vec.push(DptData {
                name: x,
                main: main,
                sub: sub,
            });
        }
        //eprintln!("{:?}", dpt_vec);

        // ----------

        let ts = gen_impl(enum_id, dpt_vec);
        return ts;
    } else {
        panic!("#[derive(dpt_types)] only applicable to enums");
    }

}

fn gen_impl(enum_id: String, dpt_vec: Vec<DptData>) -> TokenStream {

    let mut implementation = String::new();
    implementation.push_str("
    use core::str::FromStr;
    #[cfg(feature = \"std\")]
    use regex::Regex;");
    
    implementation.push_str(&format!("impl {enum_id} {{"));

    if !dpt_vec.is_empty() {
        // format to DPST string
        let match_dpst_strings: Vec<String> =
            dpt_vec.iter().map(|x| x.to_dpst_match(&enum_id)).collect();
        let match_dpst_string = match_dpst_strings.join("\n");
        // eprintln!("{}", match_dpst_string);

        implementation.push_str(
            "
            #[cfg(feature = \"std\")]
       pub fn dpst(&self) -> &str {
           match &*self {",
        );
        implementation.push_str(&match_dpst_string);
        implementation.push_str(
            "
          }
       }",
        );

        // parse from DPST string
        let main_sub_match_strings: Vec<String> = dpt_vec
            .iter()
            .map(|x| x.to_main_sub_match(&enum_id))
            .collect();
        let main_sub_match_string = main_sub_match_strings.join("\n");
        // eprintln!("{}", main_sub_match_string);

        implementation.push_str(
            "
            #[cfg(feature = \"std\")]
                    pub fn from_dpst(s: &str) -> Result<Self, u8> {
                        let re = Regex::new(r\"DPST-(?P<main>\\d{1})-(?P<sub>\\d{1})\").unwrap();
                        let caps = re.captures(s).unwrap();
        
                        let main = u16::from_str(&caps[\"main\"]).unwrap();
                        let sub = u16::from_str(&caps[\"sub\"]).unwrap();
        
                        match (main, sub) {",
        );
        implementation.push_str(&main_sub_match_string);
        implementation.push_str(
            "(_, _) => Err(42),
                }
            
        }",
        );

        // to main 
        let to_main_match_strings: Vec<String> = dpt_vec
            .iter()
            .map(|x| x.to_main_match(&enum_id))
            .collect();
        let to_main_match_string = to_main_match_strings.join("\n");
        // eprintln!("{}", to_main_match_string);

        implementation.push_str(
            "
       pub fn main(&self) -> u16 {
           match &*self {",
        );
        implementation.push_str(&to_main_match_string);
        implementation.push_str(
            "
          }
       }");
        // to sub 
        let to_sub_match_strings: Vec<String> = dpt_vec
            .iter()
            .map(|x| x.to_sub_match(&enum_id))
            .collect();
        let to_sub_match_string = to_sub_match_strings.join("\n");
        // eprintln!("{}", to_sub_match_string);

        implementation.push_str(
            "
       pub fn sub(&self) -> u16 {
           match &*self {",
        );
        implementation.push_str(&to_sub_match_string);
        implementation.push_str(
            "
          }
       }");
    
    }

    implementation.push_str("}");

    //eprintln!("{}", implementation);

    return TokenStream::from_str(&implementation).unwrap();
}
