# css/css-view-transitions/inline-child-with-overflow-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/inline-child-with-overflow-shadow-ref.html"
}
```

## style[0]

```css

body {
  background: rebeccapurple;
  margin: 0;
}

.target {
  width: 100px;
  height: 200px;
  position: absolute;
  top: 100px;
  left: 100px;
  view-transition-name: target;
}

.child {
  background: green;
  font-size: 100px;
  box-shadow: -20px -20px yellow;
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
