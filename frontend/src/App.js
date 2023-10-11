import './App.css';

import { useRef, useState, useEffect } from 'react';

function App() {
  const urlInputRef = useRef();
  const [ shortedURL, setShortedURL ] = useState("");

  function submitHandler(event) {
      const url = urlInputRef.current.value;

      // eslint-disable-next-line react-hooks/rules-of-hooks
      useEffect(() => {
              fetch(
                  'http://localhost:8000/?url=' + url,
                  {method: 'GET'}
              ).then(r => {
                  return r.json();
              }).then((data) => {
                  setShortedURL(data.url);
              });
      }, [url]);
  }

  return (
    <div className="App">
        <h2>URL Shortener</h2>
        <div className="container">
            <form>
                <input type="text" placeholder="Enter URL" ref={urlInputRef} />
                <button onClick={ submitHandler }>Short!</button>
            </form>
            {/* eslint-disable-next-line jsx-a11y/anchor-has-content */}
            { shortedURL ? <a href={shortedURL}></a> : "" }
        </div>
    </div>
  );
}

export default App;
