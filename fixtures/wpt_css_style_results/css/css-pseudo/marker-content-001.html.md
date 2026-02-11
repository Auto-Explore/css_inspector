# css/css-pseudo/marker-content-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-001.html"
}
```

## style[0]

```css

li::marker {
  content: "1.\00a0";
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
