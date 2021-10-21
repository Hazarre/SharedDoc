/*
 * Copyright 2021 Li-Heng Henry Chang
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */


// for authentication 
use std::collections::HashMap;

// marine 
use marine_rs_sdk::{marine, get_call_parameters};
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;


use std::fs;
use std::path::PathBuf;


module_manifest!();
const SITES_DIR: &str = "/sites/";

/// Log level can be changed by `RUST_LOG` env as well.
pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

/// You can read or write files from the file system if there is permission to use directories described in `Config.toml`.
pub fn put(name: String, file_content: String) -> String {
    log::info!("put called with file name {}\n", name);
    let rpc_tmp_filepath = format!("{}{}", SITES_DIR, name);
    let result = fs::write(PathBuf::from(rpc_tmp_filepath), file_content.into_bytes());
    if let Err(e) = result {
        return format!("file can't be written: {}", e);
    }
    String::from("Ok")
}


pub fn get(file_name: String) -> String{
    log::info!("get called with file name: {}\n", file_name);
    let tmp_filepath = format!("{}{}", SITES_DIR, file_name);
    fs::read_to_string(tmp_filepath).unwrap_or_else(|_| String::from("") )
}

pub fn add(name: String, mut file_content: String) -> String {
    log::info!("put called with file name {}\n", name);
    let rpc_tmp_filepath = format!("{}{}", SITES_DIR, name);
    file_content.push_str(&get(name));
    let result = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content.into_bytes());
    if let Err(e) = result {
        return format!("file can't be written: {}", e);
    }
    String::from("Ok")
}

pub fn is_owner() -> bool {
    let meta = get_call_parameters();
    let caller = meta.init_peer_id;
    let owner = meta.service_creator_peer_id;
    caller == owner
}

#[marine]
pub fn am_i_owner() -> bool {
    is_owner()
}

fn pwdstr2hashmap(pwdstr_str: String)-> HashMap::<String, Vec<String>>{
    let mut hm = HashMap::<String, Vec<String>>::new();
    let docs_perm = pwdstr_str.split(";");

    for doc_perm in docs_perm{
        if !doc_perm.is_empty(){
            let mut split = doc_perm.split("-");
            let doc_title = split.next().unwrap_or("");
            let hashstr = split.next().unwrap_or("");
            let keys: Vec<String> = hashstr.split(",").map(|s| s.to_string()).collect();
            hm.insert(
                doc_title.to_string(), 
                keys
            );   
        }
    }
    hm
}

fn hashmap2pwdstr(hm: HashMap::<String, Vec<String>> )-> String {
    let mut output = "".to_string() ;
    for (doc, editors) in &hm {
        let mut doc_str = doc.to_string();
        doc_str.push_str("-");
        for e in editors{
            doc_str.push_str(e);
            doc_str.push_str(",");
        }
        doc_str = doc_str.to_string().split_at(doc_str.len() - 1).0.to_string();
        output.push_str(&doc_str);
        output.push_str(";");
    };
    output
}

fn pairin_hm(doc: String, editor: String, hm: HashMap::<String, Vec<String>> ) -> bool {
    if hm.contains_key(&doc) {
        let vec =  hm.get(&doc).unwrap();
        return vec.contains(&editor);
    }
    false
}


fn get_pwd_hm() -> HashMap::<String, Vec<String>>{
    let pwdstr = get("pwd.pwd".to_string());
    pwdstr2hashmap(pwdstr)
}

#[marine]
fn authenticate(filename: String, self_id: String) -> bool{
    let hm = get_pwd_hm();
    pairin_hm(filename, self_id, hm)
}

// create a document, this is a service that only the owner, or the deployer of the service is allowed to call
#[marine]
fn create_doc(filename: String, self_id: String) -> bool {
    // only the creator of the service can create a document 
    // true if file created 
    // false if the file already exists or if self_id is not the owners 

    if am_i_owner() {
        let hm = get_pwd_hm();
        if hm.contains_key(&filename) { // if the file exists
             return false 
        }
        else{ // if the file doens't exist 
            // create an empty file 
            let fname = filename.clone();
            put(filename, "".to_string());
            let mut pwdstr = fname;
            pwdstr.push_str("-");
            pwdstr.push_str(&self_id);
            pwdstr.push_str(";");
            add("pwd.pwd".to_string(), pwdstr);
            return true

        }
    }
    return false
}

