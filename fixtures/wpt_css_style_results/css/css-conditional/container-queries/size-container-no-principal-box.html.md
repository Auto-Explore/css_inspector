# css/css-conditional/container-queries/size-container-no-principal-box.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/size-container-no-principal-box.html"
}
```

## style[0]

```css

  #outer {
    container-type: inline-size;
  }
  #inner_none {
    display: none;
    container-type: inline-size;
  }
  #inner_contents {
    display: contents;
    container-type: inline-size;
  }
  @container (min-width: 0) {
    span { color: red; }
  }
  @container (min-width: 0) {
    #ref { color: green; }
  }
  @container not (max-width: 0) {
    span { background-color: red; }
  }
  @container not (max-width: 0) {
    #ref { background-color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
