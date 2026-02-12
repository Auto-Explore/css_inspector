# css/css-cascade/revert-rule-layer.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-rule-layer.tentative.html"
}
```

## style[0]

```css

  @layer {
    #test1 {
      color: green;
    }
  }
  @layer {
    #test1 {
      color: red;
      color: revert-rule;
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

## style[1]

```css

  @layer {
    #test2 {
      color: red;
    }
  }
  @layer {
    #test2 {
      color: green;
    }
    #test2 {
      color: red;
      color: revert-rule;
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
