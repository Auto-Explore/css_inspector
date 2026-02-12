# css/css-pseudo/marker-text-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-shadow.html"
}
```

## style[0]

```css

ol {
  float: left;
  width: 50px;
  list-style-position: inside;
}
.text-shadow.explicit ::marker,
.text-shadow.inherit {
  text-shadow: #0f0 1px 2px 3px;
}
.marker-decimal {
  list-style-type: decimal;
}
.marker-string {
  list-style-type: "2. ";
}
.marker-content::marker {
  content: "3. ";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
