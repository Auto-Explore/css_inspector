# css/css-mixins/function-container-style.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-container-style.html"
}
```

## style[0]

```css

  #container, #target {
    container-type: size;
  }
  #size-container {
    container-type: size;
    width: 100px;
    height: 100px;
    --x: size;
  }
  #parent {
    --x: parent;
  }
  @function --f() {
    result: A;
    @container (style(--x: before)) {
      result: B;
    }
    @container (style(--x: target)) {
      result: C;
    }
    @container (style(--x: parent)) {
      result: D;
    }
    @container (style(--x: size)) {
      result: E;
    }
  }
  #target {
    --x: target;
    --actual: --f();
  }
  #target::before {
    content: "test";
    --x: before;
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
