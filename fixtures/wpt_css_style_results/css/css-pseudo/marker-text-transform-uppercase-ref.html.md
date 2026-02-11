# css/css-pseudo/marker-text-transform-uppercase-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-transform-uppercase-ref.html"
}
```

## style[0]

```css

.outside { list-style-position: outside }
.inside { list-style-position: inside }
.type-roman { list-style-type: upper-roman }
.type-string { list-style-type: "II. " }
.content-counter::marker { content: counter(list-item, upper-roman) ". " }
.content-string::marker { content: "IV. " }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
