# css/css-pseudo/marker-intrinsic-contribution-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-intrinsic-contribution-001.html"
}
```

## style[0]

```css

li {
  font: 10px/1 Ahem;
  border: 5px solid orange;
  float: left;
  clear: left;
}
.inside {
  list-style-position: inside;
}
.symbol {
  list-style-type: disc;
}
.decimal {
  list-style-type: decimal;
}
.string {
  list-style-type: "string";
}
.content::marker {
  content: "content";
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
