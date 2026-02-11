# css/css-conditional/container-queries/transition-style-change-event-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/transition-style-change-event-002.html"
}
```

## style[0]

```css

  .container {
    container-type: inline-size;
  }
  #outer {
    width: 100px;
    color: green;
  }
  #target {
    transition: color 100s step-end;
  }

  @container (min-width: 200px) {
    #inner { color: red; }
  }
  @container (min-width: 200px) {
    #target {
      /* This rule exists just to have a container query dependency between
         target and #inner */
      background-color: orange;
    }
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
