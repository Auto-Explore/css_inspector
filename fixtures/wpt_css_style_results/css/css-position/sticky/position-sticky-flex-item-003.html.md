# css/css-position/sticky/position-sticky-flex-item-003.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-flex-item-003.html"
}
```

## style[0]

```css

#scroller {
  overflow: hidden;
  inline-size: 100px;
  block-size: 100px;
  background-color: green;
}

#flex {
  display: flex;
  flex-direction: column;

  /* Use a small block-size so that the flex items overflow the flex container.
     It's necessary to trigger the bug. */
  block-size: 10px;
}

#non-sticky {
  flex: 0 0 80px;
  background-color: red;
}

#sticky {
  position: sticky;
  flex: 0 0 100px;
  bottom: 0;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
