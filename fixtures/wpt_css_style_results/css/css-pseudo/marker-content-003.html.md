# css/css-pseudo/marker-content-003.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-003.html"
}
```

## style[0]

```css

li::marker {
  content: "X";
  display: grid;
  grid-template-columns: auto 10px;
}
li { background: grey; }
span { font-size: 32pt; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
