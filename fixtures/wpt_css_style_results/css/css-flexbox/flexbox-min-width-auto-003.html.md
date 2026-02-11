# css/css-flexbox/flexbox-min-width-auto-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-width-auto-003.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        width: 30px;  /* Shrink flex items below min-width */
        margin-bottom: 2px; /* (Just for spacing things out, visually) */
      }

      .flexbox > * {
        /* Flex items have purple border: */
        border: 2px dotted purple;
      }

      .flexbox > * > * {
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
