# css/selectors/not-default-ns-002.html

```json
{
  "format_version": 3,
  "file": "css/selectors/not-default-ns-002.html"
}
```

## style[0]

```css

@namespace url("http://www.w3.org/2000/svg");

*|input {
  display: none;
}

/* No type selector, so selector should _not_ match and keep the input hidden */
*|input:not(:disabled) {
  display: initial;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
