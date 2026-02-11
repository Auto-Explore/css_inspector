# css/css-pseudo/marker-content-001c.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-001c.html"
}
```

## style[0]

```css

li::marker {
  content: "1.\00a0";
  display: block;
  margin: 0 auto 0 auto;
}
li:nth-of-type(2)::marker {
  content: "2.\00a0";
}
li:nth-child(3)::marker {
  content: "3.\00a0";
}
span { font-size: 32pt; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
