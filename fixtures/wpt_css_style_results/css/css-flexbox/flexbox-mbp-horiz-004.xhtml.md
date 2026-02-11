# css/css-flexbox/flexbox-mbp-horiz-004.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-mbp-horiz-004.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        width: 200px;
        display: flex;
        margin-bottom: 2px;
        border: 1px dotted black;
      }
      div.height50 { height: 50px; }

      .marginA  { margin: 10%  8%  6%  4%; }
      .marginB  { margin:  8% 10% 12% 14%; }
      .paddingA { padding: 8%  6%  4%  2%; }
      .paddingB { padding: 6%  8% 10% 12%; }

      div.child1 {
        flex: none;
        width: 10px;
        height: 10px;
        background: lightgreen;
      }
      div.child2 {
        flex: none;
        width: 10px;
        height: 10px;
        background: purple;
      }

      div.filler {
        /* Filler-div to fill up content-box and make padding easier to see. */
        height: 10px;
        width: 100%;
        background: lightgrey;
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
