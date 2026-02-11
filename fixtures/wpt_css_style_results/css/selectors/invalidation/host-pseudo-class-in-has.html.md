# css/selectors/invalidation/host-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/host-pseudo-class-in-has.html"
}
```

## style[0]

```css

      .subject {
        color: red;
      }
      .subject:has(:is(:host(.a) > .foo .bar)) { color: green }
      .subject:has(:is(:host(.a) .bar)) { color: blue }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
