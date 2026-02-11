# css/css-scrollbars/scrollbar-after-remove-resize.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-after-remove-resize.html"
}
```

## style[0]

```css

  #scroller {
    width: 300px;
    height: 300px;
    overflow-y: scroll;
    border: 1px solid black;
    resize: both;
  }

  #scroller::-webkit-scrollbar {
    background: pink;
  }
  #scroller::-webkit-scrollbar-thumb {
    background: orange;
  }

  #content {
    width: 100%;
    height: 150%;
    background: lightgray;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
