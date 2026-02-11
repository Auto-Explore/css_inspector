# css/CSS2/floats-clear/margin-collapse-164.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-164.xht"
}
```

## style[0]

```css

   .outer { margin: 1em; background: red; height: 4.5em; }
   .border { border: solid; width: 10em; }
   .box { margin: 0; background: yellow; }
   .float { margin: 0; width: 5em; height: 1.5em; background: orange; float: right; }
   .clear { margin-top: 3em; height: 1.5em; background: aqua; clear: both; }
   .control { border: solid; width: 10em; background: yellow; margin: 1em; }
   .control .a { margin: 0 0 0 auto; width: 5em; height: 1.5em; background: orange; }
   .control .b { margin-top: 1.5em; height: 1.5em; background: aqua; }
  
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
