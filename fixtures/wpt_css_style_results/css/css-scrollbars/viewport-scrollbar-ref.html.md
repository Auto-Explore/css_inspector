# css/css-scrollbars/viewport-scrollbar-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/viewport-scrollbar-ref.html"
}
```

## style[0]

```css

#outer {
  border: 1px solid black;
  width: 200px; height: 200px;
  overflow: scroll;
  scrollbar-color: yellow blue;
}
#inner {
  width: 400px; height: 400px;
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
