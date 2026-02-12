# css/css-variables/variable-declaration-57.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-57.html"
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
  --a: unset;
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
