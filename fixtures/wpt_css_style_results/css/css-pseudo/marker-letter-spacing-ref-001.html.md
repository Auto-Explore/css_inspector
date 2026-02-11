# css/css-pseudo/marker-letter-spacing-ref-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-letter-spacing-ref-001.html"
}
```

## style[0]

```css

div {
  float: left;
  font: 25px/1 Ahem;
}
.inside {
  list-style-position: inside;
  width: 250px;
}
.outside {
  list-style-position: outside;
  width: 100px;
  margin-left: 150px;
}
ol {
  padding: 0;
}
.marker-disc {
  list-style-type: disc;
}
.marker-decimal {
  list-style-type: "2 . \A0 ";
}
.marker-string {
  list-style-type: "X p";
}
.marker-content::marker {
  content: "X p p";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
