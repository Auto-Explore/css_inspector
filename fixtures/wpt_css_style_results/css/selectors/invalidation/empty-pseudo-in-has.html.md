# css/selectors/invalidation/empty-pseudo-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/empty-pseudo-in-has.html"
}
```

## style[0]

```css

  #subject { color: red }
  #subject:has(:empty) { color: green }
  #subject:has(:not(:empty)) { color: blue }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
