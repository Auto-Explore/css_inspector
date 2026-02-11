# css/css-overflow/overflow-clip-margin-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-003.html"
}
```

## style[0]

```css

  .parent {
      width: 100px;
      height: 100px;
      background-color: green;
      overflow: clip;
      overflow-clip-margin: 1px;
      box-shadow: 20px 20px 5px red;
  }
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```
