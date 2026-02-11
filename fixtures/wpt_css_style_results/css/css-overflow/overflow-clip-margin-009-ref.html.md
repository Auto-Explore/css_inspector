# css/css-overflow/overflow-clip-margin-009-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-009-ref.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  overflow-clip-margin: 20px;
  overflow: clip;
}
.child {
  width: 200px;
  height: 200px;
  background: lightblue;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
