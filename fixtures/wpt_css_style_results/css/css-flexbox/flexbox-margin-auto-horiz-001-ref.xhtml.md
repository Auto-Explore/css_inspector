# css/css-flexbox/flexbox-margin-auto-horiz-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-margin-auto-horiz-001-ref.xhtml"
}
```

## style[0]

```css

      div { height: 20px; }
      div.flexbox {
        width: 200px;
        background: lightgray;
        margin-bottom: 2px;
      }
      div.a {
        width: 20px;
        background: green;
        display: inline-block;
      }
      div.b {
        width: 20px;
        background: pink;
        display: inline-block;
      }
      div.c {
        width: 20px;
        background: purple;
        display: inline-block;
      }

      <!-- These classes allow us to conveniently/concisely specify margin
           values below, for exact positioning of the items on each reference
           line. ("l" = "margin-_l_eft", and the number = number of pixels) -->
      div.l180 { margin-left: 180px }
      div.l90  { margin-left:  90px }
      div.l80  { margin-left:  80px }
      div.l70  { margin-left:  70px }
      div.l53  { margin-left: calc(160px / 3) } <!-- == 53.33333px -->
      div.l35  { margin-left:  35px }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
