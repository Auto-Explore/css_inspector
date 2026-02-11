# css/selectors/invalidation/negated-never-matches-negated-last-of-type-when-ancestor-changes.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/negated-never-matches-negated-last-of-type-when-ancestor-changes.html"
}
```

## style[0]

```css

.some-hidden > :not(.never-matches:not(:last-of-type)) {
  display: none;
}
.to-hide {
  color: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
