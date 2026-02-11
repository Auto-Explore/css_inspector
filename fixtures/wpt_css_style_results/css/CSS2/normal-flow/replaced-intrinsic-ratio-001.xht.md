# css/CSS2/normal-flow/replaced-intrinsic-ratio-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/replaced-intrinsic-ratio-001.xht"
}
```

## style[0]

```css

   body { width: 15em; border: silver dashed 1px; }
   table { border-spacing: 0; }
   td { padding: 0; }
   p, table { height: 1em; line-height: 1em; margin: 6em 0; }

   /* basic tests for inline and block */
   #img1 { margin-top: -1em; }
   #img2 { display: block; }

   /* shrinkwrapped */
   #p3 { width: 100%; float: left; margin: 0; }
   #t4 { width: 15em; display: table-cell; }
   #t5 { width: 100%; }

   /* controls */
   object { background: red; }
   object, .control { border: blue solid 1em; margin: 0 1em; }

   .control { background: green; }
   .inst { height: auto; margin: 1em 0; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
