# css/css-cascade/revert-layer-015.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-015.html"
}
```

## style[0]

```css

@layer first {
    input::placeholder { background-color: green; }
}
@layer second {
    input::placeholder { background-color: revert-layer; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
