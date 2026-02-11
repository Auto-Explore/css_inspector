# css/css-pseudo/marker-letter-spacing.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-letter-spacing.html"
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
.letter-spacing.explicit ::marker,
.letter-spacing.inherit {
  letter-spacing: 25px;
}
.marker-disc {
  list-style-type: disc;
}
.marker-decimal {
  list-style-type: decimal;
}
.marker-string {
  list-style-type: "Xp";
}
.marker-content::marker {
  content: "Xpp";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
