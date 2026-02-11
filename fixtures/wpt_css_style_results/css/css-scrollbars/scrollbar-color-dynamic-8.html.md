# css/css-scrollbars/scrollbar-color-dynamic-8.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-dynamic-8.html"
}
```

## style[0]

```css

  :root::-webkit-scrollbar {
    background: pink;
  }
  :root::-webkit-scrollbar-thumb {
    background: orange;
  }

  body {
    display: flex;
    flex-wrap: wrap;
    width: 200vw;
    height: 200vh;
  }
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
