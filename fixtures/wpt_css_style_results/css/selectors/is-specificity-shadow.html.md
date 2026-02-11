# css/selectors/is-specificity-shadow.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-specificity-shadow.html"
}
```

## style[0]

```css

  main :not(:is(:host(#a))) { color: green; }
  main :not(:is(:host(.a))) { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
