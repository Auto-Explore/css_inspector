# css/CSS2/margin-padding-clear/margin-collapse-114.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-114.xht"
}
```

## style[0]

```css

   table { border-spacing: 0; font-size: 50px; border: solid 3px; background: red; }
   td { padding: 0; vertical-align: top;}
   /* test */
   .test { background: orange; width: 1em; }
   .test .a { background: yellow; margin: 0 0 1em 0; height: 1em; }
   .test .b { background: red; margin: 2em 0 0 0; position: relative; }
   .test .c { background: lime; margin: 3em 0 0 0; height: 1em; }
   .test .d { background: aqua; margin: 2em 0 1em 0; height: 1em; width: 100%; }
   .test .pos { position: absolute; }
   .test .float { float: left; }
   /* control */
   .control div { width: 1em; }
   .control .a { border-top: 1em yellow solid; }
   .control .b { border-top: 3em orange solid; }
   .control .c { border-top: 1em lime solid; }
   .control .d { border-top: 1em aqua solid; }
   .control .e { border-top: 1em orange solid; }
  
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
