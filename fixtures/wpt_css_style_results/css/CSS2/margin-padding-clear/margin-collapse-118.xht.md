# css/CSS2/margin-padding-clear/margin-collapse-118.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-118.xht"
}
```

## style[0]

```css

   table { border-spacing: 0; font-size: 50px; border: solid; background: red; }
   td { padding: 0; width: 1em; }
   /* test */
   .test .outer { background: orange; }
   .test .a { background: red; margin: 0; height: auto; }
   .test .aa { background: yellow; margin: 0 0 1em 0; height: 1em; }
   .test .b { background: lime; margin: 0; height: 1em; }
   /* control */
   .control .a { border-top: 1em yellow solid; }
   .control .b { border-top: 1em orange solid; }
   .control .c { border-top: 1em lime solid; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
