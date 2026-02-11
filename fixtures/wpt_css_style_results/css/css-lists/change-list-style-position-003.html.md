# css/css-lists/change-list-style-position-003.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/change-list-style-position-003.html"
}
```

## style[0]

```css

div {
  border: 5px solid orange;
  display: list-item;
  margin-left: 40px;
}
div > div {
  list-style-type: decimal;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
