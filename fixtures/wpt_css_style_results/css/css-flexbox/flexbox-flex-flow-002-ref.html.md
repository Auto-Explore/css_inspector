# css/css-flexbox/flexbox-flex-flow-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-flex-flow-002-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      height: 60px;
      width: 60px;
      font: 10px sans-serif;
      background: yellow;
      float: left;
      border: 1px solid black;
    }
    .flexContainer > * {
      border: 1px dotted gray;
      width: 28px;
      height: 28px;
      float: left;
    }

    /* The single-line flex containers' flex items are shrunk in main axis: */
    .singleLineHoriz > * {
      width: 18px;
    }
    .singleLineVert  > * {
      height: 18px;
      float: none;
    }
    .hidden {
      /* We use this to hide the "4" box in each of the multi-line chunks.
         The testcase has 3 flex items in each flex container, but it's easier
         to write this reference case w/ a hidden 4th box as a space-filler. */
      visibility: hidden;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
