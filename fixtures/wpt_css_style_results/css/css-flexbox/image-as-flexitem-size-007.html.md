# css/css-flexbox/image-as-flexitem-size-007.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/image-as-flexitem-size-007.html"
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
      img {
        padding: 1px 2px 3px 4px;
        box-sizing: border-box;
        background: pink;
      }

      br { clear: both; }

      .flexbox > * {
        /* Disable "min-width:auto"/"min-height:auto" to focus purely on
           later channels of influence. */
        min-width: 0;
        min-height: 0;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
