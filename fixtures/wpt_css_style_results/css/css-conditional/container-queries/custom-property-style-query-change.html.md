# css/css-conditional/container-queries/custom-property-style-query-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/custom-property-style-query-change.html"
}
```

## style[0]

```css

  #container { container-name: my-container; }
  #child, #grandchild { color: red; }
  @container style(--target: child) {
    #child { color: green; }
  }
  @container my-container style(--target: grandchild) {
    #grandchild { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  @property --length {
    syntax: "<length>";
    inherits: false;
    initial-value: 0px;
  }

  #reg_container {
    container-name: my-reg-container;
    font-size: 50px;
  }
  #reg_child, #reg_grandchild { color: red; }
  @container style(--length: 100px) {
    #reg_child { color: green; }
  }
  @container my-reg-container style(--length: 200px) {
    #reg_grandchild { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
