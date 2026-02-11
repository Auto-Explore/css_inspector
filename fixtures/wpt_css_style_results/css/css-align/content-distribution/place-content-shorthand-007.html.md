# css/css-align/content-distribution/place-content-shorthand-007.html

```json
{
  "format_version": 3,
  "file": "css/css-align/content-distribution/place-content-shorthand-007.html"
}
```

## style[0]

```css

div {
  display: grid;
  grid: 200px / 200px;
  width: 400px;
  height: 400px;
  background: blue;
  place-content: end space-evenly;
}
span {
  background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “place-content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
