# css/css-view-transitions/scoped/box-decoration-painting.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/box-decoration-painting.html"
}
```

## style[0]

```css

#target {
  position: relative;
  top: 30px;
  left: 30px;

  width: 200px;
  height: 300px;

  background: blue;
  border-width: 5px 10px 15px 20px;
  border-style: solid;
  border-color: yellow;

  box-shadow: 10px 5px 0px 20px black;
}

/* This is needed since we inherit from the scope and make the scope
   not visible in the VT callback
*/
::view-transition { visibility: visible; }

::view-transition-group(*) {
  animation-play-state: paused;
}
::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}
::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}
#target.after {
  visibility: hidden;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
