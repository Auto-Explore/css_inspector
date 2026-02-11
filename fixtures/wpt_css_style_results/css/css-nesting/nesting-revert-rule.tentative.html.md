# css/css-nesting/nesting-revert-rule.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/nesting-revert-rule.tentative.html"
}
```

## style[0]

```css

  :root {
    #test1 {
      color: green;
    }
    #test1 {
      color: red;
      color: revert-rule;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

  :root {
    #test2 {
      /* CSSNestedDeclarationsRule { */
        color: green;
      /* } */
      & {
        color: red;
        color: revert-rule;
      }
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

  :root {
    #test3 {
      /* CSSNestedDeclarationsRule { */
        color: green;
      /* } */
      .something {}
      /* CSSNestedDeclarationsRule { */
        color: red;
        color: revert-rule;
      /* } */
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

  @scope (#test4) {
    /* CSSNestedDeclarationsRule { */
      color: green;
    /* } */
  }

  #test4 {
    color: red;
    color: revert-rule;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
