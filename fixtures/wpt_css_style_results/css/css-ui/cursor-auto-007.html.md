# css/css-ui/cursor-auto-007.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-auto-007.html"
}
```

## style[0]

```css

#test {
  border: solid blue;
  width: 200px;
  cursor: auto;
}
#test * {
  cursor: url("support/cursors/fail.png"), help !important; /* Override UA styles, regardless of specificity */
  cursor: auto !important; /* Override UA styles, regardless of specificity */
}
#ref {
  width: 100px;
  height: 100px;
  cursor: default;
  border: solid orange;
}
:root {
  cursor: help; /* give the root element a different cursor so that
  it is easy to tell if something changes when hovering the target.*/
}

/* Hide the text if we cannot make it unselectable.
 user-select is not part of css-ui level 3, so we should not depend on it
 but it is nice to test it if it is supported.
 Test for level 4 can take the conditional out. */
.unselectable { display: none; }
@supports (user-select: none) or (-webkit-user-select: none) {
  .unselectable {
    display: block;
    user-select: none;
    -webkit-user-select: none;
  }
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-user-select”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
