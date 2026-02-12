# css/css-variables/variable-declaration-45.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-45.html"
}
```

## style[0]

```css

body {
  --a: green;
  color: crimson;
}
p {
  color: red;
}
p {
  color: orange;
  --a: inherit;
  color: var(--a);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
