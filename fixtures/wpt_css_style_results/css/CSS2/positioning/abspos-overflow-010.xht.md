# css/CSS2/positioning/abspos-overflow-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/abspos-overflow-010.xht"
}
```

## style[0]

```css

   #outer {
    overflow: auto; /* this should have no visible effect as the element sizes vertically to fit its contents */
    color: green;
   }
   #inner {
    position: absolute; /* relative to the viewport, not that it matters */
    z-index: -1; /* this should have no effect, as the element is fully transparent anyway */
   }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
