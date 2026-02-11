# css/css-flexbox/flexbox-mbp-horiz-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-mbp-horiz-001-ref.xhtml"
}
```

## style[0]

```css

      div { height: 100px; border: 0; }
      div.flexbox {
        width: 200px;
      }
      div.a {
        display: inline-block;
        background: lightgreen;
        border-style: dotted;
        border-left-width: 2px;
        border-right-width: 4px;
      }
      div.b {
        display: inline-block;
        background: yellow;
        border-style: dashed;
        border-left-width: 7px;
        border-right-width: 3px;
      }
      div.c {
        display: inline-block;
        background: orange;
      }
      div.flexNone {
        display: inline-block;
        background: pink;
      }
      div.flexBasis {
        display: inline-block;
        background: gray;
      }
      div.spacer {
        height: 15px;
        background: purple;
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
