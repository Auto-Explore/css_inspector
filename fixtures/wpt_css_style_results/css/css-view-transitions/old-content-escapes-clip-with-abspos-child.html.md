# css/css-view-transitions/old-content-escapes-clip-with-abspos-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/old-content-escapes-clip-with-abspos-child.html"
}
```

## style[0]

```css

#clipper {
  border-radius: 20px;
  overflow: hidden;
  width: max-content;
}
#target {
  width: 100px;
  height: 100px;
  background: lightblue;
  view-transition-name: target;
}
#abspos {
  position: absolute;
  left: 0;
  width: 50px;
  height: 50px;
  background: pink;
}

:root { view-transition-name: none }

::view-transition {
  background: rebeccapurple;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}
::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}
::view-transition-group(*) {
  animation-play-state: paused;
}
```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Invalid value for property “background”.",
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
