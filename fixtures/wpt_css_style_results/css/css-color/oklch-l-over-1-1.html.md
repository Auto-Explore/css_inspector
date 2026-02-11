# css/css-color/oklch-l-over-1-1.html

```json
{
  "format_version": 3,
  "file": "css/css-color/oklch-l-over-1-1.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px}
    .ref { background-color: oklch(1 0.5 50); height: 100px}
    /* l = 1.5 should clamp back to 1 */
    .test { background-color: oklch(1.5 0.5 50); height: 100px}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
