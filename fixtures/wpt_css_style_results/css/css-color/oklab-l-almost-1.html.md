# css/css-color/oklab-l-almost-1.html

```json
{
  "format_version": 3,
  "file": "css/css-color/oklab-l-almost-1.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px; }
    .test { background-color: red; height: 100px; }
    /* Non-zero a/b is used to show if the result is white or not, but the
     * test passes as long as it's the same color. */
    .ref { background-color: oklab(1 0.15 0.15); height: 100px; }
    .test { background-color: oklab(99.9999% 0.15 0.15); }
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
