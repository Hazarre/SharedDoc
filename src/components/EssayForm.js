import React from 'react';

class EssayForm extends React.Component {
    constructor(props) {
      super(props);
      this.state = {
        value: ''
      };
  
      this.handleChange = this.handleChange.bind(this);
      this.handleSubmit = this.handleSubmit.bind(this);
    }
  
    handleChange(event) {
      this.setState({value: event.target.value});
    }
  
    handleSubmit(event) {
      console.log(this.state.value);
      
      alert('An essay was submitted: ' + this.state.value);
      event.preventDefault();
    }

    // function showTextFile(){

    // }


    render() {
      return (
        <form onSubmit={this.handleSubmit} className={"essayForm"}   >
    
            <textarea value={this.state.value} onChange={this.handleChange} 
              style={{height: "400px"}}/>
          
          <input type="submit" value="Save File" />
          <input type="submit" value="Read File" />
        </form>
      );
    }
  }

  export default EssayForm;