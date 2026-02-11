# css/css-pseudo/marker-content-005.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-005.html"
}
```

## style[0]

```css

li {
  counter-increment: foo 5;
}
li::marker {
  content: counters(list-item, ".") counter(foo, decimal-leading-zero);
}
span { font-size: 32pt; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
