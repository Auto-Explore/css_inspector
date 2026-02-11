# css/css-view-transitions/scoped/capture.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/capture.html"
}
```

## style[0]

```css

  .parent {
    background-color: lightblue;
    height: 200px;
    width: 200px;
    view-transition-name: parent;
    position: relative;
  }

  .child {
    background-color: blue;
    height: 100px;
    width: 100px;
    view-transition-name: child;
    position: absolute;
    left: 50px;
    top: 50px;
  }

  .sibling {
    background-color: gray;
    margin-top: 20px;
    height: 200px;
    width: 200px;
    view-transition-name: sibling;
  }

  /* view transition pseudos */
  ::view-transition-group(*),
  ::view-transition-image-pair(*) {
    animation: unset;
  }

  ::view-transition-old(*) {
    animation: -ua-view-transition-fade-out 1s paused;
  }

  ::view-transition-new(*) {
    animation: -ua-view-transition-fade-in 1s paused;
  }

```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
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
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
