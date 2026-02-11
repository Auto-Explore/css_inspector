# css/css-variables/variable-declaration-14.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-14.html"
}
```

## style[0]

```css

p {
  color: green;
}
span {
  color: red;
  --a:var(--b)red;
  --b:orange;
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
