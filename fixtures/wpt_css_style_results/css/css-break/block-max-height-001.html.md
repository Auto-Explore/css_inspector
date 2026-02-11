# css/css-break/block-max-height-001.html

```json
{
  "format_version": 3,
  "file": "css/css-break/block-max-height-001.html"
}
```

## style[0]

```css

html,body {
    color:black; background-color:white; font:24px/1 monospace; padding:0; margin:0;
    width: 300px;
}
.columns {
  columns: 3;
  background: grey;
  margin-bottom: 1em;
}
.columns > div { max-height:160px; background: yellow; }
.columns > div > div { height:200px; width:50px; border:solid; }
  
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
