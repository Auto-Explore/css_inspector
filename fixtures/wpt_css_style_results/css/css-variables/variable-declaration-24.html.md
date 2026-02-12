# css/css-variables/variable-declaration-24.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-24.html"
}
```

## style[0]

```css

body {
  color: green;
}
p {
  color: red;
}
p {
  color: orange;
  --a: red;
  --b: crimson;
  --a: var(--b) <!--; /* valid at parse */
  color: var(--a);    /* but IACVT at substitution */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
