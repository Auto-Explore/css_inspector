# css/css-tables/collapsed-border-partial-invalidation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/collapsed-border-partial-invalidation-003.html"
}
```

## style[0]

```css

table {
  border-collapse: collapse;
}
tr {
  border: 1px solid grey;
}

.foo {
  border-right: 20px solid black;
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
