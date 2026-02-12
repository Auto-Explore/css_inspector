# css/css-cascade/scope-has-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-has-crash.html"
}
```

## style[0]

```css

@scope (:not(:has(:nth-child(2 of .category)))) {
    .category {
      grid-column:span 2
    }
}

@scope (.category-wrap:has(:nth-child(2 of .category))) {
  /* Non-matching @media-rule, etc */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
