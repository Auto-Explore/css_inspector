# css/css-viewport/zoom/resources/nested-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/resources/nested-iframe.html"
}
```

## style[0]

```css

body {
  --top-scale: 1;
  --sub-scale: 1;
  margin: calc(12px * var(--top-scale));
  background-color: orange;
}
iframe {
  border: none;
  width: calc(96px * var(--top-scale) * var(--sub-scale));
  height: calc(48px * var(--top-scale) * var(--sub-scale));
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
