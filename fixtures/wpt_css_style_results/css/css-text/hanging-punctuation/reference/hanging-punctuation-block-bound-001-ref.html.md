# css/css-text/hanging-punctuation/reference/hanging-punctuation-block-bound-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hanging-punctuation/reference/hanging-punctuation-block-bound-001-ref.html"
}
```

## style[0]

```css

  div
    {
      font-family: monospace;
      font-size: 60px;
      line-height: 1.5em; /* computes to 90px */
    }

  /*
  We draw the box (perfectly fitted to the content
  in the block axis, because it has the same content)
  but not the text.
  */

  div#box-drawn-under
    {
      background-color: lime;
      border: black solid 3px;
      color: transparent;
      position: absolute;
      width: 240px;
      z-index: -1;
    }

  /*
  We draw the text, but into a wider box so
  the period will fit, and overlap it with
  the previously-drawn box.
  */

  div#text-drawn-over
    {
      border: transparent solid 3px;
      width: 300px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
