# css/CSS2/floats/crashtests/float-dynamic-change-6.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/crashtests/float-dynamic-change-6.html"
}
```

## style[0]

```css

.placeholder:first-line { }
.inner {
  float: inherit;
}
.inner::after {
  position: absolute;
  float: inherit;
  content: open-quote;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
