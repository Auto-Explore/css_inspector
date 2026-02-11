# css/css-pseudo/marker-content-018.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-018.html"
}
```

## style[0]

```css

ol {
  float: left;
  width: 100px;
}
.inside {
  list-style-position: inside;
}
li:nth-child(1)::marker { content: "1" }
li:nth-child(2)::marker { content: "1 " }
li:nth-child(3)::marker { content: "1  " }
li:nth-child(4)::marker { content: " 1" }
li:nth-child(5)::marker { content: "  1" }
li:nth-child(6)::marker { content: "  1  " }
li:nth-child(7)::marker { content: "1\9 2" }
li:nth-child(8)::marker { content: "1\a 2" }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
