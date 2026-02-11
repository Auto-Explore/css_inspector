# css/css-flexbox/flexbox-mbp-horiz-003-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-mbp-horiz-003-ref.xhtml"
}
```

## style[0]

```css

      div { height: 20px; border: 0; }
      div.flexbox {
        width: 200px;
        margin-bottom: 2px;
      }

      <!-- customizations for flexbox border/padding -->
      .borderA {
        border-style: dashed;
        border-color: purple;
        border-top-width: 6px;
        border-right-width: 4px;
        border-bottom-width: 2px;
        border-left-width: 8px;
      }

      .borderB {
        border-style: dashed;
        border-color: purple;
        border-top-width: 4px;
        border-right-width: 5px;
        border-bottom-width: 6px;
        border-left-width: 7px;
      }

      .paddingA {
        padding: 4px 3px 2px 1px;
      }

      .paddingB {
        padding: 8px 11px 14px 17px;
      }

      div.child1 {
        display: inline-block;
        width: 74px;
        background: lightgreen;
        border-style: dotted;
        border-left-width: 2px;
        border-right-width: 4px;
      }
      div.child2 {
        display: inline-block;
        width: 110px;
        background: yellow;
        border-style: dashed;
        border-left-width: 7px;
        border-right-width: 3px;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
