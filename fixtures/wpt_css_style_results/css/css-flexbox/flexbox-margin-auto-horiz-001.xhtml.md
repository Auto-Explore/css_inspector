# css/css-flexbox/flexbox-margin-auto-horiz-001.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-margin-auto-horiz-001.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        width: 200px;
        height: 20px;
        background: lightgray;
        display: flex;
        margin-bottom: 2px;
      }
      div.a {
        width: 20px;
        background: green;
        margin-left: auto;
      }
      div.b {
        width: 20px;
        background: pink;
        margin-left: auto;
        margin-right: auto;
      }
      div.c {
        width: 20px;
        background: purple;
        margin-right: auto;
      }
    
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
