# css/css-view-transitions/scoped/scope-during-transition-2.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/scope-during-transition-2.html"
}
```

## style[0]

```css

  #outer {
    box-sizing: border-box;
    border: 5px solid black;
    height: 300px;
    width: 300px;
    position: relative;
    view-transition-name: outer;
    z-index: 0;
  }

  #inner {
    box-sizing: border-box;
    border: 5px solid black;
    height: 100px;
    width: 100px;
    position: absolute;
    left: 100px;
    top: 100px;
    view-transition-name: inner;
    z-index: 1;
  }

  ::view-transition-group(*) {
    animation-timing-function: steps(1, jump-start);
    animation-play-state: paused;
  }

  #outer::view-transition {
    background-color: green;
  }

  #inner::view-transition {
    background-color: rebeccapurple;
  }

  /*
   * This rule should not apply since there will be an active view-transition on
   * inner, when the view-transition on outer starts.
   */
  #outer::view-transition-group(inner) {
    background-color: red;
  }

```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
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
    },
    {
      "message": "Invalid value for property “background-color”.",
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
