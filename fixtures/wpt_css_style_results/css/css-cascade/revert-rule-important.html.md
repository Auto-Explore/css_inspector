# css/css-cascade/revert-rule-important.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-rule-important.html"
}
```

## style[0]

```css

  #test1 {
    color: revert-rule !important;
  }
  #test1 {
    color: green;
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
      color: revert-rule !important;
    }
  }
  #test2 {
    color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

  #test3 {
    color: green;
  }
  #test3 {
    color: revert-rule !important;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
