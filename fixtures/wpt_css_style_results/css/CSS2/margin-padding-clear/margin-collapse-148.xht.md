# css/CSS2/margin-padding-clear/margin-collapse-148.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-148.xht"
}
```

## style[0]

```css

   td { width: 3em; padding: 0; }

   .test { background: yellow; }
   .a { margin: 0; height: 1em; background: orange; }
   .b { margin: 0; background: aqua; height: 2em; }
   .c { margin: 0; background: red; }
   .d { margin: 0; background: red; }
   .e { margin: 0; background: lime; height: 1em; float: left; width: 100%; }
   .f { margin: 0 0 1em 0; background: red; }

   .control { background: red; }
   .orange { border-top: solid 1em orange; }
   .yellow { border-top: solid 1em yellow; }
   .lime { border-top: solid 1em lime; }
   .aqua { border-top: solid 1em aqua; }
  
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
