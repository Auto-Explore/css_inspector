# css/css-flexbox/flexbox-min-height-auto-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-height-auto-004.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        flex-direction: column;
        height: 30px;  /* Shrink flex items below min-height */
        margin-right: 2px; /* (Just for spacing things out, visually) */
        float: left;
      }

      .flexbox > * {
        /* Flex items have purple border: */
        border: 2px dotted purple;
      }

      .flexbox > * > * {
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
