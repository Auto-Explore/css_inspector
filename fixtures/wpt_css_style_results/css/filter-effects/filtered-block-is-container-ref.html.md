# css/filter-effects/filtered-block-is-container-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filtered-block-is-container-ref.html"
}
```

## style[0]

```css

#scroller {
  width: 500px;
  height: 500px;
  overflow: scroll
}
.spacer {
  width: 1000px;
  height: 500px;
  background-color: lightgreen;
}
#transform {
  /* no-op transform */
  transform: translateX(0px);
  width: 100px;
  height: 50px;
  background-color: lightblue;
}
#fixed, #absolute {
  width: 100px;
  height: 100px;
}
#fixed {
  position: fixed;
  top: 100px;
  left: 150px;
  background-color: green;
}
#absolute {
  position: absolute;
  top: 200px;
  left: 300px;
  background-color: blue;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
