# css/css-text-decor/crashtests/text-decoration-first-line-layer-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/crashtests/text-decoration-first-line-layer-crash.html"
}
```

## style[0]

```css

.c2 {
  float: right;
  text-decoration: overline;
}

.c2::first-letter {
  vertical-align: super;
  opacity: 0.1;
}

.c2::first-line {
  text-decoration: overline;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
