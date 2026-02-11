# css/css-flexbox/flexbox-min-width-auto-002c.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-width-auto-002c.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        width: 0px;  /* No available space; shrink flex items to min-width */
        margin-bottom: 2px; /* (Just for spacing things out, visually) */
      }

      .flexbox > * {
        /* Flex items have purple border: */
        border: 2px dotted purple;
        /* Flex items have sizing constraint in cross axis: */
        max-height: 30px;
        height: 60px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
