# css/css-flexbox/flexbox-min-height-auto-002a.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-height-auto-002a.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        flex-direction: column;
        height: 1px;  /* No available space; shrink flex items to min-height */
        margin-right: 2px; /* (Just for spacing things out, visually) */
        float: left;
      }

      .flexbox > * {
        /* Flex items have purple border: */
        border: 2px dotted purple;
        /* Flex items have sizing constraint in cross axis: */
        width: 30px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
