# css/css-view-transitions/capture-with-visibility-mixed-descendants-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/capture-with-visibility-mixed-descendants-ref.html"
}
```

## style[0]

```css

:root {
  scrollbar-width: none;
}
body {
  background: pink;
}
.target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  background: blue;
}
.invisible {
  top: 200px;
  left: 200px;
  width: 500px;
  height: 500px;
  position: relative;
  background: red;
  visibility: hidden;
}
.visible {
  background: green;
  width: 10px;
  height: 10px;
  visibility: visible;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
