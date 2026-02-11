# css/css-viewport/zoom/text-decoration-thickness.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/text-decoration-thickness.html"
}
```

## style[0]

```css

.decoration-thickness {
  text-decoration: underline blue;
  text-decoration-thickness: 5px;
}
.zoom {
  zoom: 2;
}
#zoomed-inherited {
  text-decoration-thickness: inherit;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
