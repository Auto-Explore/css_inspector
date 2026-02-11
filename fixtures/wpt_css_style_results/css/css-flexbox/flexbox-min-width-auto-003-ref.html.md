# css/css-flexbox/flexbox-min-width-auto-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-width-auto-003-ref.html"
}
```

## style[0]

```css

      .item {
        /* Flex items have purple border: */
        border: 2px dotted purple;
        margin-bottom: 2px; /* (Just for spacing things out, visually) */
      }

      .small { width: 26px; }
      .big   { width: 80px; }

      .item > * {
        /* Flex items' contents are gray & fixed-size: */
        background: gray;
        height: 40px;
        width: 80px;
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
