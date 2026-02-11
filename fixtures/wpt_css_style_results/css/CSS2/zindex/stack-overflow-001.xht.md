# css/CSS2/zindex/stack-overflow-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/stack-overflow-001.xht"
}
```

## style[0]

```css

   div { border: solid; margin: 1em; height: 5em; width: 5em; }
   div p { margin: 0 0 -5em 0; height: 5em; width: 5em; }
   .a .before { background: maroon; color: white; }
   .a .scroll { background: red; overflow: scroll; }
   .a .after { background: lime; }
   .b .before { background: maroon; }
   .b .scroll { background: red; color: white; overflow: scroll; }
   .b .after { background: lime; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
