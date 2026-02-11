# css/css-pseudo/marker-line-break.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-line-break.html"
}
```

## style[0]

```css

ol {
  float: left;
  width: 50px;
}
li {
  list-style-position: inside;
  width: 0;
}
.line-break-strict.explicit ::marker,
.line-break-strict.inherit {
  line-break: strict;
}
.line-break-anywhere.explicit ::marker,
.line-break-anywhere.inherit {
  line-break: anywhere;
}
.marker-disc {
  list-style-type: disc;
}
.marker-decimal {
  list-style-type: decimal;
}
.marker-string {
  list-style-type: "ab";
}
.marker-content::marker {
  content: "cd";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
