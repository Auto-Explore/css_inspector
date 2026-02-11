# css/css-conditional/container-queries/transition-scrollbars.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/transition-scrollbars.html"
}
```

## style[0]

```css

  #scrollable {
    overflow: auto;
    width: 100px;
    height: 100px;
  }
  #container {
    container-type: inline-size;
  }
  #target {
    background-color: black;
  }

  /* Matches with or without a scrollbar: */
  @container (max-width: 100px) {
    #target {
      background-color: blue;
    }
  }

  /* Matches only when there's a scrollbar: */
  @container (max-width: 99px) {
    #target {
      background-color: green;
      font-size: 10px;
      transition: 2s steps(2, start) background-color;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
