# css/css-overflow/overflow-clip-margin-007.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-007.html"
}
```

## style[0]

```css

  body {
    height: 15000px;
  }
  .auto {
    content-visibility: auto;
    width: 100px;
    height: 100px;
    overflow-clip-margin: 10000px;
    background: lightblue;
  }
  .big {
    width: 10px;
    height: 20000px;
    position: relative;
    top: -10000px;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
