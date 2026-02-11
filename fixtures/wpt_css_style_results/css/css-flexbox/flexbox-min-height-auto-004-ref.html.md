# css/css-flexbox/flexbox-min-height-auto-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-height-auto-004-ref.html"
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

      .xvisible { overflow-x: visible; }
      .xhidden  { overflow-x: hidden;  }
      .xscroll  { overflow-x: scroll;  }
      .xauto    { overflow-x: auto;    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
