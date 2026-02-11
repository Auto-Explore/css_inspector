# css/css-pseudo/active-selection-043.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-043.html"
}
```

## style[0]

```css

  div
    {
      font-size: 300%;
    }

  img
    {
      background-color: red;
      /* so that the padding belt is painted red */
      border: red solid 10px;
      padding: 10px;
    }

  /*
  Chromium 80+ highlights the descender space
  below the baseline on which the image "sits" while
  Firefox 72+ only highlights the image itself.
  */

  img::selection
    {
      background-color: red;
      color: red;
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
