# css/selectors/invalidation/target-pseudo-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/target-pseudo-in-has.html"
}
```

## style[0]

```css

  #parent1 { color: grey; }
  #parent1:has(:target) { color: green; }

  #parent2 { color: blue; }
  #parent2:has(:not(:target)) { color: grey; }
  #parent2:has(:target) { color: green; }

  #parent3 { color: green; }
  #parent3:not(:has(:target)) { color: grey; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
