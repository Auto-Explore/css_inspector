# css/css-anchor-position/position-try-fallbacks-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-fallbacks-crash.html"
}
```

## style[0]

```css

.abs-cb {
  position: relative;
  width: 0;
  height: 0;
}

[abs] {
  position: absolute;
  position-anchor: auto;
}

.c {
  width: 1px;
  height: 1px;
  margin-right: anchor-size(width);
  position-try-fallbacks: top left, --position-fallback_1, flip-x;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
