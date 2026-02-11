# css/filter-effects/filtered-inline-is-container-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filtered-inline-is-container-ref.html"
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
  /* no-op filter */
  transform: translateX(0px);
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
