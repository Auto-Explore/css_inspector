# css/css-pseudo/marker-content-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-003-ref.html"
}
```

## style[0]

```css

li {
  list-style-type: 'X';
  background: grey;
}
li::marker {
  padding-right: 10px;
}
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
