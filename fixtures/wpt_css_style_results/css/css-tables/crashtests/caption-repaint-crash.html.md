# css/css-tables/crashtests/caption-repaint-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/crashtests/caption-repaint-crash.html"
}
```

## style[0]

```css

*:defined {
  outline: currentColor dashed;
}
*:read-write {
  animation: kf 200ms ease-out 16384 alternate-reverse backwards
}
@keyframes kf {
  20% {
    clip-path: polygon(1px 0.75em, 128px 100%)
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
