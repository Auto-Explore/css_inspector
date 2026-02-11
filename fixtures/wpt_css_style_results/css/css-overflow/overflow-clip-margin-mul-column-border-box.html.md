# css/css-overflow/overflow-clip-margin-mul-column-border-box.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-mul-column-border-box.html"
}
```

## style[0]

```css

  body {
    width: 200px;
    height: 50px;
    column-count: 2;
  }

  .container {
    height: 50px;
    border: 5px solid grey;
    padding: 5px;
    overflow: clip;
    overflow-clip-margin: border-box;
  }

  .content {
    position: relative;
    top: -20px;
    left: -20px;
    width: 100px;
    height: 50px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```
