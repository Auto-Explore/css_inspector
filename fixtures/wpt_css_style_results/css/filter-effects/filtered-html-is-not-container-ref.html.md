# css/filter-effects/filtered-html-is-not-container-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filtered-html-is-not-container-ref.html"
}
```

## style[0]

```css

#spacer {
  width: 2000px;
  height: 2000px;
  background-color: lightgreen;
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
