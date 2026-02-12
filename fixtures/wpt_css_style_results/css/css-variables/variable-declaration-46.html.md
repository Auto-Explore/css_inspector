# css/css-variables/variable-declaration-46.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-46.html"
}
```

## style[0]

```css

body {
  --a: crimson;
}
p {
  color: red;
}
p {
  color: orange;
  --a: initial;
  color: var(--a,green);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
