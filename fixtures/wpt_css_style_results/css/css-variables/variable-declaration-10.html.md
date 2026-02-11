# css/css-variables/variable-declaration-10.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-10.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: orange;
  --b: green;
  --a: var(--b,/**/a);
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
