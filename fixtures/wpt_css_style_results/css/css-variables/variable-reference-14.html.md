# css/css-variables/variable-reference-14.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-14.html"
}
```

## style[0]

```css

#a {
  --a: green !important;
}
p {
  color: red;
  --a: crimson;
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
