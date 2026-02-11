# css/selectors/is-where-pseudo-classes.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-pseudo-classes.html"
}
```

## style[0]

```css

  button {
    color: black;
  }
  /* Selects #a, #c */
  :is(main :where(main #a), #c:nth-child(odd), #d):is(:enabled) {
    color: green;
  }
  /* Selects #b, #d, #f */
  button:is(:nth-child(even), span #e):is(:enabled, :where(:disabled)) {
    color: blue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
