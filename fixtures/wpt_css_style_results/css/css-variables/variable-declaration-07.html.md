# css/css-variables/variable-declaration-07.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-07.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: crimson;
  --b: green;
  --a: var(--b,);
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
