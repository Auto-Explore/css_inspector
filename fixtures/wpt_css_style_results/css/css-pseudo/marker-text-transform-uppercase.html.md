# css/css-pseudo/marker-text-transform-uppercase.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-transform-uppercase.html"
}
```

## style[0]

```css

.outside { list-style-position: outside }
.inside { list-style-position: inside }
.type-roman { list-style-type: lower-roman }
.type-string { list-style-type: "ii. " }
.content-counter::marker { content: counter(list-item, lower-roman) ". " }
.content-string::marker { content: "iv. " }
::marker {
  text-transform: uppercase;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
