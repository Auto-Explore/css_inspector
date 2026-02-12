# css/selectors/invalidation/open-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/open-pseudo-class-in-has.html"
}
```

## style[0]

```css

  #subject:has(#dialog:open) { color: green; }
  #subject:has(#details:open) { color: blue; }
  #subject:has(#select:open) { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
