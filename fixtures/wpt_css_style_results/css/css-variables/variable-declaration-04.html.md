# css/css-variables/variable-declaration-04.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-04.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: var(--b) ;
  --b: green;
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
