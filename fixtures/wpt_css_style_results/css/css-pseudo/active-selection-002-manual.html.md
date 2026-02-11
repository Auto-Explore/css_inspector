# css/css-pseudo/active-selection-002-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-002-manual.html"
}
```

## style[0]

```css

  div
    {
      background-color: red;
      color: aqua;
      display: inline;
      font-size: 300%;
    }

  div::selection
    {
      background-color: green;
    }

  /*
  Chromium 77.0.3855.0 creates a background-color #334C00 or rgb(51, 76, 0)
  and not a green (#008000 or rgb(0, 128, 0)) background color.

  Issue 1018450: background-color of ::selection should be painted as specified
  https://bugs.chromium.org/p/chromium/issues/detail?id=1018450
  */
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
