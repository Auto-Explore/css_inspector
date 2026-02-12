# css/CSS2/margin-padding-clear/margin-collapse-110.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-110.xht"
}
```

## style[0]

```css

   table { border-spacing: 0; font-size: 50px; border: solid 3px; background: orange; }
   td { padding: 0; }
   div { height: 1em; width: 1em; }
   .a { background: yellow; }
   .b { background: lime; }
   /* column 1 */
   .t1 .a { margin: 1em 0 1em; }
   .t1 .b { margin: 1em 0 1em; }
   /* column 2 */
   .t2 .a { margin: 1em 0 1em; }
   .t2 .b { margin: 0em 0 1em; }
   /* column 3 */
   .t3 .a { margin: 1em 0 0em; }
   .t3 .b { margin: 1em 0 1em; }
   /* column 4 */
   .t4 .a { margin: 1em 0 2em; }
   .t4 .b { margin: -1em 0 1em; }
   /* column 5 */
   .t5 .a { margin: 1em 0 -1em; }
   .t5 .b { margin: 2em 0 1em; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
