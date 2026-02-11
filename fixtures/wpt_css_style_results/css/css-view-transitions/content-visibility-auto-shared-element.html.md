# css/css-view-transitions/content-visibility-auto-shared-element.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-visibility-auto-shared-element.html"
}
```

## style[0]

```css

body {
  overflow: hidden;
}
.flex {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: flex-start;
}
.box {
  width: 100px;
  height: 500px;
  contain: paint;
}
.shared {
  background: green;
  border: 1px solid black;
  box-sizing: border-box;
}
.spacer {
  height: 3000px;
}
#hidden {
  width: 10px;
  height: 10px;
  background: red;
  contain: paint;
  view-transition-name: hidden;
}
.locked {
  content-visibility: auto;
  contain-intrinsic-size: 500px;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { visibility: hidden; }

html::view-transition-group(*) { animation-duration: 0s; }
html::view-transition-new(*) { animation: unset; opacity: 0; }
html::view-transition-old(*) { animation: unset; opacity: 1; }
html::view-transition-group(root) { display: none; }
html::view-transition { background: pink }

```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
