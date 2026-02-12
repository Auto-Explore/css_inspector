# css/css-overflow/overflow-alignment-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-alignment-001-ref.html"
}
```

## style[0]

```css

  /* Cram Tests */
  body { height: 600px; border-bottom: solid orange; } /* Reftest Max Size. Do not exceed this line. */
  html { font-size: 10px; }
  th, td { padding: 0; }

  /* Styling/Readability */
  abbr, th[scope=row] { font-variant: small-caps; text-transform: lowercase; color: gray; }
  thead { display: table-footer-group; }
  caption { font-weight: bold; caption-side: bottom; }
  /* Note: Annotations are at the bottom / right to avoid using up checked reftest area. */

  /* Create an overflowing box */
   .indicator {
      width: 72px;
      height: 72px;
      background: green;
   }

   /* Create a test box with appropriate scrollbars */
   .test {
      width: 24px;
      height: 24px;
      overflow: hidden;
   }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
