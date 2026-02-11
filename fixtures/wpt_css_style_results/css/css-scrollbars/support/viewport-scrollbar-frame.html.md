# css/css-scrollbars/support/viewport-scrollbar-frame.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/support/viewport-scrollbar-frame.html"
}
```

## style[0]

```css

html {
  scrollbar-color: yellow blue;
}
html, body {
  margin: 0;
  padding: 0;
}
#inner {
  width: 400px;
  height: 400px;
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
