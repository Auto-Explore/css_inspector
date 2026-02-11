# css/CSS2/zindex/z-index-abspos-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-abspos-009.xht"
}
```

## style[0]

```css

   * { background: transparent; margin: 0; }
   html { margin: 5em auto;
          height: 35px; width: 35px;
          border: solid 10px red;
          background: transparent; }
   body { background: transparent url(support/swatch-red.png) no-repeat center;
          height: 15px; width: 15px;
          border: 10px solid green; }
   div { background: red url(support/swatch-green.png) center no-repeat;
         height: 15px; width: 15px; padding: 9px;
         border: solid 10px green;
         position: absolute; margin-top: -19px; margin-left: -19px; z-index: -1; }
   p { position: absolute; top: 0; left: 0; margin: 1em; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
