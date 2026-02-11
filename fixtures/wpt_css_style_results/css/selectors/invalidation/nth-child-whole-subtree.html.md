# css/selectors/invalidation/nth-child-whole-subtree.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/nth-child-whole-subtree.html"
}
```

## style[0]

```css

  div:nth-child(odd of :not(.c)) {
    background-color: silver;
  }
  .c * {}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
