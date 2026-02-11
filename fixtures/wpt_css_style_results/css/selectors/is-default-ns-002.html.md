# css/selectors/is-default-ns-002.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-default-ns-002.html"
}
```

## style[0]

```css

@namespace url("http://www.w3.org/2000/svg");

/* No type selector, so selector should match and hide the <input> */
*|*:is(:disabled) {
  display: none;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
