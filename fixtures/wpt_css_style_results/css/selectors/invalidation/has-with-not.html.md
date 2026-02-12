# css/selectors/invalidation/has-with-not.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-not.html"
}
```

## style[0]

```css

div, main { color: grey }
#subject:has(:not(.test)) { color: green }
#subject:has(.test :not(.test)) { color: red }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
