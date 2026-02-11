# css/css-variables/variable-declaration-16.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-16.html"
}
```

## style[0]

```css

body {
  font-family: serif;
}
p {
  font-family: monospace;
  --a: var(--b), sans-serif;
  --b: Ahem;
  font-family: var(--a);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
