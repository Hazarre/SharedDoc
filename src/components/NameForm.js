import React from "react";
class NameForm extends React.Component {
    constructor(props) {
      super(props);
      this.state = {value: ''};
  
      this.handleChange = this.handleChange.bind(this);
      this.handleSubmit = this.handleSubmit.bind(this);
    }
  
    handleChange(event) {
      this.setState({value: event.target.value});
    }
  
    handleSubmit(event) {
      alert('A name was submitted: ' + this.state.value);
      event.preventDefault();
    }
  
    render() {
      return (
        <form onSubmit={this.handleSubmit}>
          <label>Filename/Editorname
            <input type="text" value={this.state.value} onChange={this.handleChange} />
          </label> 
          

          <input type="submit" value="read" />
          <input type="submit" value="save" />
          <input type="submit" value="add editor" />
          <input type="submit" value="remove editor" />
        </form>
      );
    }
  }

  export default NameForm;