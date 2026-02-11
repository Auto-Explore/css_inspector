# css/CSS2/margin-padding-clear/margin-collapse-106.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-106.xht"
}
```

## style[0]

```css

   table { border-spacing: 0; font: 50px/1 Ahem; border: solid 3px; background: red; }
   td { padding: 0; }
   /* test */
   .test { background: orange; }
   .test div { margin: 1em 0; background: red; }
   .test .a { color: yellow; }
   .test .b { color: lime; }
   /* control */
   .control div { width: 1em; }
   .control .c { border-top: 1em orange solid; }
   .control .a { border-top: 1em yellow solid; }
   .control .b { border-top: 1em lime solid; }
   /* control control */
   .red { background: red; height: 1em; width: 10em; border: solid; }
   .orange { border-top: 1em orange solid; }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
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
