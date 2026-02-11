# css/css-flexbox/image-as-flexitem-size-005.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/image-as-flexitem-size-005.html"
}
```

## style[0]

```css

      .flexbox {
        display: flex;
        flex-direction: row;
        border: 1px solid black;
        margin: 0 2px 2px 0; /* (Just for spacing things out, visually) */
        width: 40px;
        height: 40px;

        justify-content: flex-start;
        align-items: flex-start;

        float: left; /* For testing in "rows" */
      }
      br { clear: both; }

      .flexbox > * {
        flex: 1;

        /* Disable "min-width:auto"/"min-height:auto" to focus purely on
           later channels of influence. */
        min-width: 0;
        min-height: 0;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
