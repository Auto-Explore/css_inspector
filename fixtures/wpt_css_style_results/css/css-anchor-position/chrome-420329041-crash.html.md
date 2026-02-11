# css/css-anchor-position/chrome-420329041-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-420329041-crash.html"
}
```

## style[0]

```css

  #crash {
    position: absolute;
    left: anchor(right);
    --svg: url("data:image/svg+xml,");
    content: var(--svg);
    fill: var(--svg);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
