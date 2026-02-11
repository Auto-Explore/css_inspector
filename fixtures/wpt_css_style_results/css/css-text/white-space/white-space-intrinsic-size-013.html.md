# css/css-text/white-space/white-space-intrinsic-size-013.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/white-space-intrinsic-size-013.html"
}
```

## style[0]

```css

  div
    {
      color: transparent;
      /*
      so that background-color can
      shine through the "A", the "GH",
      the "MNO" and the "WXYZ" glyphs
      */
      font-family: Ahem;
      font-size: 25px;
      line-height: 1;
      width: 0;
      /*
      'width: 0' will trigger
      min-content size
      for div#min-sized-parent
      */
    }

  div#test-overlapped-red
    {
      background-color: red;
      float: left;
      white-space: pre-wrap;
      width: auto;
    }

  div#reference-overlapping-green
    {
      background-color: green;
      position: absolute;
      width: auto;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
