# css/css-flexbox/flexbox-paint-ordering-002.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-paint-ordering-002.xhtml"
}
```

## style[0]

```css

      body {
        line-height: 0;
      }

      .flexbox {
        display: inline-flex;
        width: 20px;
        height: 10px;
        border: 2px solid gray;
        margin-bottom: 10px;
        margin-right: 10px;
      }
      .a {
        width: 10px;
        height: 10px;
        background: lightblue;
        min-width: 0;
      }
      .b {
        width: 10px;
        height: 10px;
        background: pink;
        min-width: 0;
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

      .on1 { order: -1; }
      .o0  { order:  0; }
      .o1  { order:  1; }
      .o2  { order:  2; }

      .zn2 { z-index: -2; }
      .zn1 { z-index: -1; }
      .z0  { z-index:  0; }
      .z1  { z-index:  1; }

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
