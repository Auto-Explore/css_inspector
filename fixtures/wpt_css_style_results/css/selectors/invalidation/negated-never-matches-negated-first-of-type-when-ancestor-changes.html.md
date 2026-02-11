# css/selectors/invalidation/negated-never-matches-negated-first-of-type-when-ancestor-changes.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/negated-never-matches-negated-first-of-type-when-ancestor-changes.html"
}
```

## style[0]

```css

.some-hidden > :not(.never-matches:not(:first-of-type)) {
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
