# css/selectors/invalidation/negated-is-always-matches-negated-first-of-type-when-ancestor-changes.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/negated-is-always-matches-negated-first-of-type-when-ancestor-changes.html"
}
```

## style[0]

```css

.some-hidden > :not(:is(.always-matches, :not(:first-of-type))) {
  display: none;
}
.to-show {
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
