# css/css-overflow/overflow-clip-margin-004.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-004.html"
}
```

## style[0]

```css

  .container {
      display: flex;
  }
  .parent {
      width: 100px;
      height: 100px;
      flex: none;
      contain: paint;
      overflow-clip-margin: 10px;
  }
  .child {
      width: 200px;
      height: 200px;
      position: relative;
      top: -50px;
      left: -50px;
      background-color: green;
  }
  .spacer {
      flex: none;
      height: 100px;
      width: 100px;
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
