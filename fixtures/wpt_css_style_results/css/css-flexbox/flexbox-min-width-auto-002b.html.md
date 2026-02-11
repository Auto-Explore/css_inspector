# css/css-flexbox/flexbox-min-width-auto-002b.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-width-auto-002b.html"
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
        min-height: 30px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
