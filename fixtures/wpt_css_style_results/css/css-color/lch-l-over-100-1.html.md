# css/css-color/lch-l-over-100-1.html

```json
{
  "format_version": 3,
  "file": "css/css-color/lch-l-over-100-1.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px}
    .ref { background-color: lch(100 150 20); height: 100px}
    /* l = 150 should clamp back to 100 */
    .test { background-color: lch(150 150 20); height: 100px}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
