# css/css-variables/variable-declaration-50.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-50.html"
}
```

## style[0]

```css

body {
  color: green;
}
p {
  color: crimson;
  --a: var(--b,red);
  --b: var(--c);
  --c: var(--d);
  --d: var(--e);
  --e: var(--a);
  --f: var(--e);
  color: var(--f);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
