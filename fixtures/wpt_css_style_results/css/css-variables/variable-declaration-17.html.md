# css/css-variables/variable-declaration-17.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-17.html"
}
```

## style[0]

```css

body {
  font-family: serif;
}
p {
  font-family: monospace;
  --a: SomeUnknownFont, var(--b);
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
