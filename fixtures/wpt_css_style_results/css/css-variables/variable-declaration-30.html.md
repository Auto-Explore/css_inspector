# css/css-variables/variable-declaration-30.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-30.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a: crimson;
  --a: var(--a);
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
