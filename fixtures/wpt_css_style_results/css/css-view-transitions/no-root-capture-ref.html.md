# css/css-view-transitions/no-root-capture-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/no-root-capture-ref.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  contain: paint;
  background: blue;
  overflow-clip-margin: 50px;
  view-transition-name: target;
}
.child {
  width: 200px;
  height: 200px;
  position: relative;
  top: 50px;
  left: 50px;
  background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
