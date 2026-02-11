# css/css-values/if-function-revert-rule.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-function-revert-rule.tentative.html"
}
```

## style[0]

```css

  #test1 {
    color: green;
  }
  #test1 {
    color: red;
    --x: 3;
    color: if(style(--x:1000):red; else:revert-rule);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “else”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

  #test2 {
    color: green;
  }
  #test2 {
    color: red;
    --x: 1000;
    color: if(style(--x:1000):revert-rule; else:red);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “else”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
