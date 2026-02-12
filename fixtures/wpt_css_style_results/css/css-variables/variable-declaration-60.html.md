# css/css-variables/variable-declaration-60.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-60.html"
}
```

## style[0]

```css

div {
  color: orange;
  --a: green;
  color: var(--a);
}
p {
  --b: İnitial;
  --c: ınitial;
  color: var(--b,var(--c,red));
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
