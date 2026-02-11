# css/CSS2/zindex/z-index-abspos-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-abspos-008.xht"
}
```

## style[0]

```css

   div { position: absolute; width: 4em; height: 4em; }
   .a { top: 1em; left: 1em; width: 12em; height: 12em; background: gray; }
   .b { top: 1em; left: 4em; background: blue; }
   .d { top: 4em; left: 7em; background: orange; }
   .e { top: 3em; left: -3em; background: yellow; z-index: 1; }
   .c { top: 3em; left: -3em; background: lime; z-index: 2; }
   .text { top: 14em; width: 30em; }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
