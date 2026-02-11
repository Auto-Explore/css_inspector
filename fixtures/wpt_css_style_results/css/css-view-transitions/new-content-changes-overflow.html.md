# css/css-view-transitions/new-content-changes-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-changes-overflow.html"
}
```

## style[0]

```css

#target {
  position: relative;
  background: green;
  width: 100px;
  height: 100px;
  view-transition-name: target;
}
#child {
  background: blue;
  position: relative;
  top: 20px;
  left: 30px;
  width: 50px;
  height: 100px;
}
#child.large {
  height: 200px;
}

html::view-transition-group(*) { animation-duration: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 1; }
html::view-transition-old(*) { animation: unset; opacity: 0; }
html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: pink; }
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
