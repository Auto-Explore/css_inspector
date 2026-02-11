# css/cssom-view/offsetParent_element_test.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/offsetParent_element_test.html"
}
```

## style[0]

```css

#fixed {
    position: fixed;
}

#none-element {
    display:none;
}

#relative-element {
    position: relative;
}

#absolute-element {
    position: absolute;
}

#transform-element {
    transform: translateX(10px);
}

#perspective-element {
    perspective: 10px;
}

#transform-style-preserve-3d-element {
    transform-style: preserve-3d;
}

#filter-element {
    filter: opacity(25%);
}

#contain-paint-element {
    contain: paint;
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
