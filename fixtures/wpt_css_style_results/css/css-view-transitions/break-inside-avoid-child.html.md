# css/css-view-transitions/break-inside-avoid-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/break-inside-avoid-child.html"
}
```

## style[0]

```css

.old {
  position: relative;
  top: 300px;
  left: 0;
  background: lightblue;
}
.new {
  position: relative;
  top: 0;
  left: 0;
  background: green;
}
.columns {
  columns: 2;
  contain: layout;
  border: 1px solid black;
  view-transition-name: target;
}

.inner {
  break-inside: avoid;
  width: 300px;
  height: 300px;
}

html::view-transition-group(root) { animation-duration: 300s; }
html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}
html::view-transition-new(target) {
  animation: unset;
  opacity: 0;
}
```

```json
{
  "errors": 5,
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
