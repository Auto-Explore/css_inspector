# css/selectors/invalidation/has-with-nesting-parent-containing-hover.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-nesting-parent-containing-hover.html"
}
```

## style[0]

```css

  dd, dt { background: white; }
  dd:hover {
    dt:has(~ &) { background: lime; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
