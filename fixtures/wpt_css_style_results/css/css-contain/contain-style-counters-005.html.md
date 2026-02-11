# css/css-contain/contain-style-counters-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-counters-005.html"
}
```

## style[0]

```css

.contain {
    float: right;
    contain: style;
    clear: both;
}
.reset { counter-reset: c;}
.increment:before { content: counters(c, ""); }
.increment { counter-increment: c; }
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
