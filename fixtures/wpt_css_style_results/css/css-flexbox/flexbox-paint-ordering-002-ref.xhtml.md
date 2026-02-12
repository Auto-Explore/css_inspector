# css/css-flexbox/flexbox-paint-ordering-002-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-paint-ordering-002-ref.xhtml"
}
```

## style[0]

```css

      body {
        line-height: 0;
      }

      .flexbox {
        display: inline-block;
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
        float: left; /* to stack horizontally, like a flex item */
      }
      .b {
        width: 10px;
        height: 10px;
        background: pink;
        float: left; /* to stack horizontally, like a flex item */
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

      /* Need to set 'position' for z-index to take effect. */
      .zn2 { z-index: -2; position: relative; }
      .zn1 { z-index: -1; position: relative; }
      .z0  { z-index:  0; position: relative; }
      .z1  { z-index:  1; position: relative; }

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
