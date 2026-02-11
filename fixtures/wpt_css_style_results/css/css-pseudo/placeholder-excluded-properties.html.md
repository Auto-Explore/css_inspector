# css/css-pseudo/placeholder-excluded-properties.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/placeholder-excluded-properties.html"
}
```

## style[0]

```css

.horizontal::placeholder {
  writing-mode: vertical-rl;
  direction: rtl;
}

.vertical {
  writing-mode: vertical-rl;
}
.vertical::placeholder {
  text-orientation: upright;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
