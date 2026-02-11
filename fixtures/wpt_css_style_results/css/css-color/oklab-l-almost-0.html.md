# css/css-color/oklab-l-almost-0.html

```json
{
  "format_version": 3,
  "file": "css/css-color/oklab-l-almost-0.html"
}
```

## style[0]

```css

    .square { border: 1px solid black; width: 200px; height: 200px; }
    .test { background-color: red; height: 100px; }
    /* Non-zero a/b is used to show if the result is black or not, but the
     * test passes as long as it's the same color. */
    .ref { background-color: oklab(0 0.15 0.15); height: 100px; }
    .test { background-color: oklab(0.0001% 0.15 0.15); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
