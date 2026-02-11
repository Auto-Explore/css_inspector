# css/css-flexbox/flexbox-items-as-stacking-contexts-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-items-as-stacking-contexts-001-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        width: 90px;
        height: 10px;
        border: 2px solid gray;
        margin-bottom: 10px;
      }
      .a {
        width: 10px;
        height: 10px;
        background: lightblue;
        min-width: 0;
        float: left;
      }
      .b {
        width: 10px;
        height: 10px;
        background: pink;
        min-width: 0;
        margin-right: 10px;
        float: left;
      }
      .aKid {
         margin-left: 3px;
         margin-top:  3px;
         width: 10px;
         height: 10px;
         background: steelblue;
         border: 1px solid blue;
      }
      .bKid {
         margin-left: 3px;
         margin-top:  6px;
         width: 10px;
         height: 10px;
         background: violet;
         border: 1px solid purple;
      }

      .z0, .z1, .zn1 { position: relative; }
      .z0  { z-index:  0; }
      .z1  { z-index:  1; }
      .zn1 { z-index: -1; }

    
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
