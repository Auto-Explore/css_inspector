# css/filter-effects/drop-shadow-currentcolor-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/drop-shadow-currentcolor-inheritance.html"
}
```

## style[0]

```css

#filter {
    color: red;
    filter: drop-shadow(100px 0px 0px currentcolor);
}
#target {
  background-color: black;
  color: green;
  width: 100px;
  height: 100px;
  filter: inherit;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
