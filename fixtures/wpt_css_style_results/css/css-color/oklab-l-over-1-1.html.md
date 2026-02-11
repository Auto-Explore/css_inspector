# css/css-color/oklab-l-over-1-1.html

```json
{
  "format_version": 3,
  "file": "css/css-color/oklab-l-over-1-1.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px}
    .ref { background-color: oklab(1 0.5 0.2); height: 100px}
    /* l = 1.5 should clamp back to 1 */
    .test { background-color: oklab(1.5 0.5 0.2); height: 100px}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
