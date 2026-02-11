# css/selectors/invalidation/not-pseudo-containing-sibling-relationship-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/not-pseudo-containing-sibling-relationship-in-has.html"
}
```

## style[0]

```css

  #test-container > div { background-color: green; }
  #target1:has(:not(.item, :nth-child(3))) { background-color: red; }
  #target2:has(:not(.item, :nth-last-child(3))) { background-color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
