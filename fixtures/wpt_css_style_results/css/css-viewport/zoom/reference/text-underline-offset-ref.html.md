# css/css-viewport/zoom/reference/text-underline-offset-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/text-underline-offset-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
}
div {
  font-size: calc(1rem * var(--scale));
  text-decoration: underline hotpink;
  text-underline-offset: calc(5px * var(--scale));
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
