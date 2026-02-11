# css/selectors/invalidation/link-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/link-pseudo-class-in-has.html"
}
```

## style[0]

```css

    /* :any-link and :link should match similarly */
    #subject:has(#target:link:any-link) { color: green; }
    #subject:has(#svgTarget:link:any-link) { color: blue; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
