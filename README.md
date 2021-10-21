# FluenceHackFun
Create a shared document app with Fluence services, Self.ID and React. 

### Client 
The clients of this app have two roles: the owner and editors. 

The owner (deployer identified by self.ID) of a SharedDoc service instance can create documents, add editors to them and share them with others. The owner shares a file to editors using their self.ID. 

Once a file is shared with a self.ID. It becomes a editor with power to read/write the document as well as share it with other self.IDs. First the verify their identity with self.ID in their browser. Then they use their self.ID to authenticate their permission to read/write/share files. 


### Running the App 
Download this directory. Inside this directory, connect to a ceramic node and run ```npm start```. 
To deploy your own version of the app, see sections below. 

### Fluence services 
Deployed at:
```
peer id 
12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn

service id
8d123d97-8632-4ece-bdfa-1bc94ee45d99
```

```rust 
service SharedDoc:
  add_editor(doc: string, editor: string, self_id: string) -> bool
  am_i_owner() -> bool
  authenticate(filename: string, self_id: string) -> bool
  create_doc(filename: string, self_id: string) -> bool
  del_editor(doc: string, editor: string, self_id: string) -> bool
  read_file(filename: string, editor: string) -> string
  write_file(filename: string, editor: string, content: string) -> string
```

Aqua services can be found at ```aqua/shared_doc.aqua```. 
Marine implementations can be found at ```marine/shared_doc```.


#### Future functoinalities: 
 - Sharing with fine grained permission. 
 - Better UI and UX. 
 - Neater marine/rust code. 

