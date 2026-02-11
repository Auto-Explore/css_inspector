# css/CSS2/positioning/abspos-overflow-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/abspos-overflow-011.xht"
}
```

## style[0]

```css

   #outer {
    position: relative;
   }
   #inner {
    position: absolute; /* positioned relative to the containing block of the div that follows it */
    top: 0;
    background: white;
    color: green;
    width: 20em;
   }
   #content {
    overflow: auto; /* this should have no effect on the stacking context calculations or painting order */
    background: red;
    color: yellow;
    width: 20em;
   }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
