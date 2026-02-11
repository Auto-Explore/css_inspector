# css/selectors/invalidation/negated-nth-child-when-ancestor-changes.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/negated-nth-child-when-ancestor-changes.html"
}
```

## style[0]

```css

.ancestor :not(:nth-child(even of .c)) {
  color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
