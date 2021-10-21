import React, { useState }  from 'react';


// import { Text } from "react-native";

import EssayForm from './components/EssayForm';
import NameForm from './components/NameForm';
import {Button, Row, Col, Container} from 'react-bootstrap';

import CeramicClient from '@ceramicnetwork/http-client'
import KeyDidResolver from 'key-did-resolver'
import ThreeIdResolver from '@ceramicnetwork/3id-did-resolver'
import { DID } from 'dids'
import { EthereumAuthProvider, SelfID } from '@self.id/web'
declare let window: any;

function App() {
    // create did instance 
    const API_URL = "https://localhost:7007"
    const ceramic = new CeramicClient(API_URL)
    const resolver = { ...KeyDidResolver.getResolver(),
                      ...ThreeIdResolver.getResolver(ceramic) }
    const did = new DID({ resolver })
    ceramic.did = did;

    async function getselfID(){
      // The following assumes there is an injected `window.ethereum` provider
      const addresses = await window.ethereum.enable()
      const authProvider = new EthereumAuthProvider(window.ethereum, addresses[0])
      
      // The following configuration assumes your local node is connected to the Clay testnet
      const self = await SelfID.authenticate({
        authProvider: new EthereumAuthProvider(window.ethereum, addresses[0]),
        ceramic: 'local',
        connectNetwork: 'testnet-clay',
      })
      console.log(self.id);
      return self.id;
    }


    const [selfId, setSelfId] = useState("self.ID not retreived yet!");
    const [fname, setFname] = useState("");
    const [ename, setEname] = useState("");


    const onIdClick = () =>{
        getselfID().then( 
        (value) => {
            setSelfId(value);
        });
    }

    
    function onFnameChange(){
      setFname("Fname");
    }

    function onAction(){
      console.log("take actoin")
    }
    
    
  return (

    
<div className="App">


<Container>

  <Row> 
    <Col>
      <Row> 
        <Button onClick={onIdClick}> Get self.ID </Button>  
      </Row>
      <Row>
        <h3> Self.ID:  {selfId} </h3>
      </Row>
      <Row>
        <form onSubmit={onAction}>
          <label> Doc Name <br></br>
            <input type="text" value={fname} onChange={onFnameChange} />
            
          </label> 
          <label> Editor <br></br>
            <input type="text" value={fname} onChange={onFnameChange} />
          </label> <br></br>

          <label> Me <br></br>
            <input type="text" value={fname} onChange={onFnameChange} />
          </label> <br></br>
          
          

          <input type="submit" value="read" />
          <input type="submit" value="save" />
          <input type="submit" value="add editor" />
          <input type="submit" value="remove editor" />
        </form>
      </Row>

    </Col>

    <Col>       
      <EssayForm />
    </Col>
  </Row>


  
</Container>



        
        
     
</div>);
}

export default App;

