# css/css-flexbox/flexbox-min-height-auto-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-height-auto-003-ref.html"
}
```

## style[0]

```css

      .item {
        /* Flex items have purple border: */
        border: 2px dotted purple;
        margin-right: 2px; /* (Just for spacing things out, visually) */
        float: left;
      }

      .small { height: 26px; }
      .big   { height: 80px; }

      .item > * {
        /* Flex items' contents are gray & fixed-size: */
        background: gray;
        width: 40px;
        height: 80px;
      }

      .yvisible { overflow-y: visible; }
      .yhidden  { overflow-y: hidden;  }
      .yscroll  { overflow-y: scroll;  }
      .yauto    { overflow-y: auto;    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
