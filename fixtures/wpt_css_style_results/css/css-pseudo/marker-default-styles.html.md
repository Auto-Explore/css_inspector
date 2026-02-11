# css/css-pseudo/marker-default-styles.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-default-styles.html"
}
```

## style[0]

```css

.outside { list-style-position: outside }
.inside { list-style-position: inside }
.symbol { list-style-type: symbol }
.decimal { list-style-type: decimal }
.string { list-style-type: "string" }
.marker::marker { content: "marker" }
li {
  /* These inheritable properties should not be inherited by ::marker */
  text-transform: lowercase;
  text-indent: 1px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
