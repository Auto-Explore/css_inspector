# css/selectors/invalidation/crashtests/has-pseudoclass-only-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/crashtests/has-pseudoclass-only-crash.html"
}
```

## style[0]

```css

.anchor:has(:first-child + :last-child) {
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
