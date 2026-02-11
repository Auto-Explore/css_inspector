# css/css-flexbox/flexbox-sizing-horiz-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-sizing-horiz-001-ref.xhtml"
}
```

## style[0]

```css

      div { height: 10px; }
      div.flexbox {
        border: 1px dashed blue;
        font-size: 10px;
        margin-bottom: 2px;
      }
      div.a, div.b, div.c { float: left }
      div.a {
        width: 20px;
        background: lightgreen;
      }
      div.b {
        width: 40px;
        background: purple;
      }
      div.c {
        width: 40px;
        background: orange;
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
