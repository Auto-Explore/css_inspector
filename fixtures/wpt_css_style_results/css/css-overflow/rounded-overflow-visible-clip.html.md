# css/css-overflow/rounded-overflow-visible-clip.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/rounded-overflow-visible-clip.html"
}
```

## style[0]

```css

.container {
  width: 50px;
  height: 100px;
  overflow: visible clip;
  background: red;
  display: inline-block;
}
.border-radius {
  border-radius: 25px;
}
.child {
  width: 100px;
  height: 200px;
  background: green;
  fill: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “overflow”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
