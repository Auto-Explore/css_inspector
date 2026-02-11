# css/css-pseudo/marker-content-013-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-013-ref.html"
}
```

## style[0]

```css

.inside {
  list-style-position: inside;
}
.string {
  list-style-type: "string";
}
.content::marker {
  content: "content";
}
.before, .after {
  display: contents;
}
.before::before, .after::after {
  content: "item";
  display: list-item;
  list-style-type: "nested";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
