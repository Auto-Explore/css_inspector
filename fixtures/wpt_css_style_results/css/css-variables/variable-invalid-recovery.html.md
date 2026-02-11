# css/css-variables/variable-invalid-recovery.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-invalid-recovery.html"
}
```

## style[0]

```css

p {
  color: red;
  transform: scale(var(--#invalid));
}
p {
  color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
