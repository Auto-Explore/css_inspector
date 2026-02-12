# css/css-variables/variable-declaration-58.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-58.html"
}
```

## style[0]

```css

body {
  --b: green;
  color: crimson;
}
p {
  color: red;
}
p {
  color: orange;
  --a: var(--b);
  --b: unset;
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
