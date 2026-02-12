# css/css-mixins/function-media-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-media-dynamic.html"
}
```

## style[0]

```css

  iframe {
    width: 50px;
    height: 50px;
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

    @function --f() {
      result: A;
      @media (width = 100px) {
        result: B;
      }
      @media ((width >= 110px) and (width <= 140px)) {
        result: C;
      }
      @media (width = 150px) {
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
