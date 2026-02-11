# css/selectors/invalidation/host-context-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/host-context-pseudo-class-in-has.html"
}
```

## style[0]

```css

      .subject {
        color: red;
      }
      .subject:has(:is(:host-context(.a) > .foo .bar)) { color: green }
      .subject:has(:is(:host-context(.a) .bar)) { color: blue }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
