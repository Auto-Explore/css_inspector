# css/css-flexbox/flexbox-flex-basis-content-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-flex-basis-content-003-ref.html"
}
```

## style[0]

```css

  .container {
    clear: both; /* In this reference case, we use floats instead of
                    flex items (see below), so the container just
                    needs to reset the float state for each example. */
  }

  .item {
    border: 2px solid teal;
    float: left; /* Use floated elements as a reference for (hopefully)
                    max-content sized flex items in testcase. */
  }
  ib {
    display: inline-block;
    background: blue;
    border: 1px solid gray;
    width: 15px;
    height: 10px;
  }
  float {
    float: left;
    background: fuchsia;
    border: 1px solid gray;
    width: 15px;
    height: 10px;
  }
  canvas {
    background: brown;
    border: 1px solid gray;
  }
  .innerFlex {
    display: flex;
  }
  innerItem {
    background: salmon;
    border: 1px solid gray;
    height: 10px;
    width: 15px;
    flex: none;
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
