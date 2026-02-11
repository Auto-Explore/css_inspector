# css/css-pseudo/active-selection-045.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-045.html"
}
```

## style[0]

```css

  /*
  Chromium 80+ highlights the
  top of line box minus
  content box and the bottom
  of line box minus content
  box.
  Firefox 72+ consistently highlights
  only the image content box itself.
  */

  div
    {
      font-size: 300px;
    }

  img::selection
    {
      background-color: red;
    }

  span#masking
    {
      background-color: white;
      display: inline-block;
      height: 100px;
      position: relative;
      right: 100px;
      width: 100px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
