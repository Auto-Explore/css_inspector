# css/css-text-decor/crashtests/pseudo-element-text-decoration-under-pseudo-first-line-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/crashtests/pseudo-element-text-decoration-under-pseudo-first-line-crash.html"
}
```

## style[0]

```css

li {
  text-decoration: overline;
}
li::marker {
  content: 'm';
}
li:first-line {
  text-decoration: underline;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
