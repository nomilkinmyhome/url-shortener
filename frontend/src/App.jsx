import './App.css';

import { useState } from 'react';

function App() {
  const [ urlInput, setUrlInput ] = useState("");
  const [ shortedURL, setShortedURL ] = useState("");

  function shortUrlHandler(event) {
      fetch(
          'http://localhost:8000/?url=' + urlInput,
          {method: 'GET'}
      ).then(r => {
          console.log(r);
          return r.json();
      }).then((data) => {
          setShortedURL(data.shorted_url);
      });
  }

  return (
    <div className="App">
        <h2>URL Shortener</h2>
        <div className="container">
            <input type="text" placeholder="Enter URL" value={ urlInput } onChange={(event) => setUrlInput(event.target.value) } />
            <button onClick={ shortUrlHandler }>Short!</button>
            { shortedURL ? <a href={"http://localhost:8000/" + shortedURL }>http://localhost:8000/{ shortedURL }</a> : "" }
        </div>
    </div>
  );
}

export default App;
