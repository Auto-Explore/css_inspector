# css/css-shadow/part/exportparts-multiple.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/exportparts-multiple.html"
}
```

## style[0]

```css

::part(inner-part) {
  color: red;
  background: blue;
  width: 100px;
  height: 100px;
}
::part(inner-part-alias) {
  color: green;
}
.change-1::part(inner-part) {
  background: green;
}

.change-2::part(inner-part-alias) {
  color: blue;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
