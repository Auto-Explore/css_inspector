# css/css-overflow/overflow-clip-margin-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-002.html"
}
```

## style[0]

```css

  .scroller {
      overflow: auto;
      width: 100px;
      height: 100px;
      /* Avoids some fuzz on scrollbar corners */
      scrollbar-color: blue blue;
  }
  .parent {
      width: 100px;
      height: 100px;
      overflow: clip;
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
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-color”.",
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
