# css/css-flexbox/flexbox-min-height-auto-002b.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-min-height-auto-002b.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        flex-direction: column;
        max-width: 34px; /* Constrain the flex container's cross size. */
        height: 1px;  /* No available space; shrink flex items to min-height */
        margin-right: 2px; /* (Just for spacing things out, visually) */
        float: left;
      }

      .flexbox > * {
        /* Flex items have purple border: */
        border: 2px dotted purple;
        /* Flex items have sizing constraint in cross axis: */
        min-width: 30px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
