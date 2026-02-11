# css/CSS2/margin-padding-clear/margin-collapse-109.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-109.xht"
}
```

## style[0]

```css

   table { border-spacing: 0; font-size: 50px; border: solid white; background: red; }
   td { padding: 0; }

   /* colors */
   .test, .control { background: lime; }
   .a { background: aqua; color: aqua; }
   .b { background: yellow; color: yellow; }

   /* test */
   .test .a { margin: 0 0 1em 0; }
   .test .c { margin: 1em 0 1em 0;
              padding-bottom: 0.04px; }
   .test .b { margin: 1em 0 0 0; }

   /* control */
   .control .a { margin: 0 0 2em; }
   .control .c { padding-bottom: 0.04px; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
