# css/css-values/chrome-typed-arithmetic-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-values/chrome-typed-arithmetic-crash.html"
}
```

## style[0]

```css

div {
  height: calc(clamp(40px, 1px * 1px / 1px, 50px) + 60px);
  width: calc(clamp(40px, 1px * 1rem / 1px, 50px) + 60px);
  margin-left: calc(1% + 1% * (1vw - 1px) / 1px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
