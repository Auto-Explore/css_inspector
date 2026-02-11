# css/css-overflow/overflow-clip-margin-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-invalidation.html"
}
```

## style[0]

```css

  .container {
      display: flex;
  }
  .parent {
      flex: none;
      width: 100px;
      height: 100px;
      flex: none;
      overflow: clip;
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
  .margin {
    overflow-clip-margin: 10px;
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
