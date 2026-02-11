# css/css-color/lab-l-over-100-1.html

```json
{
  "format_version": 3,
  "file": "css/css-color/lab-l-over-100-1.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px}
    .ref { background-color: lab(100 150 20); height: 100px}
    /* l = 150 should clamp back to 100 */
    .test { background-color: lab(150 150 20); height: 100px}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
