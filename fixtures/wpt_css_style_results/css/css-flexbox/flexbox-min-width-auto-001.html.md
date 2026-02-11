# css/css-flexbox/flexbox-min-width-auto-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-width-auto-001.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        width: 1px;  /* No available space; shrink flex items to min-width */
        margin-bottom: 2px; /* (Just for spacing things out, visually) */
      }

      .flexbox > * {
        /* Flex items have purple border: */
        border: 2px dotted purple;
      }

      .flexbox > * > * {
        /* Flex items' contents are gray & fixed-size: */
        background: gray;
        height: 10px;
        width: 80px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
