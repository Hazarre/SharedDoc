# SharedDoc with Fluence and Self.ID
Create a shared document app with [Fluence services](https://doc.fluence.dev/docs/), [Self.ID](https://developers.ceramic.network/reference/self-id/classes/web.SelfID/#authenticate) and React. Users can create, share and share files as well as adjust editing permission. A GUI in the browser can be used for ease of access.

### Youtube Demo
https://youtu.be/IpWqufnPs58

![alt text](https://github.com/Hazarre/SharedDoc/blob/main/demo.png)

### Client 
The clients of this app have two roles: the owner and editors. 

The owner (deployer identified by self.ID) of a SharedDoc service instance can create documents, add editors to them and share them with others. The owner shares a file to editors using their self.ID. 

Once a file is shared with a self.ID. It becomes a editor with power to read/write the document as well as share it with other self.IDs. First the verify their identity with self.ID in their browser. Then they use their self.ID to authenticate their permission to read/write/share files. 


### Running the App 
Download this directory. Inside this directory, [connect to a ceramic node](https://developers.ceramic.network/run/nodes/community-nodes/) and run ```npm install``` then```npm start```. You also need a metamask account for easy of use.
To deploy your own version of the app, see sections below. 

### Fluence services 
Deployed at:
```
peer id 
12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn

service id
829128d8-3641-49eb-8f77-e3df5d04f808
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

Marine implementations can be found at ```marine/shared_doc```. Run ```./deploy.sh``` to deplot to a Fluence node. 

React app have config files in the main directory and files in ```src/```

#### Future functoinalities: 
 - Sharing with fine grained permission. 
 - Better UI and UX. 
 - Neater marine/rust code. 

