# css/css-view-transitions/named-element-with-fix-pos-child-old.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/named-element-with-fix-pos-child-old.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
}
.child {
  width: 100px;
  height: 100px;
  position: fixed;
  top: 150px;
  left: 150px;
  background: grey;
}

html::view-transition-group(target) { animation-duration: 300s; }
html::view-transition-new(target) { animation: unset; opacity: 0; }
html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
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
