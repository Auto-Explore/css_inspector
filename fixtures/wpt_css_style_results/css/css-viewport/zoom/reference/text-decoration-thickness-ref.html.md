# css/css-viewport/zoom/reference/text-decoration-thickness-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/text-decoration-thickness-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
}
div {
  font-size: calc(1rem * var(--scale));
  text-decoration: underline blue;
  text-decoration-thickness: calc(5px * var(--scale));
}
.zoom {
  --scale: 2;
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
