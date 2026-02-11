# css/css-mixins/function-container-self.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-container-self.html"
}
```

## style[0]

```css

  #container, #target {
    container-type: size;
  }
  #container {
    width: 100px;
    height: 100px;
  }
  #target {
    width: 50px;
    height: 50px;
  }
  @function --f() {
    result: A;
    @container (width = 100px) {
      result: B;
    }
    @container (width = 50px) {
      result: C;
    }
  }
  #target {
    --actual: --f();
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
