# css/css-scrollbars/scrollbar-color-009.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-009.html"
}
```

## style[0]

```css

  :root {
    scrollbar-color: yellow blue;
  }

  body {
    overflow: scroll;
  }

  ::-webkit-scrollbar-corner {
    background-color: purple;
  }
```

```json
{
  "errors": 2,
  "messages": [
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
