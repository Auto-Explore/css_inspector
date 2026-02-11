# css/css-pseudo/marker-computed-content.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-computed-content.html"
}
```

## style[0]

```css

.no-list > li {
  display: block;
}
.normal::marker {
  content: normal;
}
.string::marker {
  content: "string";
}
.image::marker {
  content: url("about:invalid");
}
.none::marker {
  content: none;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
