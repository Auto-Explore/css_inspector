# css/css-flexbox/flexbox-justify-content-horiz-003-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-justify-content-horiz-003-ref.xhtml"
}
```

## style[0]

```css

      body { margin-left: 0px; } /* We'll apply margins w/ style attribute */
      div.flexbox {
        margin-bottom: 2px;
        line-height: 0;
      }
      div.flexbox > * {
        display: inline-block;
      }
      div.a {
        height: 20px;
        width: 35px;
        background: lightgreen;
      }
      div.b {
        height: 20px;
        width: 40px;
        background: pink;
      }
      div.c {
        height: 20px;
        width: 45px;
        background: orange;
      }
    
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
