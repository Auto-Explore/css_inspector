# css/css-contain/contain-paint-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-dynamic-001.html"
}
```

## style[0]

```css

  /* Selectors for contain */
  #none .wrapper {
      contain: none;
  }
  #paint .wrapper {
      contain: paint;
  }
  #none_to_paint .wrapper {
      contain: none;
  }
  #paint_to_none .wrapper {
      contain: paint;
  }

  /* Selectors for testing absolute/fixed positioned elements */
  #top_spacer {
      height: 100px;
      background: lightgray;
  }
  .absolute_pos {
      position: absolute;
      top: 42px;
  }
  .fixed_pos {
      position: fixed;
      top: 42px;
  }

  /* Selectors for testing IFC (floats) */
  .floatleft {
      float: left;
  }
  .clearleft {
      clear: left;
  }

  /* Selectors for testing IFC (margin collapsing) */
  .blockmargin {
      margin: 25px 0;
  }
  .wrapper.blockmargin {
      background: lightgray;
  }

  .rect {
      background: black;
      width: 50px;
      height: 100px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
