# css/css-variables/variable-declaration-48.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-48.html"
}
```

## style[0]

```css

body {
  color: green;
}
p {
  color: crimson;
  --a: red var(--b);
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
