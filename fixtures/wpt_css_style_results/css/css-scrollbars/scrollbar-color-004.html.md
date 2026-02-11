# css/css-scrollbars/scrollbar-color-004.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-004.html"
}
```

## style[0]

```css

  .container {
    overflow: auto;
    height: 200px;
    width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .content {
    height: 300px;
    width: 100%;
    background: lightsalmon;
  }

  body {
    scrollbar-color: yellow blue;
  }

  ::-webkit-scrollbar {
    display: none;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scrollbar-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
