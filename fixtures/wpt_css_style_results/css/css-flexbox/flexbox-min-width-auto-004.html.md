# css/css-flexbox/flexbox-min-width-auto-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-width-auto-004.html"
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