// add editor 
#[marine]
fn add_editor(doc:String, editor: String,  self_id: String) ->bool {
    /*
    args: 
     - self_id: the self.id already with editor access 
     - editor: the self.id to be granted access
     - doc: the file name of the document

    out: 
     - true if document exists, editing permission will be granted to editor
     - false if doc doesn't exist!  => create a document first 
    */
    
    let mut hm = get_pwd_hm();
    if hm.contains_key(&doc) { // if document exists, grant editor permission 
        let mut vec = hm.get(&doc).unwrap().to_vec();
        if vec.contains(&self_id) { // check wether self_id has sharing abilty
            if !vec.contains(&editor) {
                vec.push(editor);
                hm.insert(doc, vec);
            }
            put("pwd.pwd".to_string(), hashmap2pwdstr(hm));
            return true
        }
    }
    return false //false
}

// delete editor 
#[marine]
fn del_editor(doc:String, editor: String,   self_id: String) -> bool {
       /*
    args: 
     - self_id: the self.id of already with editor access 
     - editor: the self.id to be granted access
     - doc: the file name of the document

    out: 
     - true if document exists, editing permission will be granted to editor
     - false if doc doesn't exist!  => create a document first 
    */
    let mut hm = get_pwd_hm();
    if hm.contains_key(&doc) { // if document exists, grant editor permission 
        let mut vec = hm.get(&doc).unwrap().to_vec();
        if vec.contains(&self_id) { // check wether self_id has sharing abilty
            if let Some(pos) = vec.iter().position(|x| *x == editor) {
                vec.remove(pos);
                hm.insert(doc, vec);
                put("pwd.pwd".to_string(), hashmap2pwdstr(hm));
            }
            return true
        }
    }
    return false //false
}


#[marine]
fn read_file(filename: String, editor: String) -> String {
    let fname = filename.clone();
    if authenticate(fname, editor){
        return get(filename.clone());
    }
    String::from("No access to read file or filed doesn't exits.")
}

#[marine]
fn write_file(filename: String, editor: String, content: String) -> String {
    let fname = filename.clone();
    if authenticate(fname, editor){
        put(filename, content);
        return String::from("File written");
    }
    String::from("No access to write file or file doesn't exist.")
}


#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;
    

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")] 
    fn test_create_doc(shared_doc: marine_test_env::shared_doc::ModuleInterface) {  
        // test creating document 
        let doc1 = "doc1.txt".to_string();
        let doc2 = "doc2.txt".to_string();
        let doc3 = "doc3.txt".to_string();
        let owner = "o1".to_string();
        let editor1 = "e1".to_string();
        let editor2 = "e2".to_string();
        let editor3 = "e3".to_string();
        assert_eq!( shared_doc.am_i_owner()     , true);
        let ownercp = owner.clone();
        assert_eq!( shared_doc.create_doc("doc1".to_string(), "o1".to_string()) , true);
        assert_eq!( shared_doc.create_doc("doc1".to_string(), "o1".to_string()) , false);
        // assert_eq!( shared_doc.create_doc(doc2, owner),  true);
        // assert_eq!( shared_doc.create_doc(doc2, owner), false);

        // test permission 

        // assert_eq!( shared_doc.add_editor(doc1, editor1, owner), true) ;
        // assert_eq!( shared_doc.add_editor(doc2, editor2, owner), true) ;

        // //non-exisiting doc 
        // assert_eq!( shared_doc.add_editor(doc3, editor3, owner), false);
        // // no permission
        // assert_eq!( shared_doc.add_editor(doc1, editor1, editor2), false) ;
        // // no 
        // assert_eq!( shared_doc.add_editor(doc1, editor3, editor1), true) ;
        // assert_eq!( shared_doc.add_editor(doc1, editor2, editor3), true) ;


        // test read/write
    }


}




