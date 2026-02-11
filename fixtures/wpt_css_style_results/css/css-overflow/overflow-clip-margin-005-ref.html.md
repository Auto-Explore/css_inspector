# css/css-overflow/overflow-clip-margin-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-005-ref.html"
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
  .child {
      position: relative;
      width: 110px;
      height: 110px;
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
    }
  ],
  "warnings": 0
}
```
