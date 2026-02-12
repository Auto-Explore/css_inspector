# css/CSS2/margin-padding-clear/margin-collapse-112.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-112.xht"
}
```

## style[0]

```css

   table { border-spacing: 0; font: 50px/1 Ahem; border: solid 3px; background: red; }
   td { padding: 0; }
   /* test */
   .test { background: orange; }
   .test div { margin: 1em 0; background: red; height: 1em; width: 1em; }
   .test .a { background: yellow; }
   .test .b { background: lime; }
   /* control */
   .control div { width: 1em; }
   .control .c { border-top: 1em orange solid; }
   .control .a { border-top: 1em yellow solid; }
   .control .b { border-top: 1em lime solid; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
