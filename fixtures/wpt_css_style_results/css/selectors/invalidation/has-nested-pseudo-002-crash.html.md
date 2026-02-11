# css/selectors/invalidation/has-nested-pseudo-002-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-nested-pseudo-002-crash.html"
}
```

## style[0]

```css

::part(foo) {:has(&){}}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
