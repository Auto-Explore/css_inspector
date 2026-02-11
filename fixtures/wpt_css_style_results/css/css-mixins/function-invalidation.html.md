# css/css-mixins/function-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-invalidation.html"
}
```

## style[0]

```css

  @function --f() {
    result: 10px;
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

## style[1]

```css

  #outer {
    width: 42px;
  }
  #target {
    width: --f();
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
