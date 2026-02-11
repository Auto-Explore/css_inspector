# css/CSS2/positioning/abspos-overflow-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/abspos-overflow-012.xht"
}
```

## style[0]

```css

   #outer {
    position: relative;
    height: 1em;
    width: 20em;
    background: red;
   }
   #inner {
    overflow: auto; /* this rule should be irrelevant since the element is empty and thus has zero height */
   }
   #positioned {
    position: absolute; /* relative to the outer div */
    top: 0;
    width: 20em;
    background: white;
    color: green;
   }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
