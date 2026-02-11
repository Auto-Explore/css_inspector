# css/css-color/oklch-l-over-1-2.html

```json
{
  "format_version": 3,
  "file": "css/css-color/oklch-l-over-1-2.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px}
    .ref { background-color: oklch(100% 0.5 50); height: 100px}
    /* l = 150% should clamp back to 100% */
    .test { background-color: oklch(150% 0.5 50); height: 100px}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
