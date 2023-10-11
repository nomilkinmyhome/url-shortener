import './App.css';

import { useState } from 'react';

function App() {
  const [ urlInput, setUrlInput ] = useState("");
  const [ shortedURL, setShortedURL ] = useState("");

  function submitHandler(event) {
      fetch(
          'http://localhost:8000/?url=' + urlInput,
          {method: 'GET'}
      ).then(r => {
          return r.json();
      }).then((data) => {
          setShortedURL(data.url);
      });
  }

  return (
    <div className="App">
        <h2>URL Shortener</h2>
        <div className="container">
            <form>
                <input type="text" placeholder="Enter URL" value={ urlInput } onChange={ setUrlInput.target.value } />
                <button onClick={ submitHandler }>Short!</button>
            </form>
            { shortedURL ? <a href={ shortedURL }>{ shortedURL }</a> : "" }
        </div>
    </div>
  );
}

export default App;
