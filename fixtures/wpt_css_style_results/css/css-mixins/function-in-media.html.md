# css/css-mixins/function-in-media.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-in-media.html"
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

    @media (width >= 50px) {
      @function --f() { result: 50; }
    }
    @media (width >= 100px) {
      @function --f() { result: 100; }
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
