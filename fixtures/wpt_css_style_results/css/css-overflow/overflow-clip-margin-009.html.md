# css/css-overflow/overflow-clip-margin-009.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-009.html"
}
```

## style[0]

```css

.prop {
  overflow-clip-margin: 20px;
}
.container {
  width: 100px;
  height: 100px;
  overflow-clip-margin: inherit;
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
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 2
}
```
