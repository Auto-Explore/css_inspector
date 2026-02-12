# css/css-mixins/function-container-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-container-dynamic.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 50px;
    height: 50px;
  }
  @function --f() {
    result: A;
    @container (width = 100px) {
      result: B;
    }
    @container ((width >= 110px) and (width <= 140px)) {
      result: C;
    }
    @container (width = 150px) {
      result: D;
    }
  }
  #target {
    --actual: --f();
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
